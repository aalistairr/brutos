use core::ops::{Deref, DerefMut};
use core::pin::Pin;
#[cfg(any(test, feature = "std"))]
use std::boxed::Box as StdBox;
#[cfg(any(test, feature = "std"))]
use std::rc::Rc as StdRc;
#[cfg(any(test, feature = "std"))]
use std::sync::Arc as StdArc;

#[cfg(feature = "brutos-alloc")]
use brutos_alloc::{
    AllocOne, Arc as BrutosArc, ArcInner as BrutosArcInner, Box as BrutosBox,
    Unique as BrutosUnique,
};

pub trait Raw: Copy {
    fn from_ptr<U>(ptr: *const U) -> Self;
    fn cast<U>(self) -> *const U;
}

impl<T> Raw for *const T {
    fn from_ptr<U>(ptr: *const U) -> *const T {
        ptr as _
    }

    fn cast<U>(self) -> *const U {
        self as _
    }
}

impl<T> Raw for *mut T {
    fn from_ptr<U>(ptr: *const U) -> *mut T {
        ptr as _
    }

    fn cast<U>(self) -> *const U {
        self as _
    }
}

pub unsafe trait Immovable: Sized + Deref {
    type Ptr: Pointer;

    unsafe fn from_pointer(ptr: Self::Ptr) -> Self;
    unsafe fn into_pointer(this: Self) -> Self::Ptr;
}

unsafe impl<'a, T> Immovable for &'a T {
    type Ptr = &'a T;

    unsafe fn from_pointer(ptr: &'a T) -> &'a T {
        ptr
    }

    unsafe fn into_pointer(this: &'a T) -> &'a T {
        this
    }
}

unsafe impl<'a, T: Pointer> Immovable for Pin<T> {
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
    type Immovable: Immovable<Ptr = Self, Target = Self::Target>;

    unsafe fn from_raw(ptr: Self::Raw) -> Self;
    fn into_raw(this: Self) -> Self::Raw;

    unsafe fn raw_deref<'a>(ptr: Self::Raw) -> &'a Self::Target;
}

pub unsafe trait PointerMut: Pointer + DerefMut {
    unsafe fn raw_deref_mut<'a>(ptr: Self::Raw) -> &'a mut Self::Target;
}

unsafe impl<'a, T> Pointer for &'a T {
    type Raw = *const T;
    type Immovable = &'a T;

    unsafe fn from_raw(ptr: *const T) -> &'a T {
        &*ptr
    }

    fn into_raw(this: &'a T) -> *const T {
        this
    }

    unsafe fn raw_deref<'b>(ptr: *const T) -> &'b T {
        &*ptr
    }
}

unsafe impl<'a, T> Pointer for &'a mut T {
    type Raw = *mut T;
    type Immovable = Pin<&'a mut T>;

    unsafe fn from_raw(ptr: *mut T) -> &'a mut T {
        &mut *ptr
    }

    fn into_raw(this: &'a mut T) -> *mut T {
        this
    }

    unsafe fn raw_deref<'b>(ptr: *mut T) -> &'b T {
        &*ptr
    }
}

unsafe impl<'a, T> PointerMut for &'a mut T {
    unsafe fn raw_deref_mut<'b>(ptr: *mut T) -> &'b mut T {
        &mut *ptr
    }
}

#[cfg(any(test, feature = "std"))]
unsafe impl<T> Pointer for StdBox<T> {
    type Raw = *mut T;
    type Immovable = Pin<StdBox<T>>;

    unsafe fn from_raw(raw: *mut T) -> StdBox<T> {
        StdBox::from_raw(raw)
    }

    fn into_raw(this: StdBox<T>) -> *mut T {
        StdBox::into_raw(this)
    }

    unsafe fn raw_deref<'a>(ptr: *mut T) -> &'a T {
        &*ptr
    }
}

#[cfg(any(test, feature = "std"))]
unsafe impl<T> PointerMut for StdBox<T> {
    unsafe fn raw_deref_mut<'a>(ptr: *mut T) -> &'a mut T {
        &mut *ptr
    }
}

#[cfg(any(test, feature = "std"))]
unsafe impl<T> Pointer for StdRc<T> {
    type Raw = *const T;
    type Immovable = Pin<StdRc<T>>;

    unsafe fn from_raw(ptr: *const T) -> StdRc<T> {
        StdRc::from_raw(ptr)
    }

    fn into_raw(this: StdRc<T>) -> *const T {
        StdRc::into_raw(this)
    }

    unsafe fn raw_deref<'a>(ptr: *const T) -> &'a T {
        &*ptr
    }
}

#[cfg(any(test, feature = "std"))]
unsafe impl<T> Pointer for StdArc<T> {
    type Raw = *const T;
    type Immovable = Pin<StdArc<T>>;

    unsafe fn from_raw(ptr: *const T) -> StdArc<T> {
        StdArc::from_raw(ptr)
    }

    fn into_raw(this: StdArc<T>) -> *const T {
        StdArc::into_raw(this)
    }

    unsafe fn raw_deref<'a>(ptr: *const T) -> &'a T {
        &*ptr
    }
}

#[cfg(feature = "brutos-alloc")]
unsafe impl<T, A: Default + AllocOne<T>> Pointer for BrutosBox<T, A> {
    type Raw = *mut T;
    type Immovable = Pin<BrutosBox<T, A>>;

    unsafe fn from_raw(raw: *mut T) -> BrutosBox<T, A> {
        BrutosBox::from_raw(raw)
    }

    fn into_raw(this: BrutosBox<T, A>) -> *mut T {
        BrutosBox::into_raw(this)
    }

    unsafe fn raw_deref<'a>(ptr: *mut T) -> &'a T {
        &*ptr
    }
}

#[cfg(feature = "brutos-alloc")]
unsafe impl<T, A: Default + AllocOne<T>> PointerMut for BrutosBox<T, A> {
    unsafe fn raw_deref_mut<'a>(ptr: *mut T) -> &'a mut T {
        &mut *ptr
    }
}

#[cfg(feature = "brutos-alloc")]
unsafe impl<T, A: Default + AllocOne<BrutosArcInner<T>>> Pointer for BrutosArc<T, A> {
    type Raw = *const T;
    type Immovable = Pin<BrutosArc<T, A>>;

    unsafe fn from_raw(ptr: *const T) -> BrutosArc<T, A> {
        BrutosArc::from_raw(ptr)
    }

    fn into_raw(this: BrutosArc<T, A>) -> *const T {
        BrutosArc::into_raw(this)
    }

    unsafe fn raw_deref<'a>(ptr: *const T) -> &'a T {
        &*ptr
    }
}

#[cfg(feature = "brutos-alloc")]
unsafe impl<T> Pointer for BrutosUnique<T> {
    type Raw = *mut T;
    type Immovable = Pin<BrutosUnique<T>>;

    unsafe fn from_raw(ptr: *mut T) -> BrutosUnique<T> {
        BrutosUnique::from_raw(ptr)
    }

    fn into_raw(this: BrutosUnique<T>) -> *mut T {
        BrutosUnique::into_raw(this)
    }

    unsafe fn raw_deref<'a>(ptr: *mut T) -> &'a T {
        &*ptr
    }
}

#[cfg(feature = "brutos-alloc")]
unsafe impl<T> PointerMut for BrutosUnique<T> {
    unsafe fn raw_deref_mut<'a>(ptr: *mut T) -> &'a mut T {
        &mut *ptr
    }
}
