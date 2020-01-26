#![feature(const_fn)]
#![cfg_attr(not(any(test, feature = "std")), no_std)]

pub mod condvar;
pub mod mutex;
pub mod spinlock;
pub mod waitq;

pub unsafe trait Critical {
    unsafe fn enter_critical();
    unsafe fn leave_critical();
}
