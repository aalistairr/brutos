#![feature(const_fn, const_if_match, const_panic, const_alloc_layout, const_loop)]
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

pub use self::unit::{PhysAddr, VirtAddr};

pub unsafe trait AllocMappedPage {
    const MAX_ORDER: u8;

    fn alloc(order: u8) -> Result<NonNull<u8>, ()>;
    unsafe fn dealloc(ptr: NonNull<u8>, order: u8);
}

pub unsafe trait AllocPhysPage {
    const MAX_ORDER: u8;

    type PageData: 'static;

    fn alloc(order: u8) -> Result<(PhysAddr, &'static Self::PageData), ()>;
    unsafe fn dealloc(addr: PhysAddr, order: u8);
    fn get_data(addr: PhysAddr) -> &'static Self::PageData;
}

pub unsafe trait MapPhysPage {
    type Err;

    unsafe fn with_mapped_page<F, R>(addr: PhysAddr, size: usize, f: F) -> Result<R, Self::Err>
    where
        F: FnOnce(*mut u8) -> R;

    unsafe fn write_bytes(addr: PhysAddr, val: u8, count: usize) -> Result<(), Self::Err> {
        Self::with_mapped_page(addr, count, |ptr: *mut u8| {
            core::ptr::write_bytes(ptr, val, count)
        })
    }

    unsafe fn copy(src: PhysAddr, dst: PhysAddr, count: usize) -> Result<(), Self::Err> {
        Self::with_mapped_page(src, count, |src: *mut u8| {
            Self::with_mapped_page(dst, count, |dst: *mut u8| {
                core::ptr::copy(src as *const u8, dst, count)
            })
        })
        .and_then(core::convert::identity)
    }
}
