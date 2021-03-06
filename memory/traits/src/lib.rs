#![cfg_attr(not(any(test, feature = "std")), no_std)]

use core::ptr::NonNull;

pub use brutos_memory_units::{MmuFlags, Order, PageSize, PhysAddr, VirtAddr};

pub unsafe trait AllocMappedPage {
    const MAX_ORDER: Order;

    fn alloc(order: Order) -> Result<NonNull<u8>, ()>;
    unsafe fn dealloc(ptr: NonNull<u8>, order: Order);
}

pub unsafe trait AllocPhysPage {
    const MAX_ORDER: Order;

    type PageData: 'static;

    fn alloc(order: Order) -> Result<(PhysAddr, &'static Self::PageData), ()>;
    unsafe fn dealloc(addr: PhysAddr, order: Order);
    fn get_data(addr: PhysAddr) -> &'static Self::PageData;
}

pub unsafe trait MapPhysPage {
    type Err;

    unsafe fn with_mapped_page<F, R>(addr: PhysAddr, order: Order, f: F) -> Result<R, Self::Err>
    where
        F: FnOnce(*mut u8) -> R;

    unsafe fn write_bytes(addr: PhysAddr, val: u8, order: Order) -> Result<(), Self::Err> {
        Self::with_mapped_page(addr, order, |ptr: *mut u8| {
            core::ptr::write_bytes(ptr, val, order.size())
        })
    }

    unsafe fn copy(src: PhysAddr, dst: PhysAddr, order: Order) -> Result<(), Self::Err> {
        Self::with_mapped_page(src, order, |src: *mut u8| {
            Self::with_mapped_page(dst, order, |dst: *mut u8| {
                core::ptr::copy(src as *const u8, dst, order.size())
            })
        })
        .and_then(core::convert::identity)
    }
}

pub trait MmuMap {
    type MapErr;
    type UnmapErr;
    type GetErr;
    type Entry: MmuEntry<CacheType = Self::CacheType>;
    type CacheType: Copy;

    fn map_keep(
        &mut self,
        flags: MmuFlags<Self::CacheType>,
        page_size: PageSize,
        virt_addr: VirtAddr,
        phys_addr: PhysAddr,
    ) -> Result<bool, Self::MapErr>;
    fn map_replace(
        &mut self,
        flags: MmuFlags<Self::CacheType>,
        page_size: PageSize,
        virt_addr: VirtAddr,
        phys_addr: PhysAddr,
    ) -> Result<Option<Self::Entry>, Self::MapErr>;
    fn get_entry(
        &self,
        page_size: PageSize,
        virt_addr: VirtAddr,
    ) -> Result<Option<Self::Entry>, Self::GetErr>;
    fn compare_and_map(
        &mut self,
        flags: MmuFlags<Self::CacheType>,
        page_size: PageSize,
        virt_addr: VirtAddr,
        compare_entry: Self::Entry,
        phys_addr: PhysAddr,
    ) -> Result<bool, Self::MapErr>;
    fn unmap(
        &mut self,
        page_size: PageSize,
        virt_addr: VirtAddr,
    ) -> Result<Option<Self::Entry>, Self::UnmapErr>;
}

pub trait MmuEntry: Copy {
    type CacheType: Copy;

    fn address(&self, page_size: PageSize) -> PhysAddr;
    fn flags(&self, page_size: PageSize) -> MmuFlags<Self::CacheType>;
}
