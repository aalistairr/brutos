#![feature(asm, global_asm, naked_functions)]
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
use core::pin::Pin;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicBool, Ordering};

use brutos_alloc::{AllocOne, Arc, ArcInner, OutOfMemory, PinWeak};
use brutos_memory_slab_alloc as slab;
use brutos_memory_traits::{AllocMappedPage, AllocPhysPage, MapPhysPage};
use brutos_memory_units::arch::PAGE_SIZE;
use brutos_memory_units::{Order, PhysAddr, VirtAddr};
use brutos_memory_vm as vm;
use brutos_sync::mpsc;
use brutos_sync::mutex::{Mutex, PinMutex};
use brutos_sync::spinlock::{Spinlock, SpinlockGuard};
use brutos_task::{sched, Task};

const STACK_SIZE: usize = 16 * PAGE_SIZE;

#[macro_use]
pub mod arch;
pub mod bootstrap;
pub mod memory;
pub mod syscall;

pub unsafe fn main(
    mmap: impl Clone + Iterator<Item = Range<PhysAddr>>,
    init_module: Option<&[u8]>,
) -> ! {
    println!("Loading BrutOS");
    memory::initialize();
    initialize_task_allocator();
    initialize_mapping_allocator();
    initialize_addr_space_allocator();
    initialize_scheduler();
    let available_memory = memory::bootstrap(mmap).expect("Failed to bootstrap physical memory");
    println!("{} bytes available", available_memory);
    create_kernel_address_space().expect("failed to create kernel address space");

    arch::initialize_with_address_space();

    create_idle_task().expect("failed to create idle task");
    create_janitor().expect("failed to create janitor");

    if let Some(init_module) = init_module {
        match bootstrap::create_bootstrap_task(init_module) {
            Ok(task) => {
                scheduler().as_ref().schedule(task);
                <Cx as brutos_sync::waitq::Context>::unlock_and_yield(&AtomicBool::new(true));
                unreachable!()
            }
            Err(e) => panic!("failed to run init: {:?}", e),
        }
    } else {
        panic!("nothing to do");
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
    const MAX_ORDER: Order = Order(brutos_memory_phys_alloc::MAX_ORDER);

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
    const MAX_ORDER: Order = Order(brutos_memory_phys_alloc::MAX_ORDER);

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
slab_allocator!(mapping_allocator, 1, ArcInner<vm::Mapping<Cx>>);
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
    is_alive: AtomicBool,
    vm: vm::Space<Cx>,
}

static mut KERNEL_ADDR_SPACE: core::mem::MaybeUninit<Pin<Arc<AddressSpace, Cx>>> =
    core::mem::MaybeUninit::uninit();

unsafe fn create_kernel_address_space() -> Result<(), OutOfMemory> {
    let addr_space = KERNEL_ADDR_SPACE.write(
        Arc::pin(AddressSpace {
            is_alive: AtomicBool::new(true),
            vm: vm::Space::new(
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

fn create_user_address_space() -> Result<Pin<Arc<AddressSpace, Cx>>, vm::mmu::MapError> {
    let addr_space = Arc::pin(AddressSpace {
        is_alive: AtomicBool::new(true),
        vm: vm::Space::new(crate::arch::memory::USER_ADDR_SPACE_RANGE, unsafe {
            crate::arch::memory::create_user_mmu_tables()?
        }),
    })
    .map_err(|(e, space)| {
        unsafe {
            crate::arch::memory::destroy_user_mmu_tables(Mutex::into_inner(space.vm.mmu_tables));
        }
        e
    })?;
    addr_space.vm().initialize();
    Ok(addr_space)
}

impl AddressSpace {
    pub unsafe fn kernel() -> &'static Pin<Arc<AddressSpace, Cx>> {
        &*KERNEL_ADDR_SPACE.as_ptr()
    }

    pub fn vm<'a>(self: &'a Pin<Arc<AddressSpace, Cx>>) -> Pin<&'a vm::Space<Cx>> {
        unsafe { self.as_ref().map_unchecked(|x| &x.vm) }
    }

    pub fn kill(&self) {
        self.is_alive.store(false, Ordering::Release);
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive.load(Ordering::Acquire)
    }
}

pub enum TaskAddrSpace {
    Active(Pin<Arc<AddressSpace, Cx>>),
    Inactive(PinWeak<AddressSpace, Cx>),
}

static mut IDLE_TASK: core::mem::MaybeUninit<Pin<Arc<Task<Cx>, Cx>>> =
    core::mem::MaybeUninit::uninit();

unsafe fn create_idle_task() -> Result<(), OutOfMemory> {
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
            arch::idle_task_entry_addr(),
            arch::idle_task_entry_arg(),
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

unsafe fn create_janitor() -> Result<(), OutOfMemory> {
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

    fn is_task_in_kernel(&mut self, task: &Pin<Arc<Task<Self>, Self>>) -> bool {
        unsafe { (*task.state.get()).regs.cs == brutos_task::arch::GDT_CODE_KERN }
    }

    fn destroy_task(&mut self, task: Pin<Arc<Task<Self>, Self>>) {
        destroy_task(task);
    }
}

fn destroy_task(task: Pin<Arc<Task<Cx>, Cx>>) {
    unsafe {
        JANITOR_TX.as_ref().unwrap().send(task);
    }
}

unsafe impl MapPhysPage for Cx {
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
