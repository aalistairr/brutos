use core::cell::UnsafeCell;
use core::fmt;
use core::marker::{PhantomData, PhantomPinned};
use core::mem::ManuallyDrop;
use core::ops::{Deref, DerefMut};
use core::pin::Pin;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicBool, Ordering};

use crate::pointer::{NonMovable, Pointer, Raw};

/// Interaction between a [`LinkedList`](LinkedList) and its elements.
///
/// Use the [`selector!`](selector!) macro to generate a type that implements `Selector`.
pub unsafe trait Selector: Sized {
    type Ptr: Pointer<Raw = Self::Raw, NonMovable = Self::Pinned, Target = Self::Target>;
    type Raw: Raw;
    type Pinned: NonMovable<Ptr = Self::Ptr, Target = <Self::Ptr as Deref>::Target>;
    type Target;

    fn node_offset() -> usize;
}

/// Generates a type that implements `Selector` for use with [`LinkedList`](crate::linked_list::LinkedList).
///
/// # Examples
/// ```ignore
/// use brutos_util::linked_list::Node;
///
/// pub struct Foo {
///     node: Node<FooSel>,
/// }
/// brutos_util::selector!(pub FooSel: Box<Foo> => node);
/// ```
///
/// References can also be used:
/// ```ignore
/// use brutos_util::linked_list::Node;
///
/// struct Bar<'a> {
///     node: Node<BarSel<'a>>,
/// }
/// brutos_util::selector!(BarSel<'a>: &'a Bar<'a> => node);
/// ```
#[macro_export]
macro_rules! selector {
    ($($x:tt)*) => {
        $crate::selector_!(@parse0 # $($x)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! selector_ {
    (@parse0 # pub $($rest:tt)*) => {
        $crate::selector_!(@parse1 ((pub)) # $($rest)*);
    };
    (@parse0 # $($rest:tt)*) => {
        $crate::selector_!(@parse1 (()) # $($rest)*);
    };

    (@parse1 ($($parsed:tt)*) # $selector_name:ident : $($rest:tt)*) => {
        $crate::selector_!(@parse2 ($($parsed)* $selector_name ()) # $($rest)*);
    };
    (@parse1 ($($parsed:tt)*) # $selector_name:ident <$selector_lt:lifetime> : $($rest:tt)*) => {
        $crate::selector_!(@parse2 ($($parsed)* $selector_name ($selector_lt)) # $($rest)*);
    };

    (@parse2 $sel:tt # $pointer_name:ident < $($rest:tt)*) => {
        $crate::selector_!(@parse3 $sel (struct $pointer_name) # $($rest)*);
    };
    (@parse2 $sel:tt # & $lt:lifetime mut $($rest:tt)*) => {
        $crate::selector_!(@parse3 $sel (& ($lt mut)) # $($rest)*);
    };
    (@parse2 $sel:tt # & $lt:lifetime $($rest:tt)*) => {
        $crate::selector_!(@parse3 $sel (& ($lt)) # $($rest)*);
    };

    // parse3, parse4: struct with a lifetime parameter (because pesky >> is a token)
    (@parse3 $sel:tt ($pointer_kind:tt $pointer_extra:tt) # $struct_name:ident < $lt:lifetime $($rest:tt)*) => {
        $crate::selector_!(@parse4 $sel ($pointer_kind $pointer_extra $struct_name ($lt)) # $($rest)*);
    };
    (@parse4 $sel:tt (struct $pointer_name:ident $struct_name:ident ($($struct_lt:tt)+)) # >> => $node_field:ident) => {
        $crate::selector_!(@emit $sel $pointer_name<$struct_name<$($struct_lt)*>>, $struct_name.$node_field);
    };
    (@parse4 $sel:tt (& ($($pointer_info:tt)*) $struct_name:ident ($($struct_lt:tt)+)) # > => $node_field:ident) => {
        $crate::selector_!(@emit $sel &$($pointer_info)* $struct_name<$($struct_lt)*>, $struct_name.$node_field);
    };

    // parse3, parse4: struct without a lifetime parameter
    (@parse3 $sel:tt ($pointer_kind:tt $pointer_extra:tt) # $struct_name:ident $($rest:tt)*) => {
        $crate::selector_!(@parse4 $sel ($pointer_kind $pointer_extra $struct_name ()) # $($rest)*);
    };
    (@parse4 $sel:tt (struct $pointer_name:ident $struct_name:ident ($($struct_lt:tt)*)) # > => $node_field:ident) => {
        $crate::selector_!(@emit $sel $pointer_name<$struct_name<$($struct_lt)*>>, $struct_name.$node_field);
    };
    (@parse4 $sel:tt (& ($($pointer_info:tt)*) $struct_name:ident ($($struct_lt:tt)*)) # => $node_field:ident) => {
        $crate::selector_!(@emit $sel &$($pointer_info)* $struct_name<$($struct_lt)*>, $struct_name.$node_field);
    };

    (@emit (($($privacy:tt)*) $selector_name:ident ($($selector_lt:lifetime)*)) $pointer_ty:ty, $struct_name:ident . $node_field:ident) => {
        #[allow(dead_code)]
        $($privacy)* struct $selector_name<$($selector_lt)?>(core::marker::PhantomData<(!, $pointer_ty)>);

        unsafe impl<$($selector_lt)?> $crate::linked_list::Selector for $selector_name<$($selector_lt)?> {
            type Ptr = $pointer_ty;
            type Raw = <$pointer_ty as $crate::pointer::Pointer>::Raw;
            type Pinned = <$pointer_ty as $crate::pointer::Pointer>::NonMovable;
            type Target = <$pointer_ty as core::ops::Deref>::Target;

            fn node_offset() -> usize {
                $crate::offset_of!($struct_name, $node_field)
            }
        }
    };
}

pub struct Node<S> {
    link: UnsafeCell<Link>,
    is_active: AtomicBool,
    _marker: PhantomData<S>,
}

unsafe impl<S> Send for Node<S> {}
unsafe impl<S> Sync for Node<S> {}

impl<S> Node<S> {
    pub const fn new() -> Node<S> {
        Node {
            link: UnsafeCell::new(Link::new()),
            is_active: AtomicBool::new(false),
            _marker: PhantomData,
        }
    }
}

impl<S> Default for Node<S> {
    fn default() -> Node<S> {
        Node::new()
    }
}

impl<S> fmt::Debug for Node<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node(<opaque>)")
    }
}

struct Link {
    prev: Option<NonNull<Self>>,
    next: Option<NonNull<Self>>,
    _pin: PhantomPinned,
}

const LINK_IS_NULL: &str = "Link is null";

impl Link {
    const fn new() -> Link {
        Link {
            prev: None,
            next: None,
            _pin: PhantomPinned,
        }
    }

    fn prev_nonnull(&self) -> NonNull<Link> {
        self.prev.expect(LINK_IS_NULL)
    }

    fn next_nonnull(&self) -> NonNull<Link> {
        self.next.expect(LINK_IS_NULL)
    }
}

impl<S: Selector> Node<S> {
    unsafe fn link_mut(&self) -> &mut Link {
        &mut *self.link.get()
    }

    unsafe fn insert_before(&self, after: NonNull<Link>) {
        let before = after.as_ref().prev_nonnull();
        self.insert_between(before, after);
    }

    unsafe fn insert_after(&self, before: NonNull<Link>) {
        let after = before.as_ref().next_nonnull();
        self.insert_between(before, after);
    }

    unsafe fn insert_between(&self, mut before: NonNull<Link>, mut after: NonNull<Link>) {
        self.is_active
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
            .expect("Value is already contained in a list");

        self.link_mut().prev = Some(before);
        self.link_mut().next = Some(after);
        before.as_mut().next = Some(self.link_mut().into());
        after.as_mut().prev = Some(self.link_mut().into());
    }

    unsafe fn unlink(&self) {
        let mut before = self.link_mut().prev_nonnull();
        let mut after = self.link_mut().next_nonnull();
        before.as_mut().next = Some(after);
        after.as_mut().prev = Some(before);

        self.is_active
            .compare_exchange(true, false, Ordering::AcqRel, Ordering::Relaxed)
            .expect("Value is not contained in a list");
    }
}

fn value_into_node_ptr<S: Selector>(value: S::Pinned) -> *const Node<S> {
    unsafe {
        let value_ptr = <_>::into_pointer(value);
        let value_ptr = <_>::into_raw(value_ptr);
        value_ptr_into_node_ptr(value_ptr)
    }
}

unsafe fn value_ptr_into_node_ptr<S: Selector>(value_ptr: S::Raw) -> *const Node<S> {
    value_ptr.cast_to::<u8>().add(S::node_offset()) as *const Node<S>
}

unsafe fn value_from_node_ptr<S: Selector>(node_ptr: *const Node<S>) -> S::Pinned {
    let value_ptr = node_ptr.cast_to::<u8>().sub(S::node_offset()) as *const S::Raw;
    let value_ptr = S::Ptr::from_raw(Raw::raw_from(value_ptr));
    <_>::from_pointer(value_ptr)
}

unsafe fn node_ptr_from_link_ptr<S: Selector>(link_ptr: NonNull<Link>) -> *const Node<S> {
    (link_ptr.as_ptr() as *mut u8).sub(crate::offset_of!(@ Node<S>, link)) as *const Node<S>
}

unsafe fn node_ptr_into_link_ptr<S: Selector>(node_ptr: *const Node<S>) -> NonNull<Link> {
    (&mut *(&*node_ptr).link.get()).into()
}

/// An intrusive linked list type.
///
/// Use the [`selector!`](selector!) macro to generate a type that implements
/// the `Selector` trait which specifies how elements are accessed.
///
/// # Examples
/// ```ignore
/// use std::pin::Pin;
/// use brutos_util::linked_list::{LinkedList, Node};
///
/// struct Foo {
///     n: u32,
///     node: Node<FooSel>,
/// }
/// brutos_util::selector!(FooSel: Box<Foo> => node);
///
/// impl Foo {
///     fn new(n: u32) -> Pin<Box<Foo>> {
///         Box::pin(Foo { n, node: Node::new() })
///     }
/// }
///
/// let mut list = Box::pin(LinkedList::<FooSel>::new());
/// list.as_mut().initialize();
///
/// list.as_mut().push_back(Foo::new(0));
/// list.as_mut().push_back(Foo::new(1));
///
/// for foo in &*list {
///     println!("{}", foo.n);
/// }
/// ```
pub struct LinkedList<S: Selector> {
    anchor: UnsafeCell<Link>,
    _marker: PhantomData<S>,
}

unsafe impl<S: Selector> Send for LinkedList<S> {}
unsafe impl<S: Selector> Sync for LinkedList<S> {}

impl<S: Selector> LinkedList<S> {
    pub const fn new() -> LinkedList<S> {
        LinkedList {
            anchor: UnsafeCell::new(Link::new()),
            _marker: PhantomData,
        }
    }

    pub fn initialize(self: Pin<&mut Self>) {
        let mut anchor = unsafe { &mut *self.anchor.get() };
        assert!(anchor.prev.is_none());
        debug_assert!(anchor.next.is_none());
        anchor.prev = Some(anchor.into());
        anchor.next = Some(anchor.into());
    }
}

#[derive(Copy, Clone)]
struct CursorInner {
    anchor: NonNull<Link>,
    link: NonNull<Link>,
}

pub struct Cursor<'a, S: Selector> {
    inner: CursorInner,
    _marker: PhantomData<&'a LinkedList<S>>,
}

pub struct CursorMut<'a, S: Selector> {
    inner: CursorInner,
    _marker: PhantomData<&'a mut LinkedList<S>>,
}

impl CursorInner {
    fn try_move(&self, to: NonNull<Link>) -> Result<Self, Self> {
        if to != self.anchor {
            Ok(CursorInner {
                anchor: self.anchor,
                link: to,
            })
        } else {
            Err(*self)
        }
    }

    fn prev(&self) -> Result<Self, Self> {
        self.try_move(unsafe { self.link.as_ref().prev_nonnull() })
    }

    fn next(&self) -> Result<Self, Self> {
        self.try_move(unsafe { self.link.as_ref().next_nonnull() })
    }

    fn get<S: Selector>(&self) -> &S::Target {
        unsafe {
            let node_ptr = node_ptr_from_link_ptr::<S>(self.link);
            let value = ManuallyDrop::new(value_from_node_ptr(node_ptr));
            let target_ptr = &**value as *const S::Target;
            &*target_ptr
        }
    }

    fn get_mut<S: Selector>(&mut self) -> Pin<&mut S::Target>
    where
        S::Ptr: DerefMut,
    {
        unsafe {
            let node_ptr = node_ptr_from_link_ptr::<S>(self.link);
            let mut value = ManuallyDrop::new(<_>::into_pointer(value_from_node_ptr(node_ptr)));
            Pin::new_unchecked(&mut *(&mut **value as *mut _))
        }
    }
}

impl<'a, S: Selector> Copy for Cursor<'a, S> {}
impl<'a, S: Selector> Clone for Cursor<'a, S> {
    fn clone(&self) -> Self {
        Cursor {
            inner: self.inner.clone(),
            _marker: PhantomData,
        }
    }
}

impl<'a, S: Selector> Deref for Cursor<'a, S> {
    type Target = S::Target;

    fn deref(&self) -> &Self::Target {
        self.inner.get::<S>()
    }
}

impl<'a, S: Selector> Cursor<'a, S> {
    fn from_inner(inner: CursorInner) -> Self {
        Cursor {
            inner,
            _marker: PhantomData,
        }
    }
    pub fn prev(&self) -> Result<Self, Self> {
        self.inner
            .prev()
            .map(Cursor::from_inner)
            .map_err(Cursor::from_inner)
    }
    pub fn next(&self) -> Result<Self, Self> {
        self.inner
            .next()
            .map(Cursor::from_inner)
            .map_err(Cursor::from_inner)
    }
}

impl<'a, S: Selector> Deref for CursorMut<'a, S> {
    type Target = S::Target;

    fn deref(&self) -> &Self::Target {
        self.inner.get::<S>()
    }
}

impl<'a, S: Selector> CursorMut<'a, S>
where
    S::Ptr: DerefMut,
{
    pub fn get_mut(&mut self) -> Pin<&mut S::Target> {
        self.inner.get_mut::<S>()
    }
}

impl<'a, S: Selector> CursorMut<'a, S> {
    fn from_inner(inner: CursorInner) -> Self {
        CursorMut {
            inner,
            _marker: PhantomData,
        }
    }
    pub fn prev(self) -> Result<Self, Self> {
        self.inner
            .prev()
            .map(CursorMut::from_inner)
            .map_err(CursorMut::from_inner)
    }
    pub fn next(self) -> Result<Self, Self> {
        self.inner
            .next()
            .map(CursorMut::from_inner)
            .map_err(CursorMut::from_inner)
    }

    pub fn unlink(self) -> S::Pinned {
        unsafe {
            let node_ptr = node_ptr_from_link_ptr::<S>(self.inner.link);
            (*node_ptr).unlink();
            value_from_node_ptr(node_ptr)
        }
    }

    pub fn insert_before(&mut self, value: S::Pinned) {
        unsafe {
            (*value_into_node_ptr::<S>(value)).insert_before(self.inner.link);
        }
    }

    pub fn insert_after(&mut self, value: S::Pinned) {
        unsafe {
            (*value_into_node_ptr::<S>(value)).insert_after(self.inner.link);
        }
    }
}

impl<S: Selector> LinkedList<S> {
    fn anchor_nonnull(&self) -> NonNull<Link> {
        unsafe { (&mut *self.anchor.get()).into() }
    }

    pub unsafe fn cursor_from_raw(&self, ptr: S::Raw) -> Cursor<S> {
        Cursor::from_inner(CursorInner {
            anchor: self.anchor_nonnull(),
            link: node_ptr_into_link_ptr::<S>(value_ptr_into_node_ptr(ptr)),
        })
    }

    pub unsafe fn cursor_from_raw_mut(self: Pin<&mut Self>, ptr: S::Raw) -> CursorMut<S> {
        CursorMut::from_inner(CursorInner {
            anchor: self.anchor_nonnull(),
            link: node_ptr_into_link_ptr::<S>(value_ptr_into_node_ptr(ptr)),
        })
    }

    fn first_link(&self) -> Option<NonNull<Link>> {
        let anchor = self.anchor_nonnull();
        let first = unsafe { anchor.as_ref().next_nonnull() };
        if first != anchor {
            Some(first)
        } else {
            None
        }
    }

    fn last_link(&self) -> Option<NonNull<Link>> {
        let anchor = self.anchor_nonnull();
        let last = unsafe { anchor.as_ref().prev_nonnull() };
        if last != anchor {
            Some(last)
        } else {
            None
        }
    }

    pub fn first(&self) -> Option<Cursor<S>> {
        self.first_link().map(|link| {
            Cursor::from_inner(CursorInner {
                anchor: self.anchor_nonnull(),
                link,
            })
        })
    }

    pub fn first_mut(self: Pin<&mut Self>) -> Option<CursorMut<S>> {
        self.first_link().map(|link| {
            CursorMut::from_inner(CursorInner {
                anchor: self.anchor_nonnull(),
                link,
            })
        })
    }

    pub fn last(&self) -> Option<Cursor<S>> {
        self.last_link().map(|link| {
            Cursor::from_inner(CursorInner {
                anchor: self.anchor_nonnull(),
                link,
            })
        })
    }

    pub fn last_mut(self: Pin<&mut Self>) -> Option<CursorMut<S>> {
        self.last_link().map(|link| {
            CursorMut::from_inner(CursorInner {
                anchor: self.anchor_nonnull(),
                link,
            })
        })
    }

    pub fn push_front(self: Pin<&mut Self>, value: S::Pinned) {
        unsafe {
            (*value_into_node_ptr::<S>(value)).insert_after(self.anchor_nonnull());
        }
    }

    pub fn push_back(self: Pin<&mut Self>, value: S::Pinned) {
        unsafe {
            (*value_into_node_ptr::<S>(value)).insert_before(self.anchor_nonnull());
        }
    }

    pub fn pop_front(self: Pin<&mut Self>) -> Option<S::Pinned> {
        self.first_mut().map(CursorMut::unlink)
    }

    pub fn pop_back(self: Pin<&mut Self>) -> Option<S::Pinned> {
        self.last_mut().map(CursorMut::unlink)
    }
}

pub struct Iter<'a, S: Selector>(Option<Cursor<'a, S>>);

impl<'a, S: Selector> Iterator for Iter<'a, S> {
    type Item = &'a S::Target;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|cursor| {
            let item_ptr = &*cursor as *const _;
            self.0 = cursor.next().ok();
            unsafe { &*item_ptr }
        })
    }
}

pub struct IterMut<'a, S: Selector>(Option<CursorMut<'a, S>>);

impl<'a, S: Selector> Iterator for IterMut<'a, S>
where
    S::Ptr: DerefMut,
{
    type Item = Pin<&'a mut S::Target>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|mut cursor| {
            let item_ptr = unsafe { cursor.get_mut().get_unchecked_mut() as *mut _ };
            self.0 = cursor.next().ok();
            unsafe { Pin::new_unchecked(&mut *item_ptr) }
        })
    }
}

impl<'a, S: Selector> IntoIterator for &'a LinkedList<S> {
    type IntoIter = Iter<'a, S>;
    type Item = &'a S::Target;

    fn into_iter(self) -> Iter<'a, S> {
        Iter(self.first())
    }
}

impl<'a, S: Selector> IntoIterator for Pin<&'a mut LinkedList<S>>
where
    S::Ptr: DerefMut,
{
    type IntoIter = IterMut<'a, S>;
    type Item = Pin<&'a mut S::Target>;

    fn into_iter(self) -> IterMut<'a, S> {
        IterMut(self.first_mut())
    }
}

impl<S: Selector> LinkedList<S> {
    pub fn iter(&self) -> Iter<S> {
        self.into_iter()
    }
}

impl<S: Selector> LinkedList<S>
where
    S::Ptr: DerefMut,
{
    pub fn iter_mut(self: Pin<&mut Self>) -> IterMut<S> {
        self.into_iter()
    }
}

impl<S: Selector> fmt::Debug for LinkedList<S>
where
    S::Target: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}
