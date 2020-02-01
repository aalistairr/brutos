#![feature(
    const_fn,
    const_if_match,
    const_panic,
    const_alloc_layout,
    const_loop,
    const_mut_refs
)]
#![feature(const_generics)]
#![feature(asm)]
#![feature(generic_associated_types)]
#![cfg_attr(test, feature(test, never_type, vec_into_raw_parts))]
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![allow(incomplete_features)]

use core::ptr::NonNull;

pub mod arch;
pub mod phys_alloc;
pub mod slab_alloc;
pub mod unit;
pub mod vm;

pub use self::unit::{Order, PhysAddr, VirtAddr};

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
