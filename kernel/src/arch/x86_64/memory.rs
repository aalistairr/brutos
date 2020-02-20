use core::mem::MaybeUninit;
use core::ops::Range;
use core::pin::Pin;
use core::ptr::NonNull;

use brutos_alloc::{Arc, OutOfMemory};
use brutos_memory_phys_alloc::bootstrap;
use brutos_memory_traits::AllocPhysPage;
use brutos_memory_units::{Order, PhysAddr, VirtAddr};
use brutos_memory_vm::{self as vm, mmu};

use crate::memory::addr_space::AddressSpace;
use crate::memory::alloc::{CutRange, FailedToBootstrap};
use crate::Cx;

pub const PHYS_IDENT_OFFSET: usize = 0xffff880000000000;
pub const PHYS_IDENT_SIZE: usize = 0x0000008000000000;
pub const PHYS_IDENT_END: usize = PHYS_IDENT_OFFSET + PHYS_IDENT_SIZE;

pub const VMA_OFFSET: usize = 0xffffffff80000000;
pub const VMA_SIZE: usize = 0x40000000;
pub const VMA_END: usize = VMA_OFFSET + VMA_SIZE;

pub const KERNEL_ADDR_SPACE_RANGE: Range<VirtAddr> =
    VirtAddr(0xffff800000000000)..VirtAddr(0xffffffffffffffff);
pub const USER_ADDR_SPACE_RANGE: Range<VirtAddr> =
    VirtAddr(0x0000000000000000)..VirtAddr(0x0000800000000000);

pub const unsafe fn map_phys_ident_unchecked(addr: PhysAddr) -> NonNull<u8> {
    NonNull::new_unchecked((addr.0 + PHYS_IDENT_OFFSET) as *mut u8)
}

pub fn map_phys_ident(addr: PhysAddr, size: usize) -> Result<NonNull<u8>, ()> {
    if addr.0.checked_add(size).ok_or(())? > PHYS_IDENT_SIZE {
        return Err(());
    }
    return unsafe { Ok(map_phys_ident_unchecked(addr)) };
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
    static PHYS_IDENT_PML4: ();
}

fn image_range() -> Range<PhysAddr> {
    unsafe {
        PhysAddr(&_image_start as *const _ as usize)..PhysAddr(&_image_end as *const _ as usize)
    }
}

fn pml4_phys() -> PhysAddr {
    unsafe { PhysAddr(&PHYS_IDENT_PML4 as *const _ as usize) }
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

static mut KERNEL_PML4: [mmu::arch::Entry; 256] = [mmu::arch::Entry::new(); 256];

pub unsafe fn create_kernel_mmu_tables() -> Result<mmu::Tables, OutOfMemory> {
    let mut tables = mmu::Tables::with_root(
        mmu::arch::Entry::new()
            .with_address(pml4_phys())
            .with_population(mmu::arch::Entry::PERMANENT)
            .with_present(true),
    );
    for pml4e_i in 0..256 {
        KERNEL_PML4[pml4e_i] = mmu::arch::create_permanent_table(
            &mut Cx,
            &mut tables.root,
            KERNEL_ADDR_SPACE_RANGE.start + pml4e_i * mmu::arch::Level::Pml4.entry_size(),
            mmu::arch::Level::Pml4,
            Default::default(),
        )
        .expect("failed to create kernel mmu tables");
    }
    Ok(tables)
}

pub unsafe fn destroy_kernel_mmu_tables(_tables: mmu::Tables) {}

pub unsafe fn create_user_mmu_tables() -> Result<mmu::Tables, mmu::MapError> {
    let mut tables = mmu::Tables::new();
    for pml4e_i in 0..256 {
        mmu::arch::x86_64::set_entry(
            &mut Cx,
            &mut tables.root,
            KERNEL_ADDR_SPACE_RANGE.start + pml4e_i * mmu::arch::Level::Pml4.entry_size(),
            mmu::arch::Level::Pml4,
            KERNEL_PML4[pml4e_i],
            Default::default(),
        )?;
    }
    Ok(tables)
}

pub unsafe fn destroy_user_mmu_tables(mut tables: mmu::Tables) {
    for pml4e_i in 0..256 {
        let _ = mmu::arch::x86_64::clear_entry(
            &mut Cx,
            &mut tables.root,
            KERNEL_ADDR_SPACE_RANGE.start + pml4e_i * mmu::arch::Level::Pml4.entry_size(),
            mmu::arch::Level::Pml4,
        );
    }
}

static mut SHARED_ORDER9_EMPTY_PAGE: MaybeUninit<(PhysAddr, &<Cx as AllocPhysPage>::PageData)> =
    MaybeUninit::uninit();

pub fn initialize() {
    let order9 =
        <Cx as AllocPhysPage>::alloc(Order(9)).expect("failed to allocate shared empty page");
    order9.1.as_ref().inc();
    unsafe {
        SHARED_ORDER9_EMPTY_PAGE.write(order9);
    }
}

impl vm::Context for Cx {
    fn shared_empty_page(&mut self, order: Order) -> Option<(PhysAddr, &Self::PageData)> {
        if order <= Order(9) {
            let (addr, data) = unsafe { &*SHARED_ORDER9_EMPTY_PAGE.as_ptr() };
            Some((*addr, data))
        } else {
            None
        }
    }
}

pub fn create_kernel_mappings(addr_space: &Pin<Arc<AddressSpace, Cx>>) {
    addr_space
        .vm()
        .create_mapping(
            PHYS_IDENT_SIZE,
            vm::Location::Fixed(VirtAddr(PHYS_IDENT_OFFSET)),
            vm::Source::Raw(PhysAddr(0)),
            mmu::PageSize::Large,
            vm::Flags {
                mapping: vm::mappings::Flags { guard_pages: false },
                mmu: mmu::Flags {
                    user_accessible: false,
                    writable: true,
                    executable: false,
                    global: true,
                    cache_disabled: false,
                    writethrough: false,
                },
            },
        )
        .expect("failed to create kernel mappings");
    addr_space
        .vm()
        .create_mapping(
            VMA_SIZE,
            vm::Location::Fixed(VirtAddr(VMA_OFFSET)),
            vm::Source::Raw(PhysAddr(0)),
            mmu::PageSize::Normal,
            vm::Flags {
                mapping: vm::mappings::Flags { guard_pages: false },
                mmu: mmu::Flags {
                    user_accessible: false,
                    writable: true,
                    executable: true,
                    global: true,
                    cache_disabled: false,
                    writethrough: false,
                },
            },
        )
        .expect("failed to create kernel mappings");
}
