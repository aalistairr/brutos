use core::cell::Cell;
use core::ops::Range;
use core::pin::Pin;

use crate::arch::PAGE_SIZE;
use crate::{Order, PhysAddr};
use brutos_util::linked_list::{self, LinkedList};

pub mod bootstrap;

pub const MAX_ORDER: u8 = 18;
const ORDER_COUNT: usize = MAX_ORDER as usize + 1;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct TooLarge;
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct NotAllocated;

#[derive(Debug)]
pub struct Allocator<'a, T> {
    regions: &'a [Region<'a, T>],
    free_pages: [LinkedList<PageSel<'a, T>>; ORDER_COUNT],
}

#[derive(Debug)]
struct Region<'a, T> {
    range: Range<PhysAddr>,
    pages: &'a [Page<'a, T>],
}

impl<'a, T> Region<'a, T> {
    fn page_at_addr(&self, addr: PhysAddr) -> &'a Page<'a, T> {
        // Subtraction can be eliminated by offsetting the pointer stored in `Region`
        // at the cost of the introduction either two comparisons or more unsafety
        &self.pages[(addr.0 - self.range.start.0) / PAGE_SIZE]
    }
}

brutos_util_macros::selector!(PageSel<'a, T: 'a>: &'a Page<'a, T> => node);
#[derive(Debug)]
struct Page<'a, T> {
    region: usize,
    addr: PhysAddr,
    tree_order: u8,
    state: Cell<State>,
    node: linked_list::Node<PageSel<'a, T>>,
    data: T,
}

unsafe impl<'a, T: Send> Send for Page<'a, T> {}
unsafe impl<'a, T: Send> Sync for Page<'a, T> {}

#[derive(Copy, Clone, PartialEq, Debug)]
enum State {
    Free(u8),
    Allocated(u8),
    Unreachable,
}

impl<'a, T: 'a> Allocator<'a, T> {
    const NEW_LINKED_LIST: LinkedList<PageSel<'a, T>> = LinkedList::new();
    pub const fn new() -> Allocator<'a, T> {
        Allocator {
            regions: &[],
            free_pages: [Self::NEW_LINKED_LIST; ORDER_COUNT],
        }
    }

    fn index_free_pages<'this>(
        self: Pin<&'this mut Self>,
        index: u8,
    ) -> Pin<&'this mut LinkedList<PageSel<'a, T>>> {
        unsafe { self.map_unchecked_mut(|a| &mut a.free_pages[index as usize]) }
    }

    pub fn initialize(mut self: Pin<&mut Self>) {
        for i in 0..=MAX_ORDER {
            self.as_mut().index_free_pages(i).initialize();
        }
    }

    pub fn allocate(
        mut self: Pin<&mut Self>,
        requested_order: Order,
    ) -> Result<Option<(PhysAddr, &'a T)>, TooLarge> {
        let requested_order = requested_order.0;
        if requested_order > MAX_ORDER {
            return Err(TooLarge);
        }
        let (full_order, page) = match (requested_order..=MAX_ORDER).find_map(|o| {
            self.as_mut()
                .index_free_pages(o)
                .pop_front()
                .map(|p| (o, p))
        }) {
            Some(x) => x,
            None => return Ok(None),
        };
        for partial_order in requested_order..full_order {
            let buddy = PhysAddr(page.addr.0 | ((1 << partial_order) * PAGE_SIZE));
            let buddy = self.regions[page.region].page_at_addr(buddy);
            buddy.state.set(State::Free(partial_order));
            self.as_mut()
                .index_free_pages(partial_order)
                .push_back(buddy);
        }
        page.state.set(State::Allocated(requested_order));
        Ok(Some((page.addr, &page.data)))
    }

    fn find_region(&self, addr: PhysAddr) -> Result<&'a Region<'a, T>, NotAllocated> {
        self.regions
            .binary_search_by(|region| {
                if region.range.contains(&addr) {
                    core::cmp::Ordering::Equal
                } else {
                    region.range.start.cmp(&addr)
                }
            })
            .map(|region| &self.regions[region])
            .map_err(|_| NotAllocated)
    }

    pub fn free(mut self: Pin<&mut Self>, mut addr: PhysAddr) -> Result<(), NotAllocated> {
        if !addr.is_aligned(PAGE_SIZE) {
            return Err(NotAllocated);
        }
        let region = self.find_region(addr)?;
        let (initial_order, tree_order) = {
            let page = region.page_at_addr(addr);
            match page.state.get() {
                State::Allocated(page_order) => (page_order, page.tree_order),
                _ => return Err(NotAllocated),
            }
        };

        for order in initial_order..=tree_order {
            if order == tree_order {
                self.add_to_free_pages(region, addr, order);
                return Ok(());
            }
            let buddy_addr = PhysAddr(addr.0 ^ ((1 << order) * PAGE_SIZE));
            let buddy = region.page_at_addr(buddy_addr);
            match buddy.state.get() {
                State::Free(buddy_order) if buddy_order == order => {
                    buddy.state.set(State::Unreachable);
                    unsafe {
                        self.as_mut()
                            .index_free_pages(order)
                            .cursor_mut_from_raw(buddy)
                            .unlink();
                    }
                    addr.0 &= !((1 << order) * PAGE_SIZE);
                }
                _ => {
                    self.add_to_free_pages(region, addr, order);
                    return Ok(());
                }
            }
        }
        unreachable!()
    }

    pub fn find(&self, addr: PhysAddr) -> Result<&'a T, NotAllocated> {
        if !addr.is_aligned(PAGE_SIZE) {
            return Err(NotAllocated);
        }
        let region = self.find_region(addr)?;
        Ok(&region.page_at_addr(addr).data)
    }

    fn add_to_free_pages(
        self: Pin<&mut Self>,
        region: &'a Region<'a, T>,
        addr: PhysAddr,
        order: u8,
    ) {
        let page = region.page_at_addr(addr);
        page.state.set(State::Free(order));
        self.index_free_pages(order).push_front(page);
    }

    #[cfg(test)]
    fn shuffle_free_pages(mut self: Pin<&mut Self>) {
        use rand::prelude::*;
        for order in 0..=MAX_ORDER {
            let mut free_pages_ll = self.as_mut().index_free_pages(order);
            let mut free_pages_vec = Vec::new();
            while let Some(page) = free_pages_ll.as_mut().pop_front() {
                free_pages_vec.push(page);
            }
            free_pages_vec.shuffle(&mut rand::thread_rng());
            while let Some(page) = free_pages_vec.pop() {
                free_pages_ll.as_mut().push_front(page);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocate() {
        use self::region as r;
        use self::tree as t;
        use self::TestState::Allocated as A;
        use self::TestState::Free as F;
        assert!(test_allocate(([], []), (0, Ok(None)), ([], [])));
        assert!(test_allocate(
            ([], []),
            (std::u8::MAX, Err(TooLarge)),
            ([], [])
        ));
        assert!(test_free(([], []), (0, Err(NotAllocated)), ([], [])));
        assert!(test_allocate(
            ([r([t([F(1)])])], [(1, vec![0])]),
            (1, Ok(Some(0))),
            ([r([t([A(1)])])], []),
        ));
        assert!(test_allocate(
            ([r([t([F(2)]), t([F(2)])])], [(2, vec![0, 4])]),
            (0, Ok(Some(0))),
            (
                [r([t([A(0), F(0), F(1)]), t([F(2)])])],
                [(0, vec![1]), (1, vec![2]), (2, vec![4])]
            ),
        ));
        assert!(test_allocate(
            (
                [r([t([A(0), F(0), F(1)]), t([F(2)])])],
                [(0, vec![1]), (1, vec![2]), (2, vec![4])]
            ),
            (0, Ok(Some(1))),
            (
                [r([t([A(0), A(0), F(1)]), t([F(2)])])],
                [(1, vec![2]), (2, vec![4])]
            ),
        ));
        assert!(test_allocate(
            (
                [r([t([A(0), A(0), F(1)]), t([F(2)])])],
                [(1, vec![2]), (2, vec![4])]
            ),
            (0, Ok(Some(2))),
            (
                [r([t([A(0), A(0), A(0), F(0)]), t([F(2)])])],
                [(0, vec![3]), (2, vec![4])]
            ),
        ));
        assert!(test_free(
            ([r([t([A(0)])])], []),
            (0, Ok(())),
            ([r([t([F(0)])])], [(0, vec![0])])
        ));
        assert!(test_free(
            (
                [r([t([A(0), A(0), A(0), F(0)]), t([F(2)])])],
                [(0, vec![3]), (2, vec![4])]
            ),
            (2, Ok(())),
            (
                [r([t([A(0), A(0), F(1)]), t([F(2)])])],
                [(1, vec![2]), (2, vec![4])]
            ),
        ));
        assert!(test_free(
            (
                [r([t([A(0), A(0), F(1)]), t([F(2)])])],
                [(1, vec![2]), (2, vec![4])]
            ),
            (1, Ok(())),
            (
                [r([t([A(0), F(0), F(1)]), t([F(2)])])],
                [(0, vec![1]), (1, vec![2]), (2, vec![4])]
            ),
        ));
        assert!(test_free(
            (
                [r([t([A(0), F(0), F(1)]), t([F(2)])])],
                [(0, vec![1]), (1, vec![2]), (2, vec![4])]
            ),
            (0, Ok(())),
            ([r([t([F(2)]), t([F(2)])])], [(2, vec![0, 4])]),
        ));
        assert!(test_allocate(
            ([r([t([A(0), A(0), A(1)]), t([F(2)])])], [(2, vec![4])]),
            (0, Ok(Some(4))),
            (
                [r([t([A(0), A(0), A(1)]), t([A(0), F(0), F(1)])])],
                [(0, vec![5]), (1, vec![6])]
            )
        ));
        assert!(test_free(
            (
                [r([t([A(0), A(0), A(1)]), t([A(0), F(0), F(1)])])],
                [(0, vec![5]), (1, vec![6])]
            ),
            (4, Ok(())),
            ([r([t([A(0), A(0), A(1)]), t([F(2)])])], [(2, vec![4])]),
        ));
        assert!(test_allocate(
            ([r([t([A(2)])]), r([t([A(0), F(0)])])], [(0, vec![5])]),
            (0, Ok(Some(5))),
            ([r([t([A(2)])]), r([t([A(0), A(0)])])], [])
        ));
        assert!(test_free(
            ([r([t([A(2)])]), r([t([A(0), A(0)])])], []),
            (5, Ok(())),
            ([r([t([A(2)])]), r([t([A(0), F(0)])])], [(0, vec![5])]),
        ));
        assert!(test_free(
            ([r([t([A(2)])]), r([t([A(0), F(0)])])], [(0, vec![5])]),
            (4, Ok(())),
            ([r([t([A(2)])]), r([t([F(1)])])], [(1, vec![4])]),
        ));
    }

    #[must_use]
    fn test_allocator<
        const N_X0: usize,
        const N_X1: usize,
        const N_Y0: usize,
        const N_Y1: usize,
    >(
        x: ([TestRegion; N_X0], [(u8, Vec<usize>); N_X1]),
        y: ([TestRegion; N_Y0], [(u8, Vec<usize>); N_Y1]),
        f: impl FnOnce(Pin<&mut Allocator<()>>) -> bool,
    ) -> bool {
        let mut a = allocator(&x.0, Some(&x.1));
        if !f(a.as_mut()) {
            return false;
        }
        compare_allocators(&a, &allocator(&y.0, Some(&y.1)))
    }

    #[must_use]
    fn test_allocate<const N_X0: usize, const N_X1: usize, const N_Y0: usize, const N_Y1: usize>(
        x: ([TestRegion; N_X0], [(u8, Vec<usize>); N_X1]),
        (order, output): (u8, Result<Option<usize>, TooLarge>),
        y: ([TestRegion; N_Y0], [(u8, Vec<usize>); N_Y1]),
    ) -> bool {
        let output = output.map(|x| x.map(|page| PhysAddr(page * PAGE_SIZE)));
        test_allocator(x, y, |a| {
            a.allocate(Order(order)).map(|x| x.map(|x| x.0)) == output
        })
    }

    #[must_use]
    fn test_free<const N_X0: usize, const N_X1: usize, const N_Y0: usize, const N_Y1: usize>(
        x: ([TestRegion; N_X0], [(u8, Vec<usize>); N_X1]),
        (page, output): (usize, Result<(), NotAllocated>),
        y: ([TestRegion; N_Y0], [(u8, Vec<usize>); N_Y1]),
    ) -> bool {
        let addr = PhysAddr(page * PAGE_SIZE);
        test_allocator(x, y, |a| a.free(addr) == output)
    }

    #[derive(Clone, PartialEq, Eq, Debug)]
    struct TestRegion(Vec<TestTree>);

    fn region<const N: usize>(trees: [TestTree; N]) -> TestRegion {
        TestRegion(trees.to_vec())
    }

    #[derive(Clone, PartialEq, Eq, Debug)]
    struct TestTree(Vec<TestState>);

    fn tree<const N: usize>(states: [TestState; N]) -> TestTree {
        TestTree(states.to_vec())
    }

    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    enum TestState {
        Allocated(u8),
        Free(u8),
    }

    impl TestState {
        fn order(&self) -> u8 {
            match *self {
                TestState::Allocated(order) | TestState::Free(order) => order,
            }
        }
    }

    #[derive(Debug)]
    struct TestAllocator {
        allocator: Option<Pin<Box<Allocator<'static, ()>>>>,
        regions_raw: (*mut Region<'static, ()>, usize, usize),
        pages_raw: (*mut Page<'static, ()>, usize, usize),
    }

    impl Drop for TestAllocator {
        fn drop(&mut self) {
            drop(self.allocator.take());
            drop(unsafe {
                Vec::from_raw_parts(self.regions_raw.0, self.regions_raw.1, self.regions_raw.2)
            });
            drop(unsafe {
                Vec::from_raw_parts(self.pages_raw.0, self.pages_raw.1, self.pages_raw.2)
            });
        }
    }

    impl std::ops::Deref for TestAllocator {
        type Target = Pin<Box<Allocator<'static, ()>>>;

        fn deref(&self) -> &Self::Target {
            self.allocator.as_ref().unwrap()
        }
    }

    impl std::ops::DerefMut for TestAllocator {
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.allocator.as_mut().unwrap()
        }
    }

    #[cfg(not(miri))]
    use std::collections::HashMap;
    #[cfg(miri)]
    type HashMap<K, V> = std::collections::HashMap<
        K,
        V,
        std::hash::BuildHasherDefault<std::collections::hash_map::DefaultHasher>,
    >;
    use std::convert::TryInto;

    #[cfg(not(miri))]
    fn new_hashmap<K: Eq + std::hash::Hash, V>() -> HashMap<K, V> {
        HashMap::new()
    }

    #[cfg(miri)]
    fn new_hashmap<K: Eq + std::hash::Hash, V>() -> HashMap<K, V> {
        HashMap::default()
    }

    fn allocator(
        regions_spec: &[TestRegion],
        free_pages_spec: Option<&[(u8, Vec<usize>)]>,
    ) -> TestAllocator {
        use self::TestState::*;
        use brutos_util::uint::UInt;

        let mut pages = Vec::new();
        let mut page_ranges = Vec::new();
        let mut free_pages = new_hashmap();
        for (region_i, region_spec) in regions_spec.iter().enumerate() {
            let mut page_i = 0;

            let region_page_start = pages.len();
            for tree_spec in &region_spec.0 {
                let tree_size: usize = tree_spec
                    .0
                    .iter()
                    .map(|state| 1usize << state.order())
                    .sum();
                assert!(tree_size.is_power_of_two());
                let tree_order = tree_size.lsb().unwrap().try_into().unwrap();

                for state_spec in &tree_spec.0 {
                    for tree_page_i in 0..(1 << state_spec.order()) {
                        let state = if tree_page_i == 0 {
                            match *state_spec {
                                Allocated(order) => State::Allocated(order),
                                Free(order) => {
                                    if free_pages_spec.is_none() {
                                        free_pages
                                            .entry(order)
                                            .or_insert(Vec::new())
                                            .push(pages.len());
                                    }
                                    State::Free(order)
                                }
                            }
                        } else {
                            State::Unreachable
                        };
                        let page_addr = PhysAddr((region_page_start + page_i) * PAGE_SIZE);
                        pages.push(Page {
                            region: region_i,
                            addr: page_addr,
                            tree_order,
                            state: state.into(),
                            node: Default::default(),
                            data: (),
                        });
                        page_i += 1;
                    }
                }
            }
            let region_page_end = pages.len();
            page_ranges.push(region_page_start..region_page_end);
        }

        let (pages_ptr, pages_len, pages_cap) = Vec::into_raw_parts(pages);
        let pages = unsafe { std::slice::from_raw_parts(pages_ptr, pages_len) };

        let regions = page_ranges
            .iter()
            .map(|range| Region {
                range: PhysAddr(range.start * PAGE_SIZE)..PhysAddr(range.end * PAGE_SIZE),
                pages: &pages[range.start..range.end],
            })
            .collect();
        let (regions_ptr, regions_len, regions_cap) = Vec::into_raw_parts(regions);
        let regions = unsafe { std::slice::from_raw_parts(regions_ptr, regions_len) };

        if let Some(free_pages_spec) = free_pages_spec {
            free_pages = free_pages_spec.iter().cloned().collect();
        }

        let mut allocator = Allocator::new();
        allocator.regions = regions;
        let mut allocator = Box::pin(allocator);
        allocator.as_mut().initialize();
        for (&order, free_pages) in &free_pages {
            for &i in free_pages {
                allocator
                    .as_mut()
                    .index_free_pages(order)
                    .push_back(&pages[i]);
            }
        }

        TestAllocator {
            allocator: Some(allocator),
            regions_raw: (regions_ptr, regions_len, regions_cap),
            pages_raw: (pages_ptr, pages_len, pages_cap),
        }
    }

    fn compare_allocators(a: &TestAllocator, b: &TestAllocator) -> bool {
        assert_eq!(a.regions.len(), b.regions.len());
        for (region_i, (ar, br)) in a.regions.iter().zip(b.regions).enumerate() {
            assert_eq!(ar.pages.len(), br.pages.len());
            for (page_i, (ap, bp)) in ar.pages.iter().zip(br.pages).enumerate() {
                let page_addr = ar.range.start + (page_i * PAGE_SIZE);
                assert_eq!((ap.region, ap.addr), (region_i, page_addr));
                assert_eq!(
                    (ap.region, ap.addr, ap.tree_order),
                    (bp.region, bp.addr, bp.tree_order)
                );
                if ap.state != bp.state {
                    return false;
                }
            }
        }
        for (a, b) in a.free_pages.iter().zip(&b.free_pages) {
            let mut cursors = (a.first(), b.first());
            loop {
                match cursors {
                    (Some(a), Some(b)) if (a.region, a.addr) != (b.region, b.addr) => return false,
                    (Some(a), Some(b)) => cursors = (a.next().ok(), b.next().ok()),
                    (None, None) => break,
                    _ => panic!("Free pages differ"),
                }
            }
        }
        true
    }
}
