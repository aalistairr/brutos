use core::cmp::min;
use core::mem::{align_of, size_of};
use core::ops::Range;
use core::pin::Pin;

use brutos_memory_units::arch::PAGE_SIZE;
use brutos_memory_units::PhysAddr;

use brutos_util::iter::unfold_value;
use brutos_util::uint::UInt;

use super::MAX_ORDER;
use super::{Allocator, Page, Region, State};

pub unsafe trait Context {
    type Err;

    fn map(&mut self, addr: PhysAddr, size: usize, align: usize) -> Result<*mut u8, Self::Err>;
}

#[derive(Debug)]
pub enum Error<MapperErr> {
    NotEnoughMemory,
    Mapper(MapperErr),
}

impl<'a, T: Default> Allocator<'a, T> {
    pub unsafe fn bootstrap<Cx, Spec>(
        self: Pin<&mut Self>,
        cx: &mut Cx,
        spec: Spec,
    ) -> Result<(usize, &'a [Region<'a, T>]), Error<Cx::Err>>
    where
        Cx: Context,
        Spec: Clone + IntoIterator<Item = Range<PhysAddr>>,
    {
        self.bootstrap_(cx, spec)
    }

    fn bootstrap_<Cx, Spec>(
        mut self: Pin<&mut Self>,
        cx: &mut Cx,
        spec: Spec,
    ) -> Result<(usize, &'a [Region<'a, T>]), Error<Cx::Err>>
    where
        Cx: Context,
        Spec: Clone + IntoIterator<Item = Range<PhysAddr>>,
    {
        spec.clone().into_iter().fold(PhysAddr(0), |prev_end, r| {
            assert!(r.start <= r.end);
            assert!(prev_end <= r.start);
            r.end
        });

        // Allocate and initialize `[Region]`
        let regions = {
            let len = spec.clone().into_iter().count();
            let size = len * size_of::<Region<'a, T>>();
            let (storage_info, phys_addr) =
                alloc_in_regions_iter(spec.clone().into_iter(), size, align_of::<Region<'a, T>>())
                    .ok_or(Error::NotEnoughMemory)?;
            let regions = cx
                .map(
                    phys_addr,
                    len * size_of::<Region<'a, T>>(),
                    align_of::<Region<'a, T>>(),
                )
                .map_err(Error::Mapper)?;
            let regions = regions as *mut Region<'a, T>;

            for (i, range) in spec.clone().into_iter().enumerate() {
                unsafe {
                    regions.add(i).write(Region { range, pages: &[] });
                }
            }
            unsafe {
                (*regions.add(storage_info.0)).range = storage_info.1;
                core::slice::from_raw_parts_mut(regions, len)
            }
        };

        // Allocate `[Page]`s
        for (i, range) in spec.clone().into_iter().enumerate() {
            let range = range.start.align_up(PAGE_SIZE)..range.end.align_down(PAGE_SIZE);
            if range.start >= range.end {
                continue;
            }

            let pages_len = (range.end.0 - range.start.0) / PAGE_SIZE;
            let pages_size = pages_len * size_of::<Page<'a, T>>();
            let pages = alloc_in_regions(regions, pages_size, align_of::<Page<'a, T>>())
                .ok_or(Error::NotEnoughMemory)?;
            let pages = cx
                .map(pages, pages_size, align_of::<Page<'a, T>>())
                .map_err(Error::Mapper)?;
            let pages = pages as *mut Page<'a, T>;
            regions[i].pages = unsafe { core::slice::from_raw_parts(pages, 0) };
        }

        // Initialize `Page`s
        let mut free_memory = 0;
        for (region_i, region) in regions.iter_mut().enumerate() {
            let range =
                region.range.start.align_up(PAGE_SIZE)..region.range.end.align_down(PAGE_SIZE);
            if range.start >= range.end {
                region.pages = &[];
                continue;
            }
            region.range = range;
            free_memory += region.range.end.0 - region.range.start.0;

            let pages = region.pages as *const [Page<T>] as *mut Page<T>;
            for (tree_start_page, tree_order) in trees(region.range.clone()) {
                for page_i_in_tree in 0..(1 << tree_order) {
                    let page_i = tree_start_page + page_i_in_tree;
                    let page_addr = region.range.start + (page_i * PAGE_SIZE);
                    unsafe {
                        pages.add(page_i).write(Page {
                            region: region_i,
                            addr: page_addr,
                            tree_order,
                            state: if page_i_in_tree == 0 {
                                State::Free(tree_order)
                            } else {
                                State::Unreachable
                            }
                            .into(),
                            node: Default::default(),
                            data: Default::default(),
                        });

                        if page_i_in_tree == 0 {
                            self.as_mut()
                                .index_free_pages(tree_order)
                                .push_back(&*pages.add(page_i));
                        }
                    }
                }
            }

            let pages_len = (region.range.end.0 - region.range.start.0) / PAGE_SIZE;
            region.pages = unsafe { core::slice::from_raw_parts(pages, pages_len) };
        }

        unsafe {
            self.as_mut().get_unchecked_mut().regions = regions;
        }

        Ok((free_memory, self.regions))
    }
}

fn trees(r: Range<PhysAddr>) -> impl Iterator<Item = (usize, u8)> {
    assert!(r.start.is_aligned(PAGE_SIZE) && r.end.is_aligned(PAGE_SIZE));
    unfold_value((r, 0), |(r, i)| {
        assert!(r.start <= r.end);
        if r.start == r.end {
            return None;
        }
        let order = min(
            MAX_ORDER as u32,
            min(
                (r.start.0 / PAGE_SIZE).lsb().unwrap_or(MAX_ORDER as u32), // alignment
                ((r.end.0 - r.start.0) / PAGE_SIZE).msb().unwrap(),        // size
            ),
        ) as u8;
        let len = 1 << order;
        Some(((r.start + (len * PAGE_SIZE)..r.end, i + len), (i, order)))
    })
}

type AllocInfo = ((usize, Range<PhysAddr>), PhysAddr);
fn alloc_in_regions_iter(
    regions: impl Iterator<Item = Range<PhysAddr>>,
    size: usize,
    align: usize,
) -> Option<AllocInfo> {
    type FRet = Option<(AllocInfo, usize)>;
    fn f(size: usize, align: usize) -> impl Fn(FRet, (usize, Range<PhysAddr>)) -> FRet {
        move |best_match, (i, range)| {
            let range = range.start.align_up(align)..range.end;
            if range.start > range.end || range.end.0 - range.start.0 < size {
                return best_match;
            }
            let addr = range.start;
            let range = range.start + size..range.end;
            let size_left = range.end.0 - range.start.0;
            let this = (((i, range), addr), size_left);
            match best_match {
                None => Some(this),
                Some((_, best_size_left)) if size_left < best_size_left => Some(this),
                x @ Some(_) => x,
            }
        }
    }
    regions.enumerate().fold(None, f(size, align)).map(|x| x.0)
}

fn alloc_in_regions<T>(regions: &mut [Region<T>], size: usize, align: usize) -> Option<PhysAddr> {
    alloc_in_regions_iter(regions.iter().map(|x| x.range.clone()), size, align).map(
        |((region_i, region_range), addr)| {
            regions[region_i].range = region_range;
            addr
        },
    )
}

#[derive(Clone)]
pub struct CutRange<I> {
    cut: Range<PhysAddr>,
    iter: I,
    range: Option<Range<PhysAddr>>,
}

impl<I> CutRange<I> {
    pub fn new(iter: I, cut: Range<PhysAddr>) -> CutRange<I> {
        CutRange {
            iter,
            cut,
            range: None,
        }
    }
}

impl<I: Iterator<Item = Range<PhysAddr>>> Iterator for CutRange<I> {
    type Item = Range<PhysAddr>;

    fn next(&mut self) -> Option<Range<PhysAddr>> {
        loop {
            let range = match self.range.take().or_else(|| self.iter.next()) {
                Some(x) => x,
                None => return None,
            };
            if range.end <= self.cut.start || range.start >= self.cut.end {
                return Some(range);
            } else if range.start < self.cut.start && range.end <= self.cut.end {
                return Some(range.start..self.cut.start);
            } else if range.start >= self.cut.start && range.end > self.cut.end {
                return Some(self.cut.end..range.end);
            } else if range.start < self.cut.start && range.end > self.cut.end {
                self.range = Some(self.cut.end..range.end);
                return Some(range.start..self.cut.start);
            } else {
                continue;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use rand::prelude::*;

    use super::*;

    use crate::Order;

    #[derive(Clone)]
    struct CutRange<I> {
        cut: Range<PhysAddr>,
        iter: I,
        range: Option<Range<PhysAddr>>,
    }

    impl<I> CutRange<I> {
        fn new(iter: I, cut: Range<PhysAddr>) -> CutRange<I> {
            CutRange {
                iter,
                cut,
                range: None,
            }
        }
    }

    impl<I: Iterator<Item = Range<PhysAddr>>> Iterator for CutRange<I> {
        type Item = Range<PhysAddr>;

        fn next(&mut self) -> Option<Range<PhysAddr>> {
            loop {
                let range = match self.range.take().or_else(|| self.iter.next()) {
                    Some(x) => x,
                    None => return None,
                };
                if range.end.0 <= self.cut.start.0 || range.start.0 >= self.cut.end.0 {
                    return Some(range);
                } else if range.start.0 < self.cut.start.0 && range.end.0 <= self.cut.end.0 {
                    return Some(range.start..self.cut.start);
                } else if range.start.0 >= self.cut.start.0 && range.end.0 > self.cut.end.0 {
                    return Some(self.cut.end..range.end);
                } else if range.start.0 < self.cut.start.0 && range.end.0 > self.cut.end.0 {
                    self.range = Some(self.cut.end..range.end);
                    return Some(range.start..self.cut.start);
                } else {
                    continue;
                }
            }
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

    #[cfg(not(miri))]
    fn new_hashmap<K: Eq + std::hash::Hash, V>() -> HashMap<K, V> {
        HashMap::new()
    }

    #[cfg(miri)]
    fn new_hashmap<K: Eq + std::hash::Hash, V>() -> HashMap<K, V> {
        HashMap::default()
    }

    struct Mapper<I> {
        memory: HashMap<Range<PhysAddr>, (*mut u8, std::alloc::Layout)>,
        valid_ranges: I,
    }

    impl<I> Mapper<I> {
        fn new(valid_ranges: I) -> Mapper<I> {
            Mapper {
                memory: new_hashmap(),
                valid_ranges,
            }
        }
    }

    unsafe impl<I> Context for Mapper<I>
    where
        I: Clone + Iterator<Item = Range<PhysAddr>>,
    {
        type Err = !;

        fn map(&mut self, addr: PhysAddr, size: usize, align: usize) -> Result<*mut u8, !> {
            let range = addr..PhysAddr(addr.0 + size);
            for existing_range in self.memory.keys() {
                assert!(range.start >= existing_range.end || range.end <= existing_range.start);
            }
            assert!(self
                .valid_ranges
                .clone()
                .fold(false, |is_valid, valid_range| {
                    is_valid | (range.start >= valid_range.start && range.end <= valid_range.end)
                }));
            let layout = std::alloc::Layout::from_size_align(size, align).unwrap();
            let ptr = unsafe { std::alloc::alloc(layout) };
            self.memory.insert(range, (ptr, layout));
            Ok(ptr)
        }
    }

    impl<I> Drop for Mapper<I> {
        fn drop(&mut self) {
            self.memory
                .values()
                .for_each(|&(ptr, layout)| unsafe { std::alloc::dealloc(ptr, layout) });
        }
    }

    fn cut_ranges(
        mmap: impl Clone + Iterator<Item = Range<PhysAddr>>,
    ) -> impl Clone + Iterator<Item = Range<PhysAddr>> {
        let mmap = CutRange::new(mmap, PhysAddr(0x0)..PhysAddr(0x500));
        let mmap = CutRange::new(mmap, PhysAddr(0x7c00)..PhysAddr(0x7e00));
        let mmap = CutRange::new(mmap, PhysAddr(0x9fc00)..PhysAddr(0x100000));
        let mmap = CutRange::new(mmap, PhysAddr(0xf00000)..PhysAddr(0x1000000));
        let mmap = CutRange::new(mmap, PhysAddr(0xc0000000)..PhysAddr(0x100000000));
        mmap
    }

    fn test_mmap(mmap: &[Range<PhysAddr>]) -> Result<(), super::super::NotAllocated> {
        let mmap = cut_ranges(mmap.iter().cloned());
        let mut mapper = Mapper::new(mmap.clone());

        let mut allocator = Box::pin(Allocator::<()>::new());
        allocator.as_mut().initialize();
        let (free_memory, _) = unsafe { allocator.as_mut().bootstrap(&mut mapper, mmap).unwrap() };
        eprintln!("Available memory: {:#x}", free_memory);

        allocator.as_mut().shuffle_free_pages();

        let mut pages = Vec::new();
        while let Some((addr, &())) = allocator.as_mut().allocate(Order(0)).unwrap() {
            eprintln!("Allocated {:?}", addr);
            pages.push(addr);
        }

        pages.shuffle(&mut rand::thread_rng());

        for addr in pages.into_iter() {
            eprintln!("Freeing {:?}", addr);
            allocator.as_mut().free(addr)?;
        }

        Ok(())
    }

    #[test]
    fn test_4m() -> Result<(), super::super::NotAllocated> {
        test_mmap(&[
            PhysAddr(0x0)..PhysAddr(0x9f800),
            PhysAddr(0x100000)..PhysAddr(0x100000 + 0x1e0000),
            PhysAddr(0x300000)..PhysAddr(0x300000 + 0x100000),
        ])
    }
}
