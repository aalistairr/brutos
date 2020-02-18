use core::pin::Pin;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicBool, Ordering};

use brutos_alloc::{AllocOne, Arc, ArcInner, OutOfMemory, PinWeak};
use brutos_memory_units::arch::PAGE_SIZE;
use brutos_memory_units::VirtAddr;
use brutos_memory_vm as vm;
use brutos_sync::mpsc;
use brutos_sync::spinlock::{Spinlock, SpinlockGuard};
use brutos_task::{sched, Task};

use crate::memory::AddressSpace;
use crate::Cx;

unsafe impl brutos_sync::Critical for Cx {
    unsafe fn enter_critical() {
        crate::arch::interrupt::mask();
        brutos_task::arch::current_task_inc_critical_count();
    }

    unsafe fn leave_critical() {
        if brutos_task::arch::current_task_dec_critical_count() {
            crate::arch::interrupt::unmask();
        }
    }
}

static SCHEDULER: sched::Scheduler<Cx> = sched::Scheduler::new();

pub fn scheduler() -> Pin<&'static sched::Scheduler<Cx>> {
    unsafe { Pin::new_unchecked(&SCHEDULER) }
}

pub fn initialize_scheduler() {
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
        crate::arch::interrupt::set_timer(1000000000);
        scheduler().unlock_and_yield(is_locked);
    }
}

pub unsafe fn yieldd() {
    let task = Task::<Cx>::current();
    let is_locked = SpinlockGuard::into_is_locked(task.switch_lock.lock());
    scheduler().schedule(scheduler().deschedule());
    <Cx as brutos_sync::waitq::Context>::unlock_and_yield(is_locked);
}

pub enum TaskAddrSpace {
    Active(Pin<Arc<AddressSpace, Cx>>),
    Inactive(PinWeak<AddressSpace, Cx>),
}

static mut IDLE_TASK: core::mem::MaybeUninit<Pin<Arc<Task<Cx>, Cx>>> =
    core::mem::MaybeUninit::uninit();

pub unsafe fn create_idle_task() -> Result<(), OutOfMemory> {
    let page_tables = AddressSpace::kernel()
        .vm()
        .mmu_tables()
        .lock()
        .page_tables();
    IDLE_TASK.write(Task::new(
        Spinlock::new(TaskAddrSpace::Inactive(Arc::pin_downgrade(
            AddressSpace::kernel(),
        ))),
        !0,
        brutos_task::EntryPoint::Kernel(
            crate::arch::idle_task_entry_addr(),
            crate::arch::idle_task_entry_arg(),
            0,
        ),
        page_tables,
    )?);
    Ok(())
}

enum JanitorSel {}

impl mpsc::Context<JanitorSel> for Cx {
    type ChannelSel = brutos_task::WaitQSel<Cx>;
}

static IS_JANITOR_CHANNEL_ALLOCATED: AtomicBool = AtomicBool::new(false);
static mut JANITOR_CHANNEL: core::mem::MaybeUninit<ArcInner<mpsc::Channel<JanitorSel, Cx>>> =
    core::mem::MaybeUninit::uninit();

unsafe impl AllocOne<ArcInner<mpsc::Channel<JanitorSel, Cx>>> for Cx {
    unsafe fn alloc(
        &mut self,
    ) -> Result<NonNull<ArcInner<mpsc::Channel<JanitorSel, Cx>>>, OutOfMemory> {
        IS_JANITOR_CHANNEL_ALLOCATED
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .expect("already allocated the janitor channel");
        Ok(NonNull::new_unchecked(JANITOR_CHANNEL.as_mut_ptr()))
    }

    unsafe fn dealloc(&mut self, _: NonNull<ArcInner<mpsc::Channel<JanitorSel, Cx>>>) {
        IS_JANITOR_CHANNEL_ALLOCATED.store(false, Ordering::Release);
    }
}

static mut JANITOR_TX: Option<mpsc::Sender<JanitorSel, Cx>> = None;

pub unsafe fn create_janitor() -> Result<(), OutOfMemory> {
    let (tx, rx) = mpsc::channel()?;
    JANITOR_TX = Some(tx);
    let page_tables = AddressSpace::kernel()
        .vm()
        .mmu_tables()
        .lock()
        .page_tables();
    let janitor = Task::new(
        Spinlock::new(TaskAddrSpace::Inactive(Arc::pin_downgrade(
            AddressSpace::kernel(),
        ))),
        !0 - 1,
        brutos_task::EntryPoint::Kernel(
            VirtAddr(janitor as usize),
            mpsc::Receiver::into_raw(rx) as usize,
            0,
        ),
        page_tables,
    )?;
    scheduler().as_ref().schedule(janitor);
    Ok(())
}

extern "C" fn janitor(rx_raw: *const mpsc::Channel<JanitorSel, Cx>) -> ! {
    let mut rx = unsafe { mpsc::Receiver::from_raw(rx_raw) };
    loop {
        let _: Pin<Arc<Task<Cx>, Cx>> = rx.recv();
    }
}

const STACK_SIZE: usize = 16 * PAGE_SIZE;

impl brutos_task::Context for Cx {
    type AddrSpace = Spinlock<TaskAddrSpace, Cx>;

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

    fn activate_task(&mut self, task: &Pin<Arc<Task<Self>, Self>>) -> bool {
        let mut addr_space_guard = task.addr_space.lock();
        match &*addr_space_guard {
            TaskAddrSpace::Inactive(addr_space) => match addr_space.upgrade() {
                Some(addr_space) if addr_space.is_alive() => {
                    *addr_space_guard = TaskAddrSpace::Active(addr_space);
                    true
                }
                _ if unsafe { (*task.state.get()).regs.cs == brutos_task::arch::GDT_CODE_KERN } => {
                    true
                }
                _ => false,
            },
            TaskAddrSpace::Active(_) => panic!("task is already active"),
        }
    }

    fn deactivate_task(&mut self, task: &Pin<Arc<Task<Self>, Self>>) {
        let mut addr_space_guard = task.addr_space.lock();
        match &*addr_space_guard {
            TaskAddrSpace::Active(addr_space) => {
                *addr_space_guard = TaskAddrSpace::Inactive(Arc::pin_downgrade(addr_space))
            }
            TaskAddrSpace::Inactive(_) => panic!("task is already inactive"),
        }
    }

    fn is_task_active(&mut self, task: &Pin<Arc<Task<Self>, Self>>) -> bool {
        match &*task.addr_space.lock() {
            TaskAddrSpace::Active(addr_space) => addr_space.is_alive(),
            TaskAddrSpace::Inactive(_) => false,
        }
    }

    fn destroy_task(&mut self, task: Pin<Arc<Task<Self>, Self>>) {
        destroy_task(task);
    }
}

pub fn destroy_task(task: Pin<Arc<Task<Cx>, Cx>>) {
    unsafe {
        JANITOR_TX.as_ref().unwrap().send(task);
    }
}
