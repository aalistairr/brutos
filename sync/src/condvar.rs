use core::pin::Pin;
use core::sync::atomic::{AtomicUsize, Ordering};

use brutos_util::linked_list::LinkedList;

use crate::mutex::MutexGuard;
use crate::spinlock::{PinSpinlock, SpinlockGuard};
use crate::waitq;

pub struct Condvar<Cx: waitq::Context> {
    waitq: PinSpinlock<LinkedList<Cx::WaitQSel>, Cx>,
    mutex: AtomicUsize,
}

impl<Cx: waitq::Context> Condvar<Cx> {
    pub fn new() -> Condvar<Cx> {
        Condvar {
            waitq: PinSpinlock::new(LinkedList::new()),
            mutex: AtomicUsize::new(0),
        }
    }

    fn waitq(self: Pin<&Self>) -> Pin<&PinSpinlock<LinkedList<Cx::WaitQSel>, Cx>> {
        unsafe { self.map_unchecked(|x| &x.waitq) }
    }

    pub fn initialize(self: Pin<&Self>) {
        self.waitq().lock().as_mut().initialize();
    }

    pub fn wait<'a, T>(self: Pin<&Self>, guard: MutexGuard<'a, T, Cx>) -> MutexGuard<'a, T, Cx> {
        unsafe {
            let mutex = guard.mutex;
            self.verify(mutex.get_ref() as *const _ as usize);
            let mut waitq = self.waitq().lock();
            drop(guard);
            waitq.as_mut().push_back(Cx::default().deschedule());
            let is_locked = SpinlockGuard::into_is_locked(Pin::into_inner_unchecked(waitq));
            Cx::unlock_and_yield(is_locked);
            mutex.lock()
        }
    }

    pub fn wait_while<'a, T, F>(
        self: Pin<&Self>,
        mut guard: MutexGuard<'a, T, Cx>,
        mut condition: F,
    ) -> MutexGuard<'a, T, Cx>
    where
        F: FnMut(&mut T) -> bool,
    {
        while condition(&mut *guard) {
            guard = self.wait(guard);
        }
        guard
    }

    pub fn notify_one(self: Pin<&Self>) {
        if let Some(task) = self.waitq().lock().as_mut().pop_front() {
            unsafe {
                Cx::default().schedule(task);
            }
        }
    }

    pub fn notify_all(self: Pin<&Self>) {
        let mut q = LinkedList::new();
        let mut q = unsafe { Pin::new_unchecked(&mut q) };
        q.as_mut().initialize();
        q.as_mut().swap(self.waitq().lock().as_mut());

        let mut cx = Cx::default();
        while let Some(task) = q.as_mut().pop_front() {
            unsafe {
                cx.schedule(task);
            }
        }
    }

    fn verify(&self, mutex_addr: usize) {
        match self.mutex.compare_and_swap(0, mutex_addr, Ordering::SeqCst) {
            // If we got out 0, then we have successfully bound the mutex to
            // this cvar.
            0 => {}

            // If we get out a value that's the same as `addr`, then someone
            // already beat us to the punch.
            n if n == mutex_addr => {}

            // Anything else and we're using more than one mutex on this cvar,
            // which is currently disallowed.
            _ => panic!(
                "attempted to use a condition variable with two \
                         mutexes"
            ),
        }
    }
}
