use core::ops::Deref;
use core::pin::Pin;
#[cfg(any(test, feature = "std"))]
use std::rc::Rc;
#[cfg(any(test, feature = "std"))]
use std::sync::Arc;

pub trait Raw {
    fn raw_from<U>(ptr: *const U) -> Self;
    fn raw_from_mut<U>(ptr: *mut U) -> Self;
    fn cast_to<U>(self) -> *const U;
    fn cast_to_mut<U>(self) -> *mut U;
}

impl<T> Raw for *const T {
    fn raw_from<U>(ptr: *const U) -> *const T {
        ptr as *const T
    }

    fn raw_from_mut<U>(ptr: *mut U) -> *const T {
        ptr as *const T
    }

    fn cast_to<U>(self) -> *const U {
        self as *const U
    }

    fn cast_to_mut<U>(self) -> *mut U {
        self as *mut U
    }
}

impl<T> Raw for *mut T {
    fn raw_from<U>(ptr: *const U) -> *mut T {
        ptr as *mut T
    }

    fn raw_from_mut<U>(ptr: *mut U) -> *mut T {
        ptr as *mut T
    }

    fn cast_to<U>(self) -> *const U {
        self as *const U
    }

    fn cast_to_mut<U>(self) -> *mut U {
        self as *mut U
    }
}

pub unsafe trait NonMovable: Deref {
    type Ptr: Pointer;

    unsafe fn from_pointer(ptr: Self::Ptr) -> Self;
    unsafe fn into_pointer(this: Self) -> Self::Ptr;
}

pub unsafe trait NonMovableMut: NonMovable {
    fn as_mut(&mut self) -> Pin<&mut Self::Target>;
}

unsafe impl<'a, T> NonMovable for &'a T {
    type Ptr = &'a T;

    unsafe fn from_pointer(ptr: &'a T) -> &'a T {
        ptr
    }

    unsafe fn into_pointer(this: &'a T) -> &'a T {
        this
    }
}

unsafe impl<T: Pointer> NonMovable for Pin<T> {
    type Ptr = T;

    unsafe fn from_pointer(ptr: T) -> Pin<T> {
        Pin::new_unchecked(ptr)
    }

    unsafe fn into_pointer(this: Pin<T>) -> T {
        Pin::into_inner_unchecked(this)
    }
}

pub unsafe trait Pointer: Sized + Deref {
    type Raw: Raw;
    type NonMovable: NonMovable<Ptr = Self, Target = Self::Target>;

    unsafe fn from_raw(ptr: Self::Raw) -> Self;
    fn into_raw(value: Self) -> Self::Raw;
}

unsafe impl<'a, T: Sized> Pointer for &'a T {
    type Raw = *const T;
    type NonMovable = &'a T;

    unsafe fn from_raw(ptr: *const T) -> &'a T {
        &*ptr
    }

    fn into_raw(this: &'a T) -> *const T {
        this
    }
}

unsafe impl<'a, T: Sized> Pointer for &'a mut T {
    type Raw = *mut T;
    type NonMovable = Pin<&'a mut T>;

    unsafe fn from_raw(ptr: *mut T) -> &'a mut T {
        &mut *ptr
    }

    fn into_raw(this: &'a mut T) -> *mut T {
        this
    }
}

#[cfg(any(test, feature = "std"))]
unsafe impl<T: Sized> Pointer for Box<T> {
    type Raw = *mut T;
    type NonMovable = Pin<Box<T>>;

    unsafe fn from_raw(ptr: *mut T) -> Box<T> {
        Box::from_raw(ptr)
    }

    fn into_raw(this: Box<T>) -> *mut T {
        Box::into_raw(this)
    }
}

#[cfg(any(test, feature = "std"))]
unsafe impl<T: Sized> Pointer for Rc<T> {
    type Raw = *const T;
    type NonMovable = Pin<Rc<T>>;

    unsafe fn from_raw(ptr: *const T) -> Rc<T> {
        Rc::from_raw(ptr)
    }

    fn into_raw(this: Rc<T>) -> *const T {
        Rc::into_raw(this)
    }
}

#[cfg(any(test, feature = "std"))]
unsafe impl<T: Sized> Pointer for Arc<T> {
    type Raw = *const T;
    type NonMovable = Pin<Arc<T>>;

    unsafe fn from_raw(ptr: *const T) -> Arc<T> {
        Arc::from_raw(ptr)
    }

    fn into_raw(this: Arc<T>) -> *const T {
        Arc::into_raw(this)
    }
}
