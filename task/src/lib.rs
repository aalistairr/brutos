#![feature(asm, global_asm)]
#![feature(const_if_match, const_panic)]
#![cfg_attr(not(any(test, feature = "std")), no_std)]

use core::cell::UnsafeCell;
use core::pin::Pin;

use brutos_alloc::{AllocOne, Arc, ArcInner, OutOfMemory};
use brutos_memory::VirtAddr;
use brutos_sync::spinlock::Spinlock;
use brutos_util::linked_list::Node;

pub mod arch;
pub mod sched;

pub trait Context: Default + AllocOne<ArcInner<Task<Self>>> + brutos_sync::Critical {
    type Process;

    fn alloc_stack(&mut self) -> Result<VirtAddr, OutOfMemory>;
    unsafe fn dealloc_stack(&mut self, stack: VirtAddr);
}

pub struct Task<Cx: Context> {
    pub process: Cx::Process,
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
    Kernel(VirtAddr, usize),
    #[cfg(feature = "user-mode")]
    User(VirtAddr, usize),
}

impl<Cx: Context> Task<Cx> {
    pub fn new(
        process: Cx::Process,
        id: usize,
        entry_point: EntryPoint,
    ) -> Result<Pin<Arc<Task<Cx>, Cx>>, OutOfMemory> {
        let kernel_stack = Cx::default().alloc_stack()?;
        let task = Arc::pin(Task {
            process,
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
