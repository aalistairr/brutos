use core::fmt;
use core::marker::PhantomData;
use core::mem::{self, MaybeUninit};
use core::pin::Pin;
use core::ptr::{self, NonNull};

use crate::{AllocOne, Layout, OutOfMemory, TransparentAlloc};

#[fundamental]
pub struct Box<T, A: Default + AllocOne<T>> {
    ptr: NonNull<T>,
    _marker: PhantomData<(T, *const A)>,
}

impl<T, A: Default + AllocOne<T>> core::ops::Deref for Box<T, A> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T, A: Default + AllocOne<T>> core::ops::DerefMut for Box<T, A> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T, A: Default + AllocOne<T>> Drop for Box<T, A> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(self.ptr.as_ptr());
            A::default().dealloc(self.ptr);
        }
    }
}

impl<T, A: Default + AllocOne<T>> Box<T, A> {
    pub fn new_uninit() -> Result<Box<MaybeUninit<T>, TransparentAlloc<T, A>>, OutOfMemory> {
        let layout = Layout::new::<MaybeUninit<T>>();
        if layout.size() == 0 {
            return Ok(Box {
                ptr: NonNull::dangling(),
                _marker: PhantomData,
            });
        }
        Ok(Box {
            ptr: unsafe { A::default().alloc()?.cast() },
            _marker: PhantomData,
        })
    }

    pub fn new_zeroed() -> Result<Box<MaybeUninit<T>, TransparentAlloc<T, A>>, OutOfMemory> {
        unsafe {
            let mut uninit = Self::new_uninit()?;
            ptr::write_bytes(uninit.as_mut_ptr(), 0u8, 1usize);
            Ok(uninit)
        }
    }

    pub fn new(x: T) -> Result<Box<T, A>, (OutOfMemory, T)> {
        unsafe {
            let mut boxed = match Self::new_uninit() {
                Ok(boxed) => boxed,
                Err(e) => return Err((e, x)),
            };
            ptr::write(boxed.as_mut_ptr(), x);
            Ok(boxed.assume_init())
        }
    }

    pub fn pin(x: T) -> Result<Pin<Box<T, A>>, (OutOfMemory, T)> {
        Self::new(x).map(Into::into)
    }

    pub unsafe fn from_raw(ptr: *mut T) -> Box<T, A> {
        Box {
            ptr: NonNull::new_unchecked(ptr),
            _marker: PhantomData,
        }
    }

    pub fn into_raw_non_null(this: Box<T, A>) -> NonNull<T> {
        let ptr = this.ptr;
        mem::forget(this);
        ptr
    }

    pub fn into_raw(this: Box<T, A>) -> *mut T {
        Box::into_raw_non_null(this).as_ptr()
    }

    pub fn into_pin(this: Box<T, A>) -> Pin<Box<T, A>> {
        unsafe { Pin::new_unchecked(this) }
    }
}

impl<T, A: Default + AllocOne<T>> Box<MaybeUninit<T>, TransparentAlloc<T, A>> {
    pub unsafe fn assume_init(self) -> Box<T, A> {
        let ptr = Box::into_raw_non_null(self).cast();
        Box {
            ptr,
            _marker: PhantomData,
        }
    }
}

impl<T: Clone, A: Default + AllocOne<T>> Box<T, A> {
    pub fn try_clone(&self) -> Result<Box<T, A>, OutOfMemory> {
        unsafe {
            let mut boxed = Box::new_uninit()?;
            ptr::write(boxed.as_mut_ptr(), (**self).clone());
            Ok(boxed.assume_init())
        }
    }
}

impl<T, A: Default + AllocOne<T>> From<Box<T, A>> for Pin<Box<T, A>> {
    fn from(boxed: Box<T, A>) -> Pin<Box<T, A>> {
        Box::into_pin(boxed)
    }
}

impl<T: PartialEq, A: Default + AllocOne<T>> PartialEq for Box<T, A> {
    #[inline]
    fn eq(&self, other: &Box<T, A>) -> bool {
        PartialEq::eq(&**self, &**other)
    }
    #[inline]
    fn ne(&self, other: &Box<T, A>) -> bool {
        PartialEq::ne(&**self, &**other)
    }
}
impl<T: PartialOrd, A: Default + AllocOne<T>> PartialOrd for Box<T, A> {
    #[inline]
    fn partial_cmp(&self, other: &Box<T, A>) -> Option<core::cmp::Ordering> {
        PartialOrd::partial_cmp(&**self, &**other)
    }
    #[inline]
    fn lt(&self, other: &Box<T, A>) -> bool {
        PartialOrd::lt(&**self, &**other)
    }
    #[inline]
    fn le(&self, other: &Box<T, A>) -> bool {
        PartialOrd::le(&**self, &**other)
    }
    #[inline]
    fn ge(&self, other: &Box<T, A>) -> bool {
        PartialOrd::ge(&**self, &**other)
    }
    #[inline]
    fn gt(&self, other: &Box<T, A>) -> bool {
        PartialOrd::gt(&**self, &**other)
    }
}
impl<T: Ord, A: Default + AllocOne<T>> Ord for Box<T, A> {
    #[inline]
    fn cmp(&self, other: &Box<T, A>) -> core::cmp::Ordering {
        Ord::cmp(&**self, &**other)
    }
}
impl<T: Eq, A: Default + AllocOne<T>> Eq for Box<T, A> {}

impl<T: core::hash::Hash, A: Default + AllocOne<T>> core::hash::Hash for Box<T, A> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}

impl<T: core::hash::Hasher, A: Default + AllocOne<T>> core::hash::Hasher for Box<T, A> {
    fn finish(&self) -> u64 {
        (**self).finish()
    }
    fn write(&mut self, bytes: &[u8]) {
        (**self).write(bytes)
    }
    fn write_u8(&mut self, i: u8) {
        (**self).write_u8(i)
    }
    fn write_u16(&mut self, i: u16) {
        (**self).write_u16(i)
    }
    fn write_u32(&mut self, i: u32) {
        (**self).write_u32(i)
    }
    fn write_u64(&mut self, i: u64) {
        (**self).write_u64(i)
    }
    fn write_u128(&mut self, i: u128) {
        (**self).write_u128(i)
    }
    fn write_usize(&mut self, i: usize) {
        (**self).write_usize(i)
    }
    fn write_i8(&mut self, i: i8) {
        (**self).write_i8(i)
    }
    fn write_i16(&mut self, i: i16) {
        (**self).write_i16(i)
    }
    fn write_i32(&mut self, i: i32) {
        (**self).write_i32(i)
    }
    fn write_i64(&mut self, i: i64) {
        (**self).write_i64(i)
    }
    fn write_i128(&mut self, i: i128) {
        (**self).write_i128(i)
    }
    fn write_isize(&mut self, i: isize) {
        (**self).write_isize(i)
    }
}

impl<T: fmt::Display, A: Default + AllocOne<T>> fmt::Display for Box<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<T: fmt::Debug, A: Default + AllocOne<T>> fmt::Debug for Box<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T, A: Default + AllocOne<T>> fmt::Pointer for Box<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // It's not possible to extract the inner Uniq directly from the Box,
        // instead we cast it to a *const which aliases the Unique
        let ptr: *const T = &**self;
        fmt::Pointer::fmt(&ptr, f)
    }
}

impl<I: Iterator, A: Default + AllocOne<I>> Iterator for Box<I, A> {
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        (**self).next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (**self).size_hint()
    }
    fn nth(&mut self, n: usize) -> Option<I::Item> {
        (**self).nth(n)
    }
    fn last(self) -> Option<I::Item> {
        #[inline]
        fn some<T>(_: Option<T>, x: T) -> Option<T> {
            Some(x)
        }

        self.fold(None, some)
    }
}

impl<I: DoubleEndedIterator, A: Default + AllocOne<I>> DoubleEndedIterator for Box<I, A> {
    fn next_back(&mut self) -> Option<I::Item> {
        (**self).next_back()
    }
    fn nth_back(&mut self, n: usize) -> Option<I::Item> {
        (**self).nth_back(n)
    }
}
impl<I: ExactSizeIterator, A: Default + AllocOne<I>> ExactSizeIterator for Box<I, A> {
    fn len(&self) -> usize {
        (**self).len()
    }
    fn is_empty(&self) -> bool {
        (**self).is_empty()
    }
}

impl<I: core::iter::FusedIterator, A: Default + AllocOne<I>> core::iter::FusedIterator
    for Box<I, A>
{
}

impl<T, A: Default + AllocOne<T>> core::borrow::Borrow<T> for Box<T, A> {
    fn borrow(&self) -> &T {
        &**self
    }
}

impl<T, A: Default + AllocOne<T>> core::borrow::BorrowMut<T> for Box<T, A> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut **self
    }
}

impl<T, A: Default + AllocOne<T>> AsRef<T> for Box<T, A> {
    fn as_ref(&self) -> &T {
        &**self
    }
}

impl<T, A: Default + AllocOne<T>> AsMut<T> for Box<T, A> {
    fn as_mut(&mut self) -> &mut T {
        &mut **self
    }
}

impl<T, A: Default + AllocOne<T>> core::ops::Receiver for Box<T, A> {}

impl<T, A: Default + AllocOne<T>> Unpin for Box<T, A> {}
