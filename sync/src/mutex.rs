use core::cell::UnsafeCell;
use core::pin::Pin;
use core::sync::atomic::{AtomicBool, Ordering};

use brutos_util::linked_list::LinkedList;

use crate::spinlock::{PinSpinlock, SpinlockGuard};
use crate::waitq;

pub struct Mutex<T, Cx: waitq::Context> {
    is_locked: AtomicBool,
    waitq: PinSpinlock<LinkedList<Cx::WaitQSel>, Cx>,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send, Cx: Send + waitq::Context> Send for Mutex<T, Cx> {}
unsafe impl<T: Send, Cx: Send + waitq::Context> Sync for Mutex<T, Cx> {}

impl<T, Cx: waitq::Context> Mutex<T, Cx> {
    pub const fn new(data: T) -> Mutex<T, Cx> {
        Mutex {
            is_locked: AtomicBool::new(false),
            waitq: PinSpinlock::new(LinkedList::new()),
            data: UnsafeCell::new(data),
        }
    }

    fn waitq(self: Pin<&Self>) -> Pin<&PinSpinlock<LinkedList<Cx::WaitQSel>, Cx>> {
        unsafe { self.map_unchecked(|x| &x.waitq) }
    }

    pub fn initialize(self: Pin<&Self>) {
        self.waitq().lock().as_mut().initialize();
    }

    pub fn try_lock(self: Pin<&Self>) -> Option<MutexGuard<T, Cx>> {
        match self
            .is_locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        {
            Ok(_) => Some(MutexGuard { mutex: self }),
            Err(_) => None,
        }
    }

    pub fn lock(self: Pin<&Self>) -> MutexGuard<T, Cx> {
        unsafe {
            if let Some(guard) = self.try_lock() {
                return guard;
            }
            let mut waitq = self.waitq().lock();
            if let Some(guard) = self.try_lock() {
                return guard;
            }
            waitq.as_mut().push_back(Cx::default().deschedule());
            let is_locked = SpinlockGuard::into_is_locked(Pin::into_inner_unchecked(waitq));
            Cx::unlock_and_yield(is_locked);
            MutexGuard { mutex: self }
        }
    }

    fn unlock(self: Pin<&Self>) {
        let mut waitq = self.waitq().lock();
        match waitq.as_mut().pop_front() {
            Some(next_task) => unsafe { Cx::default().schedule(next_task) },
            None => self.is_locked.store(false, Ordering::Release),
        }
    }

    pub fn into_inner(this: Self) -> T {
        UnsafeCell::into_inner(this.data)
    }
}

pub struct MutexGuard<'a, T, Cx: waitq::Context> {
    pub(crate) mutex: Pin<&'a Mutex<T, Cx>>,
}

impl<'a, T, Cx: waitq::Context> Drop for MutexGuard<'a, T, Cx> {
    fn drop(&mut self) {
        self.mutex.unlock();
    }
}

impl<'a, T, Cx: waitq::Context> core::ops::Deref for MutexGuard<'a, T, Cx> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, T, Cx: waitq::Context> core::ops::DerefMut for MutexGuard<'a, T, Cx> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

pub struct PinMutex<T, Cx: waitq::Context>(Mutex<T, Cx>);

impl<T, Cx: waitq::Context> PinMutex<T, Cx> {
    pub const fn new(data: T) -> PinMutex<T, Cx> {
        PinMutex(Mutex::new(data))
    }

    fn mutex(self: Pin<&Self>) -> Pin<&Mutex<T, Cx>> {
        unsafe { self.map_unchecked(|x| &x.0) }
    }

    pub fn initialize(self: Pin<&Self>) {
        self.mutex().initialize();
    }

    pub fn try_lock(self: Pin<&Self>) -> Option<Pin<MutexGuard<T, Cx>>> {
        self.mutex()
            .try_lock()
            .map(|g| unsafe { Pin::new_unchecked(g) })
    }

    pub fn lock(self: Pin<&Self>) -> Pin<MutexGuard<T, Cx>> {
        unsafe { Pin::new_unchecked(self.mutex().lock()) }
    }
}
