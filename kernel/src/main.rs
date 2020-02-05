#![feature(asm, global_asm, naked_functions)]
#![feature(const_raw_ptr_deref, const_mut_refs)]
#![feature(maybe_uninit_extra)]
#![feature(format_args_nl)]
#![no_std]
#![no_main]

use core::ops::Range;
use core::pin::Pin;
use core::ptr::NonNull;
use core::sync::atomic::AtomicBool;

use brutos_alloc::{AllocOne, Arc, ArcInner, OutOfMemory, PinWeak};
use brutos_memory::arch::PAGE_SIZE;
use brutos_memory::slab_alloc as slab;
use brutos_memory::vm;
use brutos_memory::{AllocMappedPage, AllocPhysPage, Order, PhysAddr, VirtAddr};
use brutos_sync::mutex::{Mutex, PinMutex};
use brutos_sync::spinlock::SpinlockGuard;
use brutos_task::{sched, Task};

const STACK_SIZE: usize = 16 * PAGE_SIZE;

#[macro_use]
pub mod arch;
pub mod memory;

pub unsafe fn main(mmap: impl Clone + Iterator<Item = Range<PhysAddr>>) -> ! {
    println!("Loading BrutOS");
    memory::initialize();
    initialize_task_allocator();
    initialize_mapping_allocator();
    initialize_addr_space_allocator();
    initialize_scheduler();
    let available_memory = memory::bootstrap(mmap).expect("Failed to bootstrap physical memory");
    println!("{} bytes available", available_memory);
    create_kernel_address_space().expect("failed to create kernel address space");
    create_idle_task().expect("failed to create idle task");

    for i in 0..32 {
        scheduler().as_ref().schedule(
            Task::new(
                Arc::pin_downgrade(AddressSpace::kernel()),
                0,
                brutos_task::EntryPoint::Kernel(VirtAddr(start_task as usize), i, 0),
            )
            .expect("failed to create task"),
        );
    }
    <Cx as brutos_sync::waitq::Context>::unlock_and_yield(&AtomicBool::new(true));
    unreachable!()
}

extern "C" fn start_task(n: usize) -> ! {
    unsafe {
        arch::interrupt::unmask();
    }
    let mut i = 0;
    loop {
        println!("task {}: {}", n, i);
        i += 1;
    }
}

#[derive(Default)]
pub struct Cx;

unsafe impl brutos_sync::Critical for Cx {
    unsafe fn enter_critical() {
        self::arch::interrupt::mask();
        brutos_task::arch::current_task_inc_critical_count();
    }

    unsafe fn leave_critical() {
        if brutos_task::arch::current_task_dec_critical_count() {
            self::arch::interrupt::unmask();
        }
    }
}

static SCHEDULER: sched::Scheduler<Cx> = sched::Scheduler::new();

fn scheduler() -> Pin<&'static sched::Scheduler<Cx>> {
    unsafe { Pin::new_unchecked(&SCHEDULER) }
}

fn initialize_scheduler() {
    scheduler().initialize();
}

unsafe impl brutos_sync::waitq::Context for Cx {
    type WaitQSel = brutos_task::WaitQSel<Cx>;

    unsafe fn deschedule(&mut self) -> Pin<Arc<Task<Cx>, Cx>> {
        scheduler().deschedule()
    }

    unsafe fn schedule(&mut self, task: Pin<Arc<Task<Cx>, Cx>>) {
        scheduler().schedule(task);
    }

    unsafe fn unlock_and_yield(is_locked: &AtomicBool) {
        arch::interrupt::set_timer(1000000000);
        scheduler().unlock_and_yield(is_locked);
    }
}

pub unsafe fn yieldd() {
    let task = Task::<Cx>::current();
    let is_locked = SpinlockGuard::into_is_locked(task.switch_lock.lock());
    scheduler().schedule(scheduler().deschedule());
    <Cx as brutos_sync::waitq::Context>::unlock_and_yield(is_locked);
}

unsafe impl AllocPhysPage for Cx {
    const MAX_ORDER: Order = Order(brutos_memory::phys_alloc::MAX_ORDER);

    type PageData = memory::PageData;

    fn alloc(order: Order) -> Result<(PhysAddr, &'static Self::PageData), ()> {
        self::memory::phys_allocator()
            .lock()
            .as_mut()
            .allocate(order)
            .expect("Allocation is too large")
            .ok_or(())
    }

    unsafe fn dealloc(addr: PhysAddr, _: Order) {
        self::memory::phys_allocator()
            .lock()
            .as_mut()
            .free(addr)
            .expect("Failed to deallocate")
    }

    fn get_data(addr: PhysAddr) -> &'static Self::PageData {
        self::memory::get_data(addr).expect("Address is not allocated")
    }
}

unsafe impl AllocMappedPage for Cx {
    const MAX_ORDER: Order = Order(brutos_memory::phys_alloc::MAX_ORDER);

    fn alloc(order: Order) -> Result<NonNull<u8>, ()> {
        self::memory::phys_allocator()
            .lock()
            .as_mut()
            .allocate(order)
            .expect("Allocation is too large")
            .ok_or(())
            .and_then(|(addr, _)| self::arch::memory::map_phys_ident(addr, order.size()))
    }

    unsafe fn dealloc(ptr: NonNull<u8>, _: Order) {
        self::memory::phys_allocator()
            .lock()
            .as_mut()
            .free(self::arch::memory::phys_ident_addr(ptr))
            .expect("Failed to deallocate")
    }
}

macro_rules! slab_allocator {
    ($n:ident, $o:expr, $t:ty) => {
        fn $n() -> Pin<&'static PinMutex<slab::Allocator<Cx>, Cx>> {
            static ALLOCATOR: PinMutex<slab::Allocator<Cx>, Cx> =
                PinMutex::new(slab::Allocator::new::<$t>(Order($o)));
            unsafe { Pin::new_unchecked(&ALLOCATOR) }
        }

        unsafe impl AllocOne<$t> for Cx {
            unsafe fn alloc(&mut self) -> Result<NonNull<$t>, OutOfMemory> {
                $n().lock().as_mut().alloc().map(NonNull::cast)
            }
            unsafe fn dealloc(&mut self, ptr: NonNull<$t>) {
                $n().lock().as_mut().dealloc(ptr.cast())
            }
        }
    };
}

slab_allocator!(task_allocator, 1, ArcInner<Task<Cx>>);
fn initialize_task_allocator() {
    task_allocator().initialize();
    task_allocator().lock().as_mut().initialize();
}
slab_allocator!(
    mapping_allocator,
    1,
    ArcInner<brutos_memory::vm::Mapping<Cx>>
);
fn initialize_mapping_allocator() {
    mapping_allocator().initialize();
    mapping_allocator().lock().as_mut().initialize();
}
slab_allocator!(addr_space_allocator, 1, ArcInner<AddressSpace>);
fn initialize_addr_space_allocator() {
    addr_space_allocator().initialize();
    addr_space_allocator().lock().as_mut().initialize();
}

pub struct AddressSpace {
    vm: brutos_memory::vm::Space<Cx>,
}

static mut KERNEL_ADDR_SPACE: core::mem::MaybeUninit<Pin<Arc<AddressSpace, Cx>>> =
    core::mem::MaybeUninit::uninit();

unsafe fn create_kernel_address_space() -> Result<(), OutOfMemory> {
    let addr_space = KERNEL_ADDR_SPACE.write(
        Arc::pin(AddressSpace {
            vm: brutos_memory::vm::Space::new(
                crate::arch::memory::KERNEL_ADDR_SPACE_RANGE,
                crate::arch::memory::create_kernel_mmu_tables()?,
            ),
        })
        .map_err(|(e, space)| {
            crate::arch::memory::destroy_kernel_mmu_tables(Mutex::into_inner(space.vm.mmu_tables));
            e
        })?,
    );
    addr_space.vm().initialize();
    arch::memory::create_kernel_mappings(addr_space);
    Ok(())
}

impl AddressSpace {
    pub unsafe fn kernel() -> &'static Pin<Arc<AddressSpace, Cx>> {
        &*KERNEL_ADDR_SPACE.as_ptr()
    }

    pub fn vm<'a>(self: &'a Pin<Arc<AddressSpace, Cx>>) -> Pin<&'a brutos_memory::vm::Space<Cx>> {
        unsafe { self.as_ref().map_unchecked(|x| &x.vm) }
    }
}

static mut IDLE_TASK: core::mem::MaybeUninit<Pin<Arc<Task<Cx>, Cx>>> =
    core::mem::MaybeUninit::uninit();

unsafe fn create_idle_task() -> Result<(), OutOfMemory> {
    IDLE_TASK.write(Task::new(
        Arc::pin_downgrade(AddressSpace::kernel()),
        !0,
        brutos_task::EntryPoint::Kernel(
            arch::idle_task_entry_addr(),
            arch::idle_task_entry_arg(),
            0,
        ),
    )?);
    Ok(())
}

impl brutos_task::Context for Cx {
    type AddrSpace = PinWeak<AddressSpace, Cx>;

    fn alloc_stack(&mut self) -> Result<VirtAddr, OutOfMemory> {
        use vm::mappings::MapError;
        use vm::mmu;
        use vm::FillError;
        let kernel = unsafe { AddressSpace::kernel() };
        let mapping = kernel
            .vm()
            .create_mapping(
                STACK_SIZE,
                vm::Location::Aligned(PAGE_SIZE),
                vm::Source::Private(vm::Object::Anonymous),
                vm::mmu::PageSize::Normal,
                vm::Flags {
                    mapping: vm::mappings::Flags { guard_pages: true },
                    mmu: vm::mmu::Flags {
                        user_accessible: false,
                        writable: true,
                        executable: false,
                        global: true,
                        cache_disabled: false,
                        writethrough: false,
                    },
                },
            )
            .map_err(|e| match e {
                MapError::OutOfSpace
                | MapError::OutsideSpaceRange
                | MapError::InvalidParameters => unreachable!(),
                MapError::OutOfMemory => OutOfMemory,
            })?;
        kernel.vm().prefill(mapping.as_ref()).map_err(|e| match e {
            FillError::Map(mmu::MapError::OutOfMemory) | FillError::OutOfMemory => OutOfMemory,
            FillError::Map(mmu::MapError::Obstructed)
            | FillError::Map(mmu::MapError::NotAllocated)
            | FillError::MappingWasDestroyed
            | FillError::MapPhysPage(()) => unreachable!(),
        })?;
        Ok(mapping.range.end)
    }

    unsafe fn dealloc_stack(&mut self, addr: VirtAddr) {
        let kernel = AddressSpace::kernel();
        kernel
            .vm()
            .remove_mapping(addr - STACK_SIZE)
            .expect("failed to deallocate stack");
    }

    fn idle_task(&mut self) -> &Pin<Arc<Task<Cx>, Cx>> {
        unsafe { &*IDLE_TASK.as_ptr() }
    }
}

unsafe impl brutos_memory::MapPhysPage for Cx {
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
