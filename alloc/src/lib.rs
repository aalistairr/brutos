#![feature(fundamental)]
#![feature(receiver_trait)]
#![feature(exact_size_is_empty)]
#![feature(core_intrinsics)]
#![feature(alloc_layout_extra)]
#![feature(specialization)]
#![feature(allocator_api)]
#![cfg_attr(not(any(test, feature = "std")), no_std)]

use core::marker::PhantomData;
use core::ptr::NonNull;

pub use core::alloc::Layout;

pub mod arc;
pub mod boxed;
pub mod unique;

pub use self::arc::{Arc, ArcInner, PinWeak, Weak};
pub use self::boxed::Box;
pub use self::unique::Unique;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct OutOfMemory;

pub unsafe trait AllocOne<T> {
    unsafe fn alloc(&mut self) -> Result<NonNull<T>, OutOfMemory>;
    unsafe fn dealloc(&mut self, ptr: NonNull<T>);
}

#[cfg(any(test, feature = "std"))]
unsafe impl<T, A: core::alloc::Alloc> AllocOne<T> for A {
    unsafe fn alloc(&mut self) -> Result<NonNull<T>, OutOfMemory> {
        self.alloc(Layout::new::<T>())
            .map(NonNull::cast)
            .map_err(|_| OutOfMemory)
    }

    unsafe fn dealloc(&mut self, ptr: NonNull<T>) {
        self.dealloc(ptr.cast(), Layout::new::<T>())
    }
}

pub struct TransparentAlloc<T, A: AllocOne<T>>(A, PhantomData<*const T>);

impl<T, A: Default + AllocOne<T>> Default for TransparentAlloc<T, A> {
    fn default() -> TransparentAlloc<T, A> {
        TransparentAlloc(A::default(), PhantomData)
    }
}

unsafe impl<U, T, A: AllocOne<T>> AllocOne<U> for TransparentAlloc<T, A> {
    unsafe fn alloc(&mut self) -> Result<NonNull<U>, OutOfMemory> {
        assert_eq!(Layout::new::<U>(), Layout::new::<T>());
        self.0.alloc().map(NonNull::cast)
    }

    unsafe fn dealloc(&mut self, ptr: NonNull<U>) {
        assert_eq!(Layout::new::<U>(), Layout::new::<T>());
        self.0.dealloc(ptr.cast());
    }
}
