use core::pin::Pin;

use brutos_alloc::Arc;
use brutos_sync::spinlock::{PinSpinlock, Spinlock};
use brutos_util::linked_list::LinkedList;

use crate::{Context, Task, WaitQSel};

pub struct Scheduler<Cx: Context> {
    current: Spinlock<Option<Pin<Arc<Task<Cx>, Cx>>>, Cx>,
    waiting: PinSpinlock<LinkedList<WaitQSel<Cx>>, Cx>,
}

impl<Cx: Context> Scheduler<Cx> {
    fn waiting(self: Pin<&Self>) -> Pin<&PinSpinlock<LinkedList<WaitQSel<Cx>>, Cx>> {
        unsafe { self.map_unchecked(|x| &x.waiting) }
    }

    pub fn deschedule(&self) -> Pin<Arc<Task<Cx>, Cx>> {
        self.current
            .lock()
            .take()
            .expect("Already descheduled the current task")
    }

    pub fn schedule(self: Pin<&Self>, task: Pin<Arc<Task<Cx>, Cx>>) {
        self.waiting().lock().as_mut().push_back(task);
    }
}
