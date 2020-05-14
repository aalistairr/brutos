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
    <Cx as brutos_sync::Critical>::enter_critical();
    println!("Loading BrutOS");
    memory::alloc::initialize();
    task::initialize_scheduler();
    let available_memory =
        memory::alloc::bootstrap(mmap).expect("Failed to bootstrap physical memory");
    println!("{} bytes available", available_memory);
    memory::initialize_with_phys_alloc();
    memory::addr_space::create_kernel_address_space()
        .expect("failed to create kernel address space");

    arch::initialize_with_address_space();

    task::create_idle_task().expect("failed to create idle task");
    task::create_janitor().expect("failed to create janitor");

    // run_demo();
    run_bootstrap(init_module);
}

unsafe fn run_bootstrap(init_module: Option<&[u8]>) -> ! {
    // schedule_demo();
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

unsafe fn schedule_demo() {
    use brutos_alloc::Arc;
    use brutos_memory_units::VirtAddr;
    use brutos_sync::spinlock::Spinlock;
    use brutos_task::Task;

    use crate::memory::addr_space::AddressSpace;
    use crate::task::TaskAddrSpace;
    let page_tables = AddressSpace::kernel()
        .vm()
        .mmu_map()
        .lock()
        .page_tables()
        .expect("the kernel has no page tables");
    for i in 0..10 {
        let task = Task::new(
            Spinlock::new(TaskAddrSpace::Inactive(Arc::pin_downgrade(
                AddressSpace::kernel(),
            ))),
            i,
            brutos_task::EntryPoint::Kernel(VirtAddr(demo as usize), i, 0),
            page_tables,
        )
        .expect("Failed to create demo task");
        task::scheduler().as_ref().schedule(task);
    }
}

unsafe fn run_demo() -> ! {
    schedule_demo();
    <Cx as brutos_sync::waitq::Context>::unlock_and_yield(&AtomicBool::new(true));
    unreachable!()
}

extern "C" fn demo(i: usize) {
    loop {
        println!("Task {} says hi", i);
    }
}
