#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![feature(const_fn, const_if_match, const_loop, const_panic)]
#![feature(const_alloc_layout)]
#![cfg_attr(test, feature(test))]

use core::marker::PhantomData;
use core::mem;
use core::pin::Pin;
use core::ptr::{self, NonNull};

use brutos_alloc::{Layout, OutOfMemory, Unique};
use brutos_util::linked_list::{LinkedList, Node};
use brutos_util::UInt;
use brutos_util::Void;

// use crate::arch::PAGE_SIZE;
use brutos_memory_traits::AllocMappedPage;
use brutos_memory_units::Order;

type Mask = u128;
const MASK_LEN: usize = 2;

const MAX_SLAB_CAP: usize = <Mask as UInt>::BIT_COUNT as usize * MASK_LEN;

pub struct Allocator<Cx: AllocMappedPage> {
    info: Info,
    slabs: LinkedList<SlabSel>,
    _marker: PhantomData<Cx>,
}

unsafe impl<Cx: AllocMappedPage> Send for Allocator<Cx> {}
unsafe impl<Cx: AllocMappedPage> Sync for Allocator<Cx> {}

impl<Cx: AllocMappedPage> Allocator<Cx> {
    fn deconstruct<'a>(
        self: &'a mut Pin<&mut Self>,
    ) -> (&'a Info, Pin<&'a mut LinkedList<SlabSel>>) {
        unsafe {
            let this = Pin::into_inner_unchecked(self.as_mut());
            (&this.info, Pin::new_unchecked(&mut this.slabs))
        }
    }

    pub const fn new<T>(order: Order) -> Allocator<Cx> {
        Allocator {
            info: Info::new::<T>(order),
            slabs: LinkedList::new(),
            _marker: PhantomData,
        }
    }

    pub fn initialize(mut self: Pin<&mut Self>) {
        let (_, slabs) = self.deconstruct();
        slabs.initialize();
    }

    #[inline(never)]
    pub fn alloc(mut self: Pin<&mut Self>) -> Result<NonNull<u8>, OutOfMemory> {
        let (info, slabs) = self.deconstruct();

        for slab in slabs {
            if let Some(ptr) = slab.alloc(info) {
                return Ok(ptr);
            }
        }
        self.create_slab_and_alloc()
    }

    #[cold]
    fn create_slab_and_alloc(mut self: Pin<&mut Self>) -> Result<NonNull<u8>, OutOfMemory> {
        let (info, slabs) = self.deconstruct();
        let mut slab = Slab::create::<Cx>(info)?;
        let ptr = slab
            .as_mut()
            .alloc(info)
            .expect("Failed to allocate from newly created slab");
        slabs.push_front(slab);
        Ok(ptr)
    }

    fn dealloc_(mut self: Pin<&mut Self>, ptr: NonNull<u8>) {
        let (info, slabs) = self.deconstruct();

        let mut next_cursor = slabs.first_mut();
        while let Some(mut cursor) = next_cursor.take() {
            if cursor.as_mut().dealloc(info, ptr) {
                if cursor.is_empty(info) {
                    let slab = cursor.unlink();
                    Slab::destroy::<Cx>(info, slab);
                }
                return;
            } else {
                next_cursor = cursor.next().ok();
            }
        }
        let _: Void = Self::panic_invalid_pointer();
    }

    #[cold]
    fn panic_invalid_pointer() -> ! {
        panic!("Attempt to dealloc invalid pointer");
    }

    #[inline(never)]
    pub unsafe fn dealloc(self: Pin<&mut Self>, ptr: NonNull<u8>) {
        self.dealloc_(ptr);
    }

    fn drop_pinned(mut self: Pin<&mut Self>) {
        let (info, slabs) = self.deconstruct();

        let mut next_cursor = slabs.first_mut();
        while let Some(cursor) = next_cursor.take() {
            assert!(cursor.is_empty(info));
            let (slab, next) = cursor.unlink_and_next();
            Slab::destroy::<Cx>(info, slab);
            next_cursor = next;
        }
    }
}

impl<Cx: AllocMappedPage> Drop for Allocator<Cx> {
    fn drop(&mut self) {
        let this = unsafe { Pin::new_unchecked(self) };
        this.drop_pinned();
    }
}

#[derive(Copy, Clone)]
struct Info {
    entry_size: usize,
    order: Order,
    cap: usize,
    initial_mask: [Mask; MASK_LEN],
}

impl Info {
    const fn new<T>(order: Order) -> Info {
        let entry_layout = Layout::new::<T>();
        assert!(order.size() >= entry_layout.align());
        let slab_size = order.size() - mem::size_of::<Slab>();
        assert!(slab_size % mem::align_of::<Slab>() == 0);
        let cap = slab_size / entry_layout.size();
        let cap = if cap < MAX_SLAB_CAP {
            cap
        } else {
            MAX_SLAB_CAP
        };
        assert!(cap > 0);

        let mut initial_mask = [0; MASK_LEN];
        {
            const BIT_COUNT: usize = <Mask as UInt>::BIT_COUNT as usize;
            let mut i = 0;
            while i < MASK_LEN {
                let start_n = i * BIT_COUNT;
                if cap > start_n {
                    let len = cap - start_n;
                    let len = if len < BIT_COUNT { len } else { BIT_COUNT };
                    initial_mask[i] = (1 << len) - 1;
                }
                i += 1;
            }
        }

        Info {
            entry_size: entry_layout.size(),
            order,
            cap,
            initial_mask,
        }
    }

    fn state_offset(&self) -> usize {
        self.order.size() - mem::size_of::<Slab>()
    }
}

brutos_util_macros::selector!(SlabSel: Unique<Slab> => node);
struct Slab {
    free_count: usize,
    entry_mask: [Mask; MASK_LEN],
    node: Node<SlabSel>,
}

impl Slab {
    fn create<Cx: AllocMappedPage>(info: &Info) -> Result<Pin<Unique<Slab>>, OutOfMemory> {
        unsafe {
            let page: *mut u8 = Cx::alloc(info.order).map_err(|()| OutOfMemory)?.as_ptr();
            let state = page.add(info.state_offset()).cast();
            ptr::write(
                state,
                Slab {
                    free_count: info.cap,
                    entry_mask: info.initial_mask,
                    node: Node::new(),
                },
            );
            Ok(Pin::new_unchecked(Unique::from_raw(state)))
        }
    }

    fn destroy<Cx: AllocMappedPage>(info: &Info, mut state: Pin<Unique<Slab>>) {
        unsafe {
            let page: *mut u8 = state.as_mut().slab_ptr(info);
            mem::forget(state);
            Cx::dealloc(NonNull::new_unchecked(page), info.order);
        }
    }

    fn free_count<'a>(self: &'a mut Pin<&mut Self>) -> &'a mut usize {
        unsafe { &mut self.as_mut().get_unchecked_mut().free_count }
    }

    fn entry_mask<'a>(self: &'a mut Pin<&mut Self>) -> &'a mut [Mask; MASK_LEN] {
        unsafe { &mut self.as_mut().get_unchecked_mut().entry_mask }
    }

    fn slab_ptr(self: &mut Pin<&mut Self>, info: &Info) -> *mut u8 {
        unsafe {
            let state_ptr: *mut Self = self.as_mut().get_unchecked_mut() as *mut Self;
            (state_ptr as *mut u8).sub(info.state_offset())
        }
    }

    fn alloc(mut self: Pin<&mut Self>, info: &Info) -> Option<NonNull<u8>> {
        if self.free_count == 0 {
            return None;
        }

        self.entry_mask
            .iter()
            .enumerate()
            .filter_map(|(mask_i, mask)| mask.lsb().map(|bit_i| (mask_i, bit_i)))
            .next()
            .map(|(mask_i, bit_i)| {
                *self.free_count() -= 1;
                self.entry_mask()[mask_i].set_bit(bit_i, false);

                let offset = mask_i * Mask::BIT_COUNT as usize * info.entry_size
                    + bit_i as usize * info.entry_size;
                unsafe { NonNull::new_unchecked(self.slab_ptr(info).add(offset)) }
            })
    }

    #[must_use]
    fn dealloc(mut self: Pin<&mut Self>, info: &Info, entry_ptr: NonNull<u8>) -> bool {
        let entry_ptr = entry_ptr.as_ptr() as *const _;
        let state_ptr: *const u8 = &*self as *const Self as *const u8;
        let slab_ptr = self.slab_ptr(info);
        if entry_ptr < slab_ptr || entry_ptr >= state_ptr {
            return false;
        }

        let entry_i = (entry_ptr as usize - slab_ptr as usize) / info.entry_size;
        let mask_i = entry_i / <Mask as UInt>::BIT_COUNT as usize;
        let bit_i = (entry_i % <Mask as UInt>::BIT_COUNT as usize) as u32;

        assert_eq!(
            self.entry_mask()[mask_i].bit(bit_i),
            false,
            "Pointer was not allocated"
        );
        self.entry_mask()[mask_i].set_bit(bit_i, true);
        *self.free_count() += 1;
        true
    }

    fn is_empty(&self, info: &Info) -> bool {
        self.free_count == info.cap
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    struct Context;

    impl Context {
        fn layout(order: Order) -> std::alloc::Layout {
            std::alloc::Layout::from_size_align(order.size(), order.size()).unwrap()
        }
    }

    unsafe impl AllocMappedPage for Context {
        const MAX_ORDER: Order = Order(255);

        fn alloc(order: Order) -> Result<NonNull<u8>, ()> {
            unsafe {
                let layout = Self::layout(order);
                Ok(NonNull::new_unchecked(std::alloc::alloc(layout)))
            }
        }

        unsafe fn dealloc(ptr: NonNull<u8>, order: Order) {
            std::alloc::dealloc(ptr.as_ptr(), Self::layout(order));
        }
    }

    #[cfg(not(miri))]
    use std::hint::black_box;
    #[cfg(miri)]
    fn black_box<T>(x: T) -> T {
        x
    }

    #[test]
    fn slab() {
        type Ty = [usize; 4];
        let mut a = black_box(Box::pin(Allocator::<Context>::new::<Ty>(Order(0))));
        a.as_mut().initialize();

        let cap = a.info.cap;
        let n = cap * 4 + cap / 2;

        let mut entries = Vec::new();
        for _ in 0..n {
            let ptr = a.as_mut().alloc().unwrap();
            entries.push(ptr);
            unsafe {
                ptr::write_bytes(ptr.as_ptr() as *mut Ty, 0u8, 1);
            }
        }
        entries.shuffle(&mut rand::thread_rng());
        for entry in entries.into_iter() {
            unsafe {
                a.as_mut().dealloc(entry);
            }
        }
    }
}
