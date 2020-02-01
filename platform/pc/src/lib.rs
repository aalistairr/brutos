#![feature(asm)]
#![feature(const_fn, const_if_match, const_panic, const_mut_refs)]
#![cfg_attr(not(any(test, feature = "std")), no_std)]

pub mod apic;
pub mod idt;
pub mod msr;
