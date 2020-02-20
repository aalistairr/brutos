use core::ops::Range;
use core::pin::Pin;
use core::ptr::NonNull;

use crate::Cx;
use brutos_alloc::{AllocOne, ArcInner, OutOfMemory};
use brutos_memory_phys_alloc::{self as phys_alloc, Allocator as PhysAllocator};
use brutos_memory_slab_alloc as slab;
use brutos_memory_traits::AllocPhysPage;
use brutos_memory_units::{Order, PhysAddr};
use brutos_sync::mutex::PinMutex;

use super::PageData;

pub fn phys_allocator() -> Pin<&'static PinMutex<PhysAllocator<'static, PageData>, Cx>> {
    static PHYS_ALLOCATOR: PinMutex<PhysAllocator<PageData>, Cx> =
        PinMutex::new(PhysAllocator::new());
    unsafe { Pin::new_unchecked(&PHYS_ALLOCATOR) }
}

static mut REGIONS: &'static [phys_alloc::Region<'static, PageData>] = &[];

pub fn initialize() {
    phys_allocator().initialize();
    phys_allocator().lock().as_mut().initialize();
    initialize_addr_space_allocator();
    initialize_mapping_allocator();
    initialize_task_allocator();
}

#[derive(Debug)]
pub struct FailedToBootstrap;

pub unsafe fn bootstrap(
    mmap: impl Clone + Iterator<Item = Range<PhysAddr>>,
) -> Result<usize, phys_alloc::bootstrap::Error<FailedToBootstrap>> {
    let (free_space, regions) = phys_allocator()
        .as_ref()
        .lock()
        .as_mut()
        .bootstrap(&mut Cx, mmap)?;
    REGIONS = regions;
    Ok(free_space)
}

pub fn get_data(addr: PhysAddr) -> Result<&'static PageData, phys_alloc::NotAllocated> {
    let regions = unsafe { REGIONS };
    phys_alloc::get_data(regions, addr)
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

unsafe impl AllocPhysPage for Cx {
    const MAX_ORDER: Order = Order(brutos_memory_phys_alloc::MAX_ORDER);

    type PageData = PageData;

    fn alloc(order: Order) -> Result<(PhysAddr, &'static Self::PageData), ()> {
        phys_allocator()
            .lock()
            .as_mut()
            .allocate(order)
            .expect("Allocation is too large")
            .ok_or(())
    }

    unsafe fn dealloc(addr: PhysAddr, _: Order) {
        phys_allocator()
            .lock()
            .as_mut()
            .free(addr)
            .expect("Failed to deallocate")
    }

    fn get_data(addr: PhysAddr) -> &'static Self::PageData {
        get_data(addr).expect("Address is not allocated")
    }
}

macro_rules! slab_allocator {
    ($n:ident, $o:expr, $t:ty) => {
        fn $n() -> Pin<&'static PinMutex<slab::Allocator<Cx>, Cx>> {
            static ALLOCATOR: PinMutex<slab::Allocator<Cx>, Cx> =
                PinMutex::new(slab::Allocator::new::<$t>(Order($o)));
            unsafe { Pin::new_unchecked(&ALLOCATOR) }
        }

        unsafe impl AllocOne<$t> for Cx {
            unsafe fn alloc(&mut self) -> Result<NonNull<$t>, OutOfMemory> {
                $n().lock().as_mut().alloc().map(NonNull::cast)
            }
            unsafe fn dealloc(&mut self, ptr: NonNull<$t>) {
                $n().lock().as_mut().dealloc(ptr.cast())
            }
        }
    };
}

slab_allocator!(task_allocator, 1, ArcInner<brutos_task::Task<Cx>>);
fn initialize_task_allocator() {
    task_allocator().initialize();
    task_allocator().lock().as_mut().initialize();
}
slab_allocator!(
    mapping_allocator,
    1,
    ArcInner<brutos_memory_vm::Mapping<Cx>>
);
fn initialize_mapping_allocator() {
    mapping_allocator().initialize();
    mapping_allocator().lock().as_mut().initialize();
}
slab_allocator!(
    addr_space_allocator,
    1,
    ArcInner<crate::memory::addr_space::AddressSpace>
);
fn initialize_addr_space_allocator() {
    addr_space_allocator().initialize();
    addr_space_allocator().lock().as_mut().initialize();
}
