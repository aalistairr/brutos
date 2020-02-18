#![feature(asm, global_asm, naked_functions, core_intrinsics)]
#![feature(
    const_raw_ptr_deref,
    const_mut_refs,
    const_if_match,
    const_fn,
    const_panic
)]
#![feature(maybe_uninit_extra)]
#![feature(format_args_nl)]
#![no_std]
#![no_main]

use core::ops::Range;
use core::sync::atomic::AtomicBool;

use brutos_memory_units::PhysAddr;

#[macro_use]
pub mod arch;
pub mod bootstrap;
pub mod memory;
pub mod syscall;
pub mod task;

#[derive(Default)]
pub struct Cx;

pub unsafe fn main(
    mmap: impl Clone + Iterator<Item = Range<PhysAddr>>,
    init_module: Option<&[u8]>,
) -> ! {
    println!("Loading BrutOS");
    memory::initialize();
    memory::initialize_task_allocator();
    memory::initialize_mapping_allocator();
    memory::initialize_addr_space_allocator();
    task::initialize_scheduler();
    let available_memory = memory::bootstrap(mmap).expect("Failed to bootstrap physical memory");
    println!("{} bytes available", available_memory);
    memory::create_kernel_address_space().expect("failed to create kernel address space");

    arch::initialize_with_address_space();

    task::create_idle_task().expect("failed to create idle task");
    task::create_janitor().expect("failed to create janitor");

    if let Some(init_module) = init_module {
        match bootstrap::create_bootstrap_task(init_module) {
            Ok(task) => {
                task::scheduler().as_ref().schedule(task);
                <Cx as brutos_sync::waitq::Context>::unlock_and_yield(&AtomicBool::new(true));
                unreachable!()
            }
            Err(e) => panic!("failed to run init: {:?}", e),
        }
    } else {
        panic!("nothing to do");
    }
}
