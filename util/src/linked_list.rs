use core::cell::UnsafeCell;
use core::fmt;
use core::marker::{PhantomData, PhantomPinned};
use core::pin::Pin;
use core::ptr::NonNull;
use core::sync::atomic::{
    AtomicBool,
    Ordering::{AcqRel, Relaxed},
};

use crate::offset_of;
use crate::pointer::{Immovable, Pointer, PointerMut, Raw};
use crate::Void;

pub unsafe trait Sel {
    type Ptr: Pointer<Immovable = Self::Immovable, Raw = Self::Raw, Target = Self::Target>;
    type Immovable: Immovable<Ptr = Self::Ptr, Target = Self::Target>;
    type Raw: Raw;
    type Target;

    fn node_offset() -> usize;
}

struct Link {
    prev: Option<NonNull<Self>>,
    next: Option<NonNull<Self>>,
    _pin: PhantomPinned,
}

#[cold]
fn panic_link_ptr_is_null() -> ! {
    panic!("Link ptr is null (did you forget to initialize the list?)");
}

impl Link {
    #[inline]
    const fn new() -> Link {
        Link {
            prev: None,
            next: None,
            _pin: PhantomPinned,
        }
    }

    #[inline]
    fn unwrap_ptr(ptr: Option<NonNull<Self>>) -> NonNull<Self> {
        match ptr {
            Some(ptr) => ptr,
            None => panic_link_ptr_is_null(),
        }
    }

    #[inline]
    fn prev(&self) -> NonNull<Self> {
        Link::unwrap_ptr(self.prev)
    }

    #[inline]
    fn next(&self) -> NonNull<Self> {
        Link::unwrap_ptr(self.next)
    }
}

pub struct Node<S: Sel> {
    link: UnsafeCell<Link>,
    is_active: AtomicBool,
    _marker: PhantomData<S>,
}

unsafe impl<S: Sel> Send for Node<S> {}
unsafe impl<S: Sel> Sync for Node<S> {}

#[cold]
fn panic_node_already_active() -> ! {
    panic!("Value is already in a list");
}

#[cold]
fn panic_node_not_active() -> ! {
    panic!("Value is not in a list");
}

impl<S: Sel> Node<S> {
    #[inline]
    pub const fn new() -> Node<S> {
        Node {
            link: UnsafeCell::new(Link::new()),
            is_active: AtomicBool::new(false),
            _marker: PhantomData,
        }
    }

    #[inline]
    unsafe fn link_mut(&self) -> &mut Link {
        &mut *self.link.get()
    }

    unsafe fn insert_before(&self, after: NonNull<Link>) {
        let before = after.as_ref().prev();
        self.insert_between(before, after);
    }

    unsafe fn insert_after(&self, before: NonNull<Link>) {
        let after = before.as_ref().next();
        self.insert_between(before, after);
    }

    unsafe fn insert_between(&self, mut before: NonNull<Link>, mut after: NonNull<Link>) {
        if self
            .is_active
            .compare_exchange(false, true, AcqRel, Relaxed)
            .is_err()
        {
            let _: Void = panic_node_already_active();
        }

        self.link_mut().prev = Some(before);
        self.link_mut().next = Some(after);
        before.as_mut().next = Some(self.link_mut().into());
        after.as_mut().prev = Some(self.link_mut().into());
    }

    unsafe fn unlink(&self) -> (NonNull<Link>, NonNull<Link>) {
        let mut before = self.link_mut().prev();
        let mut after = self.link_mut().next();
        before.as_mut().next = Some(after);
        after.as_mut().prev = Some(before);

        if self
            .is_active
            .compare_exchange(true, false, AcqRel, Relaxed)
            .is_err()
        {
            let _: Void = panic_node_not_active();
        }

        (before, after)
    }
}

fn value_into_node_ptr<S: Sel>(value: S::Immovable) -> *const Node<S> {
    unsafe {
        let value = S::Immovable::into_pointer(value);
        let raw = S::Ptr::into_raw(value);
        raw_as_node_ptr(raw)
    }
}

unsafe fn raw_as_node_ptr<S: Sel>(raw: S::Raw) -> *const Node<S> {
    raw.cast::<u8>().add(S::node_offset()) as *const Node<S>
}

unsafe fn node_ptr_as_link_ptr<S: Sel>(node: *const Node<S>) -> NonNull<Link> {
    (*node).link_mut().into()
}

unsafe fn raw_as_link_ptr<S: Sel>(raw: S::Raw) -> NonNull<Link> {
    node_ptr_as_link_ptr(raw_as_node_ptr::<S>(raw))
}

unsafe fn link_ptr_as_node_ptr<S: Sel>(link: NonNull<Link>) -> *const Node<S> {
    (link.as_ptr() as *const u8).sub(offset_of!(@ Node<S>, link)) as *const Node<S>
}

unsafe fn node_ptr_as_raw<S: Sel>(node: *const Node<S>) -> S::Raw {
    S::Raw::from_ptr((node as *const u8).sub(S::node_offset()))
}

unsafe fn link_ptr_as_raw<S: Sel>(link: NonNull<Link>) -> S::Raw {
    node_ptr_as_raw(link_ptr_as_node_ptr::<S>(link))
}

impl<S: Sel> Default for Node<S> {
    #[inline]
    fn default() -> Node<S> {
        Node::new()
    }
}

impl<S: Sel> fmt::Debug for Node<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node")
    }
}

pub struct LinkedList<S: Sel> {
    anchor: UnsafeCell<Link>,
    _marker: PhantomData<S>,
}

unsafe impl<S: Sel> Send for LinkedList<S> where S::Immovable: Send {}
unsafe impl<S: Sel> Sync for LinkedList<S> where S::Immovable: Send {}

#[cold]
fn panic_already_initialized() -> ! {
    panic!("Linked list is already initialized");
}

impl<S: Sel> LinkedList<S> {
    pub const fn new() -> LinkedList<S> {
        LinkedList {
            anchor: UnsafeCell::new(Link::new()),
            _marker: PhantomData,
        }
    }

    pub fn initialize(self: Pin<&mut Self>) {
        let mut anchor = unsafe { &mut *self.anchor.get() };
        if anchor.prev.is_some() {
            let _: Void = panic_already_initialized();
        }
        debug_assert!(anchor.next.is_none());
        anchor.prev = Some(anchor.into());
        anchor.next = Some(anchor.into());
    }

    fn is_initialized(&self) -> bool {
        let anchor = unsafe { &*self.anchor.get() };
        debug_assert_eq!(anchor.prev.is_some(), anchor.next.is_some());
        anchor.prev.is_some()
    }
}

#[derive(Copy, Clone)]
struct CursorBase {
    anchor: NonNull<Link>,
    link: NonNull<Link>,
}

impl CursorBase {
    #[inline]
    fn from_anchor_and_link(anchor: NonNull<Link>, link: NonNull<Link>) -> Option<Self> {
        if link != anchor {
            Some(CursorBase {
                anchor: anchor,
                link,
            })
        } else {
            None
        }
    }

    #[inline]
    fn try_move(&self, to: NonNull<Link>) -> Result<Self, Self> {
        Self::from_anchor_and_link(self.anchor, to).ok_or_else(|| self.clone())
    }

    #[inline]
    fn prev(&self) -> Result<Self, Self> {
        self.try_move(unsafe { self.link.as_ref().prev() })
    }

    #[inline]
    fn next(&self) -> Result<Self, Self> {
        self.try_move(unsafe { self.link.as_ref().next() })
    }

    #[inline]
    fn as_ref<S: Sel>(&self) -> &S::Target {
        unsafe { <S::Ptr as Pointer>::raw_deref(link_ptr_as_raw::<S>(self.link)) }
    }

    #[inline]
    fn as_mut<S: Sel>(&mut self) -> Pin<&mut S::Target>
    where
        S::Ptr: PointerMut,
    {
        unsafe {
            Pin::new_unchecked(<S::Ptr as PointerMut>::raw_deref_mut(link_ptr_as_raw::<S>(
                self.link,
            )))
        }
    }

    unsafe fn get_immovable_unchecked<S: Sel>(&self) -> S::Immovable {
        let node = link_ptr_as_node_ptr::<S>(self.link);
        let raw = node_ptr_as_raw(node);
        let value = S::Ptr::from_raw(raw);
        S::Immovable::from_pointer(value)
    }

    fn get<'a, S: Sel>(&self) -> Transient<'a, S::Immovable> {
        unsafe { Transient::new(self.get_immovable_unchecked::<S>()) }
    }

    fn get_mut<'a, S: Sel>(&mut self) -> TransientMut<'a, S::Immovable> {
        unsafe { TransientMut::new(self.get_immovable_unchecked::<S>()) }
    }

    #[inline]
    fn peek_next_link(&self) -> Option<NonNull<Link>> {
        let next_link = unsafe { self.link.as_ref().next() };
        if next_link != self.anchor {
            Some(next_link)
        } else {
            None
        }
    }

    #[inline]
    fn peek_prev_link(&self) -> Option<NonNull<Link>> {
        let prev_link = unsafe { self.link.as_ref().prev() };
        if prev_link != self.anchor {
            Some(prev_link)
        } else {
            None
        }
    }

    #[inline]
    fn peek_next<S: Sel>(&self) -> Option<&S::Target> {
        self.peek_next_link()
            .map(|link| unsafe { <S::Ptr as Pointer>::raw_deref(link_ptr_as_raw::<S>(link)) })
    }

    #[inline]
    fn peek_prev<S: Sel>(&self) -> Option<&S::Target> {
        self.peek_prev_link()
            .map(|link| unsafe { <S::Ptr as Pointer>::raw_deref(link_ptr_as_raw::<S>(link)) })
    }
}

pub struct Cursor<'a, S: Sel> {
    base: CursorBase,
    _marker: PhantomData<&'a LinkedList<S>>,
}

impl<'a, S: Sel> Copy for Cursor<'a, S> {}
impl<'a, S: Sel> Clone for Cursor<'a, S> {
    #[inline]
    fn clone(&self) -> Cursor<'a, S> {
        Cursor {
            base: self.base.clone(),
            _marker: PhantomData,
        }
    }
}

impl<'a, S: Sel> Cursor<'a, S> {
    #[inline]
    fn from_base(base: CursorBase) -> Self {
        Cursor {
            base,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn prev(&self) -> Result<Self, Self> {
        self.base
            .prev()
            .map(Self::from_base)
            .map_err(Self::from_base)
    }

    #[inline]
    pub fn next(&self) -> Result<Self, Self> {
        self.base
            .next()
            .map(Self::from_base)
            .map_err(Self::from_base)
    }

    #[inline]
    pub fn as_ref(&self) -> &S::Target {
        self.base.as_ref::<S>()
    }

    pub fn get(&self) -> Transient<'a, S::Immovable> {
        self.base.get::<S>()
    }

    #[inline]
    pub fn peek_prev(&self) -> Option<&S::Target> {
        self.base.peek_prev::<S>()
    }

    #[inline]
    pub fn peek_next(&self) -> Option<&S::Target> {
        self.base.peek_next::<S>()
    }
}

impl<'a, S: Sel> core::ops::Deref for Cursor<'a, S> {
    type Target = S::Target;

    #[inline]
    fn deref(&self) -> &S::Target {
        self.as_ref()
    }
}

pub struct CursorMut<'a, S: Sel> {
    base: CursorBase,
    _marker: PhantomData<Pin<&'a mut LinkedList<S>>>,
}

impl<'a, S: Sel> CursorMut<'a, S> {
    #[inline]
    fn from_base(base: CursorBase) -> Self {
        CursorMut {
            base,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn prev(&self) -> Result<Self, Self> {
        self.base
            .prev()
            .map(Self::from_base)
            .map_err(Self::from_base)
    }

    #[inline]
    pub fn next(&self) -> Result<Self, Self> {
        self.base
            .next()
            .map(Self::from_base)
            .map_err(Self::from_base)
    }

    #[inline]
    pub fn as_ref(&self) -> &S::Target {
        self.base.as_ref::<S>()
    }

    #[inline]
    pub fn as_mut(&mut self) -> Pin<&mut S::Target>
    where
        S::Ptr: PointerMut,
    {
        self.base.as_mut::<S>()
    }

    pub fn get(&self) -> Transient<'a, S::Immovable> {
        self.base.get::<S>()
    }

    pub fn get_mut(&mut self) -> TransientMut<S::Immovable> {
        self.base.get_mut::<S>()
    }

    #[inline]
    pub fn peek_prev(&self) -> Option<&S::Target> {
        self.base.peek_prev::<S>()
    }

    #[inline]
    pub fn peek_next(&self) -> Option<&S::Target> {
        self.base.peek_next::<S>()
    }

    fn insert_before_(&mut self, value: S::Immovable) -> *const Node<S> {
        unsafe {
            let node = value_into_node_ptr::<S>(value);
            (*node).insert_before(self.base.link);
            node
        }
    }

    fn insert_after_(&mut self, value: S::Immovable) -> *const Node<S> {
        unsafe {
            let node = value_into_node_ptr::<S>(value);
            (*node).insert_after(self.base.link);
            node
        }
    }

    pub fn insert_before(&mut self, value: S::Immovable) {
        self.insert_before_(value);
    }

    pub fn insert_after(&mut self, value: S::Immovable) {
        self.insert_after_(value);
    }

    pub fn insert_before_and_get(mut self, value: S::Immovable) -> CursorMut<'a, S> {
        unsafe {
            let node = self.insert_before_(value);
            CursorMut::from_base(CursorBase {
                anchor: self.base.anchor,
                link: node_ptr_as_link_ptr(node),
            })
        }
    }

    pub fn insert_after_and_get(mut self, value: S::Immovable) -> CursorMut<'a, S> {
        unsafe {
            let node = self.insert_after_(value);
            CursorMut::from_base(CursorBase {
                anchor: self.base.anchor,
                link: node_ptr_as_link_ptr(node),
            })
        }
    }

    fn unlink_(self) -> (S::Immovable, (NonNull<Link>, NonNull<Link>)) {
        unsafe {
            let node = link_ptr_as_node_ptr::<S>(self.base.link);
            let links = (*node).unlink();
            let raw = node_ptr_as_raw(node);
            let value = S::Ptr::from_raw(raw);
            (S::Immovable::from_pointer(value), links)
        }
    }

    pub fn unlink(self) -> S::Immovable {
        self.unlink_().0
    }

    pub fn unlink_and_prev(self) -> (S::Immovable, Option<CursorMut<'a, S>>) {
        let anchor = self.base.anchor;
        let (value, links) = self.unlink_();
        (
            value,
            CursorBase::from_anchor_and_link(anchor, links.0).map(Self::from_base),
        )
    }

    pub fn unlink_and_next(self) -> (S::Immovable, Option<CursorMut<'a, S>>) {
        let anchor = self.base.anchor;
        let (value, links) = self.unlink_();
        (
            value,
            CursorBase::from_anchor_and_link(anchor, links.1).map(Self::from_base),
        )
    }
}

impl<'a, S: Sel> core::ops::Deref for CursorMut<'a, S> {
    type Target = S::Target;

    #[inline]
    fn deref(&self) -> &S::Target {
        self.as_ref()
    }
}

impl<S: Sel> LinkedList<S> {
    fn anchor(&self) -> NonNull<Link> {
        unsafe { (&mut *self.anchor.get()).into() }
    }

    #[inline]
    pub unsafe fn cursor_from_raw(&self, raw: S::Raw) -> Cursor<S> {
        Cursor::from_base(CursorBase {
            anchor: self.anchor(),
            link: raw_as_link_ptr::<S>(raw),
        })
    }

    #[inline]
    pub unsafe fn cursor_mut_from_raw(self: Pin<&mut Self>, raw: S::Raw) -> CursorMut<S> {
        CursorMut::from_base(CursorBase {
            anchor: self.anchor(),
            link: raw_as_link_ptr::<S>(raw),
        })
    }

    fn first_cursor_base(&self) -> Option<CursorBase> {
        let first = unsafe { self.anchor().as_ref().next() };
        if first != self.anchor() {
            Some(CursorBase {
                anchor: self.anchor(),
                link: first,
            })
        } else {
            None
        }
    }

    fn last_cursor_base(&self) -> Option<CursorBase> {
        let last = unsafe { self.anchor().as_ref().prev() };
        if last != self.anchor() {
            Some(CursorBase {
                anchor: self.anchor(),
                link: last,
            })
        } else {
            None
        }
    }

    pub fn first(&self) -> Option<Cursor<S>> {
        self.first_cursor_base().map(Cursor::from_base)
    }

    pub fn first_mut(self: Pin<&mut Self>) -> Option<CursorMut<S>> {
        self.first_cursor_base().map(CursorMut::from_base)
    }

    pub fn last(&self) -> Option<Cursor<S>> {
        self.last_cursor_base().map(Cursor::from_base)
    }

    pub fn last_mut(self: Pin<&mut Self>) -> Option<CursorMut<S>> {
        self.last_cursor_base().map(CursorMut::from_base)
    }

    pub fn push_front(self: Pin<&mut Self>, value: S::Immovable) -> CursorMut<S> {
        unsafe {
            let node = value_into_node_ptr::<S>(value);
            (*node).insert_after(self.anchor());
            CursorMut::from_base(
                CursorBase::from_anchor_and_link(self.anchor(), node_ptr_as_link_ptr(node))
                    .unwrap(),
            )
        }
    }

    pub fn push_back(self: Pin<&mut Self>, value: S::Immovable) -> CursorMut<S> {
        unsafe {
            let node = value_into_node_ptr::<S>(value);
            (*node).insert_before(self.anchor());
            CursorMut::from_base(
                CursorBase::from_anchor_and_link(self.anchor(), node_ptr_as_link_ptr(node))
                    .unwrap(),
            )
        }
    }

    pub fn pop_front(self: Pin<&mut Self>) -> Option<S::Immovable> {
        self.first_mut().map(CursorMut::unlink)
    }

    pub fn pop_back(self: Pin<&mut Self>) -> Option<S::Immovable> {
        self.last_mut().map(CursorMut::unlink)
    }

    pub fn swap(self: Pin<&mut Self>, other: Pin<&mut Self>) {
        unsafe {
            let mut self_anchor = self.anchor();
            let mut other_anchor = other.anchor();

            let (mut self_first, mut self_last) =
                (self_anchor.as_ref().next(), self_anchor.as_ref().prev());
            let (mut other_first, mut other_last) =
                (other_anchor.as_ref().next(), other_anchor.as_ref().prev());

            self_anchor.as_mut().next = Some(other_first);
            self_anchor.as_mut().prev = Some(other_last);
            other_anchor.as_mut().next = Some(self_first);
            other_anchor.as_mut().prev = Some(self_last);

            self_first.as_mut().prev = Some(other_anchor);
            self_last.as_mut().next = Some(other_anchor);
            other_first.as_mut().prev = Some(self_anchor);
            other_last.as_mut().next = Some(self_anchor);
        }
    }

    fn drop_pinned(mut self: Pin<&mut Self>) {
        if !self.is_initialized() {
            return;
        }
        while let Some(_) = self.as_mut().pop_front() {}
    }
}

impl<S: Sel> Drop for LinkedList<S> {
    fn drop(&mut self) {
        unsafe {
            Pin::new_unchecked(self).drop_pinned();
        }
    }
}

pub struct Iter<'a, S: Sel>(Option<Cursor<'a, S>>);

impl<'a, S: Sel> Iterator for Iter<'a, S> {
    type Item = &'a S::Target;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|cursor| {
            let item_ptr = cursor.as_ref() as *const _;
            self.0 = cursor.next().ok();
            unsafe { &*item_ptr }
        })
    }
}

pub struct IterMut<'a, S: Sel>(Option<CursorMut<'a, S>>);

impl<'a, S: Sel> Iterator for IterMut<'a, S>
where
    S::Ptr: PointerMut,
{
    type Item = Pin<&'a mut S::Target>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|mut cursor| {
            let item_ptr = unsafe { cursor.as_mut().get_unchecked_mut() as *mut _ };
            self.0 = cursor.next().ok();
            unsafe { Pin::new_unchecked(&mut *item_ptr) }
        })
    }
}

impl<'a, S: Sel> IntoIterator for &'a LinkedList<S> {
    type IntoIter = Iter<'a, S>;
    type Item = &'a S::Target;

    fn into_iter(self) -> Iter<'a, S> {
        Iter(self.first())
    }
}

impl<'a, S: Sel> IntoIterator for Pin<&'a mut LinkedList<S>>
where
    S::Ptr: PointerMut,
{
    type IntoIter = IterMut<'a, S>;
    type Item = Pin<&'a mut S::Target>;

    fn into_iter(self) -> IterMut<'a, S> {
        IterMut(self.first_mut())
    }
}

impl<S: Sel> LinkedList<S> {
    pub fn iter(&self) -> Iter<S> {
        self.into_iter()
    }

    pub fn iter_mut(self: Pin<&mut Self>) -> IterMut<S>
    where
        S::Ptr: PointerMut,
    {
        self.into_iter()
    }
}

impl<S: Sel> fmt::Debug for LinkedList<S>
where
    S::Target: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

pub struct Transient<'a, T>(core::mem::ManuallyDrop<T>, PhantomData<&'a T>);

impl<'a, T> Transient<'a, T> {
    fn new(x: T) -> Transient<'a, T> {
        Transient(core::mem::ManuallyDrop::new(x), PhantomData)
    }

    pub unsafe fn transmute_lt<'b>(self) -> Transient<'b, T> {
        Transient(self.0, PhantomData)
    }
}

impl<'a, T> core::ops::Deref for Transient<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &*self.0
    }
}

pub struct TransientMut<'a, T>(core::mem::ManuallyDrop<T>, PhantomData<&'a mut T>);

impl<'a, T> TransientMut<'a, T> {
    fn new(x: T) -> TransientMut<'a, T> {
        TransientMut(core::mem::ManuallyDrop::new(x), PhantomData)
    }

    pub unsafe fn transmute_lt<'b>(self) -> TransientMut<'b, T> {
        TransientMut(self.0, PhantomData)
    }
}

impl<'a, T> core::ops::Deref for TransientMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &*self.0
    }
}

impl<'a, T> core::ops::DerefMut for TransientMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.0
    }
}
