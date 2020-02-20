use core::ptr::NonNull;

use brutos_memory_traits::{AllocMappedPage, AllocPhysPage, MapPhysPage};
use brutos_memory_units::{Order, PhysAddr};
use brutos_memory_vm::PageRefCount;

use crate::Cx;

pub mod addr_space;
pub mod alloc;

#[derive(Default)]
pub struct PageData {
    ref_count: PageRefCount,
}

impl AsRef<PageRefCount> for PageData {
    fn as_ref(&self) -> &PageRefCount {
        &self.ref_count
    }
}

unsafe impl AllocMappedPage for Cx {
    const MAX_ORDER: Order = <Cx as AllocPhysPage>::MAX_ORDER;

    fn alloc(order: Order) -> Result<NonNull<u8>, ()> {
        <Cx as AllocPhysPage>::alloc(order)
            .and_then(|(addr, _)| crate::arch::memory::map_phys_ident(addr, order.size()))
    }

    unsafe fn dealloc(ptr: NonNull<u8>, order: Order) {
        <Cx as AllocPhysPage>::dealloc(crate::arch::memory::phys_ident_addr(ptr), order)
    }
}

unsafe impl MapPhysPage for Cx {
    type Err = ();

    unsafe fn with_mapped_page<F, R>(addr: PhysAddr, order: Order, f: F) -> Result<R, Self::Err>
    where
        F: FnOnce(*mut u8) -> R,
    {
        crate::arch::memory::map_phys_ident(addr, order.size())
            .map(NonNull::as_ptr)
            .map(f)
    }
}
