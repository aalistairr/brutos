use core::ops::Range;
use core::pin::Pin;

use brutos_alloc::{AllocOne, Arc, ArcInner};
use brutos_util::linked_list::{LinkedList, Node, Transient, TransientMut};

use crate::arch::PAGE_SIZE;
use crate::VirtAddr;

pub trait Context<T>: Default + AllocOne<ArcInner<Mapping<T, Self>>> {}

impl<T, Cx> Context<T> for Cx where Cx: Default + AllocOne<ArcInner<Mapping<T, Self>>> {}

pub struct Mappings<T, Cx: Context<T>> {
    range: Range<VirtAddr>,
    mappings: LinkedList<MappingSel<T, Cx>>,
}

brutos_util_macros::selector!(MappingSel<T, Cx: Context<T>>: Arc<Mapping<T, Cx>, Cx> => node);
pub struct Mapping<T, Cx: Context<T>> {
    pub range: Range<VirtAddr>,
    pub flags: Flags,
    data: T,
    node: Node<MappingSel<T, Cx>>,
}

impl<T, Cx: Context<T>> core::ops::Deref for Mapping<T, Cx> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MapError {
    OutOfMemory,
    OutsideSpaceRange,
    OutOfSpace,
    InvalidParameters,
}

impl From<brutos_alloc::OutOfMemory> for MapError {
    fn from(brutos_alloc::OutOfMemory: brutos_alloc::OutOfMemory) -> MapError {
        MapError::OutOfMemory
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum UnmapError {
    NotStartOfMapping,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Location {
    Aligned(usize),
    Fixed(VirtAddr),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Flags {
    pub guard_pages: bool,
}

impl<T, Cx: Context<T>> Mappings<T, Cx> {
    pub fn new(range: Range<VirtAddr>) -> Mappings<T, Cx> {
        assert!(range.start <= range.end);
        Mappings {
            range,
            mappings: LinkedList::new(),
        }
    }

    fn deconstruct<'a>(
        self: Pin<&'a mut Self>,
    ) -> (&Range<VirtAddr>, Pin<&'a mut LinkedList<MappingSel<T, Cx>>>) {
        unsafe {
            let this = Pin::into_inner_unchecked(self);
            (&this.range, Pin::new_unchecked(&mut this.mappings))
        }
    }

    pub fn initialize(self: Pin<&mut Self>) {
        let (_, mappings) = self.deconstruct();
        mappings.initialize();
    }

    pub fn find(&self, at: VirtAddr) -> Option<Transient<Pin<Arc<Mapping<T, Cx>, Cx>>>> {
        let mut next_mapping = self.mappings.first();
        while let Some(mapping) = next_mapping.take() {
            if at >= mapping.range.start && at < mapping.range.end {
                return Some(mapping.get());
            }
            next_mapping = mapping.next().ok();
        }
        None
    }

    pub fn create(
        self: Pin<&mut Self>,
        size: usize,
        at: Location,
        flags: Flags,
        data: T,
    ) -> Result<TransientMut<Pin<Arc<Mapping<T, Cx>, Cx>>>, MapError> {
        let (self_range, mut mappings) = self.deconstruct();
        let mut data = Some(data);
        let data = &mut data;
        match mappings.as_mut().first_mut() {
            None => new_mapping(self_range.clone(), size, at, flags, data)?
                .ok_or(MapError::OutOfSpace)
                .map(|m| {
                    let mut m = mappings.push_back(m);
                    unsafe { m.get_mut().transmute_lt() }
                }),
            Some(mapping) => {
                let free_range = self_range.start..mapping.range.start;
                let free_range = cut_free_range(free_range, None, Some(mapping.flags), flags);
                if let Some(m) = new_mapping(free_range, size, at, flags, data)? {
                    let mut m = mapping.insert_before_and_get(m);
                    return Ok(unsafe { m.get_mut().transmute_lt() });
                }

                let mut next_mapping = Some(mapping);
                while let Some(mapping) = next_mapping.take() {
                    let (next_start, next_flags) = mapping
                        .peek_next()
                        .map(|m| (m.range.start, Some(m.flags)))
                        .unwrap_or((self_range.end, None));
                    let free_range = mapping.range.end..next_start;
                    let free_range =
                        cut_free_range(free_range, Some(mapping.flags), next_flags, flags);
                    if let Some(m) = new_mapping(free_range, size, at, flags, data)? {
                        let mut m = mapping.insert_after_and_get(m);
                        return Ok(unsafe { m.get_mut().transmute_lt() });
                    }
                    next_mapping = mapping.next().ok();
                }
                Err(MapError::OutOfSpace)
            }
        }
    }

    pub fn remove(
        self: Pin<&mut Self>,
        at: VirtAddr,
    ) -> Result<Pin<Arc<Mapping<T, Cx>, Cx>>, UnmapError> {
        let (_, mappings) = self.deconstruct();
        let mut next_mapping = mappings.first_mut();
        while let Some(mapping) = next_mapping.take() {
            if mapping.range.start == at {
                return Ok(mapping.unlink());
            }
            next_mapping = mapping.next().ok();
        }
        Err(UnmapError::NotStartOfMapping)
    }
}

fn cut_free_range(
    free_range: Range<VirtAddr>,
    before: Option<Flags>,
    after: Option<Flags>,
    new: Flags,
) -> Range<VirtAddr> {
    let start = if new.guard_pages || before.map(|f| f.guard_pages).unwrap_or(false) {
        free_range.start.checked_add(PAGE_SIZE)
    } else {
        Some(free_range.start)
    };
    let end = if new.guard_pages || after.map(|f| f.guard_pages).unwrap_or(false) {
        free_range.end.checked_sub(PAGE_SIZE)
    } else {
        Some(free_range.end)
    };
    match (start, end) {
        (Some(start), Some(end)) => start..end,
        _ => VirtAddr(0)..VirtAddr(0),
    }
}

fn available_addr(free_range: Range<VirtAddr>, size: usize, at: Location) -> Option<VirtAddr> {
    match at {
        Location::Aligned(align) => {
            let free_range = free_range.start.checked_align_up(align)?..free_range.end;
            if free_range.start <= free_range.end && free_range.end - free_range.start >= size {
                Some(free_range.start)
            } else {
                None
            }
        }
        Location::Fixed(fixed) => {
            if fixed >= free_range.start && fixed.checked_add(size)? <= free_range.end {
                Some(fixed)
            } else {
                None
            }
        }
    }
}

fn new_mapping<T, Cx: Context<T>>(
    free_range: Range<VirtAddr>,
    size: usize,
    at: Location,
    flags: Flags,
    data: &mut Option<T>,
) -> Result<Option<Pin<Arc<Mapping<T, Cx>, Cx>>>, MapError> {
    available_addr(free_range, size, at)
        .map(|addr| {
            Arc::pin(Mapping {
                range: addr..addr + size,
                data: data.take().unwrap(),
                node: Node::new(),
                flags,
            })
            .map_err(|(e, _)| e.into())
        })
        .transpose()
}

impl<T, Cx: Context<T>> Mapping<T, Cx> {
    pub fn data(self: Pin<&Self>) -> Pin<&T> {
        unsafe { self.map_unchecked(|x| &x.data) }
    }
}
