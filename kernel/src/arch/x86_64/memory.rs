use core::ops::Range;
use core::pin::Pin;
use core::ptr::NonNull;

use brutos_alloc::{Arc, OutOfMemory};
use brutos_memory_phys_alloc::bootstrap::{self, CutRange};
use brutos_memory_traits::{AllocPhysPage, MapPhysPage, PageSize};
use brutos_memory_units::{Order, PhysAddr, VirtAddr};
use brutos_memory_vm as vm;
use brutos_platform_pc::mmu;

use crate::memory::addr_space::AddressSpace;
use crate::memory::alloc::FailedToBootstrap;
use crate::memory::Object;
use crate::Cx;

pub type MmuMap = mmu::Map<Cx>;

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

unsafe impl mmu::Context for Cx {
    fn alloc_table() -> Result<PhysAddr, OutOfMemory> {
        <Cx as AllocPhysPage>::alloc(Order(0))
            .map(|(addr, _)| addr)
            .map_err(|()| OutOfMemory)
    }

    unsafe fn dealloc_table(addr: PhysAddr) {
        <Cx as AllocPhysPage>::dealloc(addr, Order(0));
    }

    fn map_table(addr: PhysAddr) -> NonNull<mmu::Table> {
        map_phys_ident(addr, Order(0).size())
            .expect("Failed to map page translation table into memory")
            .cast()
    }
}

static mut KERNEL_PML4: [mmu::Entry; 256] = [mmu::Entry::new(); 256];

pub unsafe fn create_kernel_mmu_map() -> Result<mmu::Map<Cx>, OutOfMemory> {
    let mut map = mmu::Map::with_root(
        mmu::Entry::new()
            .with_address(pml4_phys())
            .with_permanent(true)
            .with_present(true),
    );
    for pml4e_i in 0..256 {
        KERNEL_PML4[pml4e_i] = map
            .create_permanent_table(
                Default::default(),
                mmu::Level::Pml4,
                KERNEL_ADDR_SPACE_RANGE.start + pml4e_i * mmu::Level::Pml4.entry_size(),
            )
            .expect("failed to create kernel mmu tables");
    }
    Ok(map)
}

pub unsafe fn destroy_kernel_mmu_map(_mmu_map: mmu::Map<Cx>) {}

pub unsafe fn create_user_mmu_map() -> Result<mmu::Map<Cx>, mmu::MapError> {
    let mut map = mmu::Map::new();
    for pml4e_i in 0..256 {
        map.set_entry(
            Default::default(),
            mmu::Level::Pml4,
            KERNEL_ADDR_SPACE_RANGE.start + pml4e_i * mmu::Level::Pml4.entry_size(),
            KERNEL_PML4[pml4e_i],
        )
        .map_err(|e| match e {
            mmu::SetError::Exists => unreachable!(),
            mmu::SetError::OutOfMemory => mmu::MapError::OutOfMemory,
            mmu::SetError::Obstructed => mmu::MapError::Obstructed,
        })?;
    }
    Ok(map)
}

pub unsafe fn destroy_user_mmu_map(mut map: mmu::Map<Cx>) {
    for pml4e_i in 0..256 {
        let _ = map.clear_entry(
            mmu::Level::Pml4,
            KERNEL_ADDR_SPACE_RANGE.start + pml4e_i * mmu::Level::Pml4.entry_size(),
        );
    }
}

const SHARED_EMPTY_PAGE_ORDER: Order = Order(9);
static mut SHARED_EMPTY_PAGE: Option<(PhysAddr, &<Cx as AllocPhysPage>::PageData)> = None;

pub fn initialize_with_phys_alloc() {
    let shared_empty_page = <Cx as AllocPhysPage>::alloc(SHARED_EMPTY_PAGE_ORDER)
        .expect("failed to allocate shared empty page");
    unsafe {
        <Cx as MapPhysPage>::write_bytes(shared_empty_page.0, 0, SHARED_EMPTY_PAGE_ORDER)
            .expect("failed to zero shared empty page");
    }
    shared_empty_page.1.as_ref().inc_page_refcount();
    unsafe {
        SHARED_EMPTY_PAGE = Some(shared_empty_page);
    }
}

pub fn shared_empty_page(
    page_size: PageSize,
) -> Option<(PhysAddr, &'static <Cx as AllocPhysPage>::PageData)> {
    if page_size.order() <= SHARED_EMPTY_PAGE_ORDER {
        let &shared_empty_page = unsafe { SHARED_EMPTY_PAGE.as_ref().unwrap() };
        Some(shared_empty_page)
    } else {
        None
    }
}

pub fn create_kernel_mappings(addr_space: &Pin<Arc<AddressSpace, Cx>>) {
    addr_space
        .vm()
        .create_mapping(
            PHYS_IDENT_SIZE,
            vm::Location::Fixed(VirtAddr(PHYS_IDENT_OFFSET)),
            vm::Source::Private(Object::Raw(PhysAddr(0))),
            vm::PageSize::Large,
            vm::Flags {
                mapping: vm::MappingFlags {
                    guarded: false,
                    wired: true,
                },
                mmu: vm::MmuFlags {
                    user_accessible: false,
                    writable: true,
                    executable: false,
                    global: true,
                    copied: false,
                    cache_type: 0,
                },
            },
        )
        .expect("failed to create kernel mappings");
    addr_space
        .vm()
        .create_mapping(
            VMA_SIZE,
            vm::Location::Fixed(VirtAddr(VMA_OFFSET)),
            vm::Source::Private(Object::Raw(PhysAddr(0))),
            vm::PageSize::Normal,
            vm::Flags {
                mapping: vm::MappingFlags {
                    guarded: false,
                    wired: true,
                },
                mmu: vm::MmuFlags {
                    user_accessible: false,
                    writable: true,
                    executable: true,
                    global: true,
                    copied: false,
                    cache_type: 0,
                },
            },
        )
        .expect("failed to create kernel mappings");
}
