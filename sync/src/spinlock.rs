use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::mem;
use core::pin::Pin;
use core::sync::atomic::{AtomicBool, Ordering};

use crate::Critical;

pub struct Spinlock<T: ?Sized, Cx> {
    is_locked: AtomicBool,
    _marker: PhantomData<Cx>,
    value: UnsafeCell<T>,
}

unsafe impl<T: ?Sized + Send, Cx> Send for Spinlock<T, Cx> {}
unsafe impl<T: ?Sized + Send, Cx> Sync for Spinlock<T, Cx> {}

impl<T, Cx> Spinlock<T, Cx> {
    pub const fn new(value: T) -> Spinlock<T, Cx> {
        Spinlock {
            is_locked: AtomicBool::new(false),
            _marker: PhantomData,
            value: UnsafeCell::new(value),
        }
    }
}

impl<T: Default, Cx> Default for Spinlock<T, Cx> {
    fn default() -> Spinlock<T, Cx> {
        Spinlock::new(T::default())
    }
}

impl<T: ?Sized, Cx: Critical> Spinlock<T, Cx> {
    pub fn try_lock(&self) -> Option<SpinlockGuard<T, Cx>> {
        unsafe {
            Cx::enter_critical();
        }
        if self
            .is_locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(SpinlockGuard { spinlock: self })
        } else {
            unsafe {
                Cx::leave_critical();
            }
            None
        }
    }

    pub fn lock(&self) -> SpinlockGuard<T, Cx> {
        loop {
            match self.try_lock() {
                Some(guard) => return guard,
                None => continue,
            }
        }
    }

    fn unlock(&self) {
        self.is_locked.store(false, Ordering::Release);
        unsafe {
            Cx::leave_critical();
        }
    }
}

pub struct SpinlockGuard<'a, T: ?Sized, Cx: Critical> {
    spinlock: &'a Spinlock<T, Cx>,
}

impl<'a, T: ?Sized, Cx: Critical> Drop for SpinlockGuard<'a, T, Cx> {
    fn drop(&mut self) {
        self.spinlock.unlock();
    }
}

impl<'a, T: ?Sized, Cx: Critical> core::ops::Deref for SpinlockGuard<'a, T, Cx> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.spinlock.value.get() }
    }
}

impl<'a, T: ?Sized, Cx: Critical> core::ops::DerefMut for SpinlockGuard<'a, T, Cx> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.spinlock.value.get() }
    }
}

impl<'a, T: ?Sized, Cx: Critical> SpinlockGuard<'a, T, Cx> {
    pub unsafe fn into_is_locked(this: SpinlockGuard<'a, T, Cx>) -> &'a AtomicBool {
        let is_locked = &this.spinlock.is_locked;
        mem::forget(this);
        is_locked
    }
}

pub struct PinSpinlock<T: ?Sized, Cx: Critical>(Spinlock<T, Cx>);

impl<T, Cx: Critical> PinSpinlock<T, Cx> {
    pub const fn new(value: T) -> PinSpinlock<T, Cx> {
        PinSpinlock(Spinlock::new(value))
    }
}

impl<T: ?Sized, Cx: Critical> PinSpinlock<T, Cx> {
    pub fn try_lock(self: Pin<&Self>) -> Option<Pin<SpinlockGuard<T, Cx>>> {
        let guard = self.get_ref().0.try_lock()?;
        unsafe { Some(Pin::new_unchecked(guard)) }
    }

    pub fn lock(self: Pin<&Self>) -> Pin<SpinlockGuard<T, Cx>> {
        let guard = self.get_ref().0.lock();
        unsafe { Pin::new_unchecked(guard) }
    }
}
