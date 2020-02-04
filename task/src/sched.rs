use core::pin::Pin;
use core::sync::atomic::AtomicBool;

use brutos_alloc::Arc;
use brutos_sync::spinlock::{PinSpinlock, Spinlock};
use brutos_util::linked_list::LinkedList;

use crate::{Context, Task, WaitQSel};

/// Unsafety: Each CPU must have exclusive access to the `current` field. This is not yet implemented!
pub struct Scheduler<Cx: Context> {
    current: Spinlock<Option<Pin<Arc<Task<Cx>, Cx>>>, Cx>,
    waiting: PinSpinlock<LinkedList<WaitQSel<Cx>>, Cx>,
}

impl<Cx: Context> Scheduler<Cx> {
    pub const fn new() -> Scheduler<Cx> {
        Scheduler {
            current: Spinlock::new(None),
            waiting: PinSpinlock::new(LinkedList::new()),
        }
    }

    fn waiting(self: Pin<&Self>) -> Pin<&PinSpinlock<LinkedList<WaitQSel<Cx>>, Cx>> {
        unsafe { self.map_unchecked(|x| &x.waiting) }
    }

    pub fn initialize(self: Pin<&Self>) {
        self.waiting().lock().as_mut().initialize();
    }

    pub unsafe fn deschedule(self: Pin<&Self>) -> Pin<Arc<Task<Cx>, Cx>> {
        self.current
            .lock()
            .take()
            .expect("Already descheduled the current task")
    }

    pub unsafe fn schedule(self: Pin<&Self>, task: Pin<Arc<Task<Cx>, Cx>>) {
        self.waiting().lock().as_mut().push_back(task);
    }

    pub unsafe fn unlock_and_yield(self: Pin<&Self>, is_locked: &AtomicBool) {
        let mut current = self.current.lock();
        assert!(
            current.is_none(),
            "the current task must be descheduled before calling `unlock_and_yield`"
        );
        let next_task = self
            .waiting()
            .lock()
            .as_mut()
            .pop_front()
            .unwrap_or_else(|| Cx::default().idle_task().clone());
        let next_state = next_task.state.get();
        *current = Some(next_task);
        drop(current); // next_state remains valid until the task is switched as Scheduler.current is not shared between CPUs
        crate::arch::switch(is_locked, next_state);
    }
}
