use core::ops::Range;
use core::pin::Pin;

use brutos_memory::phys_alloc::bootstrap;
use brutos_memory::phys_alloc::Allocator as PhysAllocator;
use brutos_memory::PhysAddr;
use brutos_sync::mutex::PinMutex;

use crate::Cx;

static PHYS_ALLOCATOR: PinMutex<PhysAllocator<Cx>, Cx> = PinMutex::new(PhysAllocator::new());

fn phys_allocator() -> Pin<&'static PinMutex<PhysAllocator<'static, Cx>, Cx>> {
    unsafe { Pin::new_unchecked(&PHYS_ALLOCATOR) }
}

pub fn initialize() {
    phys_allocator().initialize();
    phys_allocator().lock().as_mut().initialize();
}

#[derive(Debug)]
pub struct FailedToBootstrap;

pub unsafe fn bootstrap(
    mmap: impl Clone + Iterator<Item = Range<PhysAddr>>,
) -> Result<usize, bootstrap::Error<FailedToBootstrap>> {
    phys_allocator()
        .as_ref()
        .lock()
        .as_mut()
        .bootstrap(&mut Cx, mmap)
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
