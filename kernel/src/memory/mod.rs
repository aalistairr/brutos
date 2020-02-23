use core::ptr::NonNull;

use brutos_alloc::OutOfMemory;
use brutos_memory_traits::{AllocMappedPage, AllocPhysPage, MapPhysPage, PageSize};
use brutos_memory_units::{Order, PhysAddr};
use brutos_memory_vm::PageRefCount;

use crate::arch::memory::shared_empty_page;
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

unsafe impl brutos_memory_vm::Context for Cx {
    type Obj = Object;
    type MmuMap = crate::arch::memory::MmuMap;
}

pub enum Object {
    Raw(PhysAddr),
    Anonymous,
}

static RAW_PAGE_DATA: PageData = PageData {
    ref_count: PageRefCount::new(),
};

unsafe impl brutos_memory_vm::Object for Object {
    type Meta = PageData;
    type GenerateErr = OutOfMemory;

    fn writable(&self) -> bool {
        match self {
            Object::Raw(_) | Object::Anonymous => true,
        }
    }

    fn generate_page(
        &self,
        offset: usize,
        page_size: PageSize,
    ) -> Result<(PhysAddr, &Self::Meta), Self::GenerateErr> {
        match self {
            &Object::Raw(start_addr) => Ok((start_addr + offset, &RAW_PAGE_DATA)),
            Object::Anonymous => match shared_empty_page(page_size) {
                Some(x) => Ok(x),
                None => {
                    let page = <Cx as AllocPhysPage>::alloc(page_size.order())
                        .map_err(|()| OutOfMemory)?;
                    unsafe {
                        <Cx as MapPhysPage>::write_bytes(page.0, 0, page_size.order())
                            .expect("failed to zero shared empty page");
                    }
                    Ok(page)
                }
            },
        }
    }

    fn destroy_page(&self, page: PhysAddr, _offset: usize, page_size: PageSize) {
        match self {
            Object::Raw(_) => (),
            Object::Anonymous => match shared_empty_page(page_size) {
                Some(_) => (),
                None => unsafe {
                    <Cx as AllocPhysPage>::dealloc(page, page_size.order());
                },
            },
        }
    }

    fn meta(&self, page: PhysAddr) -> &Self::Meta {
        match self {
            Object::Raw(_) => &RAW_PAGE_DATA,
            Object::Anonymous => <Cx as AllocPhysPage>::get_data(page),
        }
    }

    fn unique_page(&self, meta: &Self::Meta) -> bool {
        match self {
            Object::Raw(_) => true,
            Object::Anonymous => meta.as_ref().unique_page(),
        }
    }

    fn inc_page_refcount(&self, meta: &Self::Meta) -> bool {
        match self {
            Object::Raw(_) => true,
            Object::Anonymous => meta.as_ref().inc_page_refcount(),
        }
    }

    fn dec_page_refcount(&self, meta: &Self::Meta) -> bool {
        match self {
            Object::Raw(_) => false,
            Object::Anonymous => meta.as_ref().dec_page_refcount(),
        }
    }
}
