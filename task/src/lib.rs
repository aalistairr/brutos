#![feature(asm, global_asm)]
#![feature(const_fn, const_if_match, const_panic)]
#![cfg_attr(not(any(test, feature = "std")), no_std)]

use core::cell::UnsafeCell;
use core::mem::ManuallyDrop;
use core::pin::Pin;

use brutos_alloc::{AllocOne, Arc, ArcInner, OutOfMemory};
use brutos_memory::VirtAddr;
use brutos_sync::spinlock::Spinlock;
use brutos_util::linked_list::Node;
use brutos_util::NonSend;

pub mod arch;
pub mod sched;

pub trait Context: Default + AllocOne<ArcInner<Task<Self>>> + brutos_sync::Critical {
    type AddrSpace;

    fn alloc_stack(&mut self) -> Result<VirtAddr, OutOfMemory>;
    unsafe fn dealloc_stack(&mut self, stack: VirtAddr);

    fn idle_task(&mut self) -> &Pin<Arc<Task<Self>, Self>>;
}

pub struct Task<Cx: Context> {
    pub addr_space: Cx::AddrSpace,
    pub id: usize,
    pub switch_lock: Spinlock<(), Cx>,
    state: UnsafeCell<State<Cx>>,
    waitq_node: Node<WaitQSel<Cx>>,
    process_node: Node<ProcessSel<Cx>>,
}

brutos_util_macros::selector!(pub WaitQSel<Cx: Context>: Arc<Task<Cx>, Cx> => waitq_node);
brutos_util_macros::selector!(pub ProcessSel<Cx: Context>: Arc<Task<Cx>, Cx> => process_node);

unsafe impl<Cx: Send + Context> Send for Task<Cx> {}
unsafe impl<Cx: Send + Context> Sync for Task<Cx> {}

#[repr(C)]
pub struct State<Cx: Context> {
    regs: self::arch::Regs, // sizeof(regs) % sizeof(usize) == 0
    kernel_stack: VirtAddr, // sizeof(regs) + 0x00
    critical_count: usize,  // sizeof(regs) + sizeof(usize)
    task: *const Task<Cx>,  // sizeof(regs) + 2 * sizeof(usize)
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum EntryPoint {
    Kernel(VirtAddr, usize, usize),
    #[cfg(feature = "user-mode")]
    User(VirtAddr, usize),
}

impl<Cx: Context> Task<Cx> {
    pub fn new(
        addr_space: Cx::AddrSpace,
        id: usize,
        entry_point: EntryPoint,
    ) -> Result<Pin<Arc<Task<Cx>, Cx>>, OutOfMemory> {
        let kernel_stack = Cx::default().alloc_stack()?;
        let task = Arc::pin(Task {
            addr_space,
            id,
            switch_lock: Spinlock::new(()),
            state: UnsafeCell::new(State {
                regs: Default::default(),
                kernel_stack,
                critical_count: 0,
                task: core::ptr::null(),
            }),
            waitq_node: Node::new(),
            process_node: Node::new(),
        })
        .map_err(|(e, _)| e)?;
        unsafe {
            (*task.state.get()).initialize(&task, entry_point, kernel_stack);
        }
        Ok(task)
    }

    pub fn current() -> NonSend<ManuallyDrop<Pin<Arc<Task<Cx>, Cx>>>> {
        unsafe {
            NonSend::new(ManuallyDrop::new(Pin::new_unchecked(Arc::from_raw(
                Self::current_task_ptr(),
            ))))
        }
    }
}

impl<Cx: Context> Drop for Task<Cx> {
    fn drop(&mut self) {
        unsafe { Cx::default().dealloc_stack((*self.state.get()).kernel_stack) }
    }
}

impl<Cx: Context> State<Cx> {
    pub fn initialize(
        &mut self,
        task: &Pin<Arc<Task<Cx>, Cx>>,
        entry_point: EntryPoint,
        kernel_stack: VirtAddr,
    ) {
        self.task = &**task;
        self.regs.initialize(self, task, entry_point, kernel_stack);
    }

    pub fn dummy() -> State<Cx> {
        State {
            regs: Default::default(),
            kernel_stack: VirtAddr(0),
            critical_count: 0,
            task: core::ptr::null(),
        }
    }
}
