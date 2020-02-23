use core::ops::Range;
use core::pin::Pin;

use brutos_alloc::Arc;
use brutos_memory_units::arch::PAGE_SIZE;
use brutos_memory_units::VirtAddr;
use brutos_util::linked_list::{LinkedList, Transient};

use super::{
    Context, CreateError, DestroyErr, DestroyError, Flags, Location, Mapping, MappingFlags,
    MappingSel,
};

pub struct Mappings<Cx: Context> {
    range: Range<VirtAddr>,
    mappings: LinkedList<MappingSel<Cx>>,
}

impl<Cx: Context> Mappings<Cx> {
    pub fn new(range: Range<VirtAddr>) -> Mappings<Cx> {
        Mappings {
            range,
            mappings: LinkedList::new(),
        }
    }

    pub fn initialize(self: Pin<&mut Self>) {
        let (_, mappings) = self.deconstruct();
        mappings.initialize();
    }

    fn deconstruct(
        self: Pin<&mut Self>,
    ) -> (&Range<VirtAddr>, Pin<&mut LinkedList<MappingSel<Cx>>>) {
        unsafe {
            let this = Pin::into_inner_unchecked(self);
            (&this.range, Pin::new_unchecked(&mut this.mappings))
        }
    }

    pub fn find(&self, addr: VirtAddr) -> Option<Transient<Pin<Arc<Mapping<Cx>, Cx>>>> {
        let mut next_mapping = self.mappings.first();
        while let Some(mapping) = next_mapping.take() {
            if addr >= mapping.range.start && addr < mapping.range.end {
                return Some(mapping.get());
            }
            next_mapping = mapping.next().ok();
        }
        None
    }

    pub fn add<F>(
        self: Pin<&mut Self>,
        size: usize,
        at: Location,
        flags: MappingFlags,
        f: F,
    ) -> Result<Pin<Arc<Mapping<Cx>, Cx>>, CreateError>
    where
        F: FnOnce(Range<VirtAddr>) -> Result<Pin<Arc<Mapping<Cx>, Cx>>, CreateError>,
    {
        let mut f = Some(f);
        let (self_range, mut mappings) = self.deconstruct();
        match mappings.as_mut().first_mut() {
            None => {
                let new_mapping = new_mapping(self_range.clone(), size, at, &mut f)?
                    .ok_or(CreateError::NoSpace)?;
                Ok(mappings.push_back(new_mapping).get().clone())
            }
            Some(mapping) => {
                let free_range = self_range.start..mapping.range.start;
                let free_range = cut_free_range(free_range, None, Some(mapping.flags), flags);
                if let Some(new_mapping) = new_mapping(free_range, size, at, &mut f)? {
                    return Ok(mapping.insert_before_and_get(new_mapping).get().clone());
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
                    if let Some(new_mapping) = new_mapping(free_range, size, at, &mut f)? {
                        return Ok(mapping.insert_after_and_get(new_mapping).get().clone());
                    }
                    next_mapping = mapping.next().ok();
                }
                Err(CreateError::NoSpace)
            }
        }
    }

    pub fn remove(
        self: Pin<&mut Self>,
        addr: VirtAddr,
    ) -> Result<Pin<Arc<Mapping<Cx>, Cx>>, DestroyError<Cx>> {
        let (_, mappings) = self.deconstruct();
        let mut next_mapping = mappings.first_mut();
        while let Some(mapping) = next_mapping.take() {
            if addr >= mapping.range.start && addr < mapping.range.end {
                return Ok(mapping.unlink());
            }
            next_mapping = mapping.next().ok();
        }
        Err(DestroyErr::MappingNotFound)
    }
}

fn cut_free_range<CacheType>(
    free_range: Range<VirtAddr>,
    before: Option<Flags<CacheType>>,
    after: Option<Flags<CacheType>>,
    new: MappingFlags,
) -> Range<VirtAddr> {
    let start = if new.guarded || before.map(|f| f.mapping.guarded).unwrap_or(false) {
        free_range.start.checked_add(PAGE_SIZE)
    } else {
        Some(free_range.start)
    };
    let end = if new.guarded || after.map(|f| f.mapping.guarded).unwrap_or(false) {
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

fn new_mapping<Cx, F>(
    free_range: Range<VirtAddr>,
    size: usize,
    at: Location,
    create_mapping: &mut Option<F>,
) -> Result<Option<Pin<Arc<Mapping<Cx>, Cx>>>, CreateError>
where
    Cx: Context,
    F: FnOnce(Range<VirtAddr>) -> Result<Pin<Arc<Mapping<Cx>, Cx>>, CreateError>,
{
    available_addr(free_range, size, at)
        .map(|addr| (create_mapping.take().unwrap())(addr..addr + size))
        .transpose()
}
