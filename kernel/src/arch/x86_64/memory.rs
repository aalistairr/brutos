use core::ops::Range;
use core::ptr::NonNull;

use brutos_alloc::OutOfMemory;
use brutos_memory::phys_alloc::bootstrap;
use brutos_memory::vm::mmu;
use brutos_memory::{PhysAddr, VirtAddr};

use crate::memory::{CutRange, FailedToBootstrap};
use crate::Cx;

pub const PHYS_IDENT_OFFSET: usize = 0xffff880000000000;
pub const PHYS_IDENT_SIZE: usize = 0x0000400000000000;
pub const PHYS_IDENT_END: usize = PHYS_IDENT_OFFSET + PHYS_IDENT_SIZE;

pub const KERNEL_ADDR_SPACE_RANGE: Range<VirtAddr> =
    VirtAddr(0xffff800000000000)..VirtAddr(0xffffffffffffffff);
pub const USER_ADDR_SPACE_RANGE: Range<VirtAddr> =
    VirtAddr(0x0000000000000000)..VirtAddr(0x0000800000000000);

pub fn map_phys_ident(addr: PhysAddr, size: usize) -> Result<NonNull<u8>, ()> {
    if addr.0.checked_add(size).ok_or(())? > PHYS_IDENT_SIZE {
        return Err(());
    }
    return unsafe {
        Ok(NonNull::new_unchecked(
            (addr.0 + PHYS_IDENT_OFFSET) as *mut u8,
        ))
    };
}

pub fn phys_ident_addr(ptr: NonNull<u8>) -> PhysAddr {
    PhysAddr(ptr.as_ptr() as usize - PHYS_IDENT_OFFSET)
}

unsafe impl bootstrap::Context for Cx {
    type Err = FailedToBootstrap;

    fn map(
        &mut self,
        addr: PhysAddr,
        size: usize,
        align: usize,
    ) -> Result<*mut u8, FailedToBootstrap> {
        debug_assert!(addr.is_aligned(align));
        map_phys_ident(addr, size)
            .map(NonNull::as_ptr)
            .map_err(|()| FailedToBootstrap)
    }
}

#[allow(improper_ctypes)]
extern "C" {
    static _image_start: ();
    static _image_end: ();
}

fn image_range() -> Range<PhysAddr> {
    unsafe {
        PhysAddr(&_image_start as *const _ as usize)..PhysAddr(&_image_end as *const _ as usize)
    }
}

pub fn remove_reserved_memory(
    multiboot_info_range: Range<PhysAddr>,
    mmap: impl Clone + Iterator<Item = Range<PhysAddr>>,
) -> impl Clone + Iterator<Item = Range<PhysAddr>> {
    let mmap = CutRange::new(mmap, PhysAddr(0x0)..PhysAddr(0x500));
    let mmap = CutRange::new(mmap, PhysAddr(0x7c00)..PhysAddr(0x7e00));
    let mmap = CutRange::new(mmap, PhysAddr(0x9fc00)..PhysAddr(0x100000));
    let mmap = CutRange::new(mmap, PhysAddr(0xf00000)..PhysAddr(0x1000000));
    let mmap = CutRange::new(mmap, PhysAddr(0xc0000000)..PhysAddr(0x100000000));
    let mmap = CutRange::new(mmap, multiboot_info_range);
    let mmap = CutRange::new(mmap, image_range());
    let mmap = mmap.filter(|r| r.start.0 < PHYS_IDENT_SIZE).map(|r| {
        if r.end.0 <= PHYS_IDENT_SIZE {
            r
        } else {
            r.start..PhysAddr(PHYS_IDENT_SIZE)
        }
    });
    mmap
}

pub fn create_kernel_mmu_tables() -> Result<mmu::Tables, OutOfMemory> {
    let mut tables = mmu::Tables::new();
    for addr in (KERNEL_ADDR_SPACE_RANGE.start.0..KERNEL_ADDR_SPACE_RANGE.end.0)
        .step_by(mmu::arch::Level::Pml4.entry_size())
    {
        unsafe {
            match mmu::arch::create_permanent_table(
                &mut Cx::default(),
                &mut tables.root,
                VirtAddr(addr),
                mmu::arch::Level::Pml4,
            ) {
                Ok(()) => (),
                Err(mmu::MapError::OutOfMemory) => return Err(OutOfMemory),
                Err(_) => unreachable!(),
            }
        }
    }
    Ok(tables)
}

pub unsafe fn destroy_kernel_mmu_tables(mut tables: mmu::Tables) {
    for addr in (KERNEL_ADDR_SPACE_RANGE.start.0..KERNEL_ADDR_SPACE_RANGE.end.0)
        .step_by(mmu::arch::Level::Pml4.entry_size())
    {
        mmu::arch::make_nonpermanent(
            &mut Cx::default(),
            &mut tables.root,
            VirtAddr(addr),
            mmu::arch::Level::Pml4,
        )
        .expect("invalid page tables");
    }
}
