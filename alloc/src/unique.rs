use core::marker::PhantomData;
use core::mem;
use core::ptr::NonNull;

pub struct Unique<T> {
    ptr: NonNull<T>,
    _marker: PhantomData<T>,
}

impl<T> Drop for Unique<T> {
    fn drop(&mut self) {
        panic!("Drop called on Unique");
    }
}

impl<T> Unique<T> {
    pub unsafe fn from_raw(ptr: *mut T) -> Unique<T> {
        Unique {
            ptr: NonNull::new_unchecked(ptr),
            _marker: PhantomData,
        }
    }

    pub fn into_raw(this: Unique<T>) -> *mut T {
        let ptr = this.ptr.as_ptr();
        mem::forget(this);
        ptr
    }

    pub fn as_ptr_non_null(&self) -> NonNull<T> {
        self.ptr
    }
}

impl<T> core::ops::Deref for Unique<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> core::ops::DerefMut for Unique<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() }
    }
}
