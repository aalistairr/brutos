#![feature(asm, global_asm, naked_functions)]
#![no_std]
#![no_main]

use core::ops::Range;
use core::pin::Pin;
use core::ptr::NonNull;
use core::sync::atomic::AtomicBool;

use brutos_alloc::{AllocOne, Arc, ArcInner, OutOfMemory};
use brutos_memory::{PhysAddr, VirtAddr};
use brutos_task::Task;

#[macro_use]
pub mod arch;
pub mod memory;
#[cfg(not(test))]
pub mod panic;

pub unsafe fn main(mmap: impl Clone + Iterator<Item = Range<PhysAddr>>) -> ! {
    println!("Loading BrutOS");
    memory::initialize();
    let available_memory = memory::bootstrap(mmap).expect("Failed to bootstrap physical memory");
    println!("{} bytes available", available_memory);
    unimplemented!()
}

#[derive(Default)]
pub struct Cx;

unsafe impl brutos_sync::Critical for Cx {
    unsafe fn enter_critical() {
        brutos_task::arch::current_task_inc_critical_count();
    }

    unsafe fn leave_critical() {
        if brutos_task::arch::current_task_dec_critical_count() {
            unimplemented!()
        }
    }
}

unsafe impl brutos_sync::waitq::Context for Cx {
    type WaitQSel = brutos_task::WaitQSel<Cx>;

    unsafe fn deschedule(&mut self) -> Pin<Arc<Task<Cx>, Cx>> {
        unimplemented!()
    }

    unsafe fn schedule(&mut self, _: Pin<Arc<Task<Cx>, Cx>>) {
        unimplemented!()
    }

    unsafe fn unlock_and_yield(_: &AtomicBool) {
        unimplemented!()
    }
}

impl brutos_task::Context for Cx {
    type Process = ();

    fn alloc_stack(&mut self) -> Result<VirtAddr, OutOfMemory> {
        unimplemented!()
    }

    unsafe fn dealloc_stack(&mut self, _: VirtAddr) {
        unimplemented!()
    }
}

unsafe impl AllocOne<ArcInner<Task<Cx>>> for Cx {
    unsafe fn alloc(&mut self) -> Result<NonNull<ArcInner<Task<Cx>>>, OutOfMemory> {
        unimplemented!()
    }

    unsafe fn dealloc(&mut self, _: NonNull<ArcInner<Task<Cx>>>) {
        unimplemented!()
    }
}
