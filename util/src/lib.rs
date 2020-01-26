#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![feature(const_fn)]

pub mod iter;
pub mod linked_list;
#[doc(hidden)]
pub mod macros;
pub mod pointer;
pub mod uint;

pub use self::uint::UInt;

pub enum Void {}

pub struct Guard<F: FnOnce()>(Option<F>);

impl<F: FnOnce()> Drop for Guard<F> {
    #[inline]
    fn drop(&mut self) {
        if let Some(f) = self.0.take() {
            f();
        }
    }
}

impl<F: FnOnce()> Guard<F> {
    #[inline]
    pub fn new(f: F) -> Guard<F> {
        Guard(Some(f))
    }

    #[inline]
    pub fn success(self) {
        core::mem::forget(self);
    }
}
