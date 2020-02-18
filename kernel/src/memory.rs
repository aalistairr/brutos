use core::ops::Range;
use core::pin::Pin;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicBool, Ordering};

use brutos_alloc::{AllocOne, Arc, ArcInner, OutOfMemory};
use brutos_memory_phys_alloc as phys_alloc;
use brutos_memory_phys_alloc::Allocator as PhysAllocator;
use brutos_memory_slab_alloc as slab;
use brutos_memory_traits::{AllocMappedPage, AllocPhysPage, MapPhysPage};
use brutos_memory_units::{Order, PhysAddr};
use brutos_memory_vm as vm;
use brutos_sync::mutex::{Mutex, PinMutex};
use brutos_task::Task;

use crate::Cx;

#[derive(Default)]
pub struct PageData {
    ref_count: vm::PageRefCount,
}

impl AsRef<vm::PageRefCount> for PageData {
    fn as_ref(&self) -> &vm::PageRefCount {
        &self.ref_count
    }
}

pub fn phys_allocator() -> Pin<&'static PinMutex<PhysAllocator<'static, PageData>, Cx>> {
    static PHYS_ALLOCATOR: PinMutex<PhysAllocator<PageData>, Cx> =
        PinMutex::new(PhysAllocator::new());
    unsafe { Pin::new_unchecked(&PHYS_ALLOCATOR) }
}

static mut REGIONS: &'static [phys_alloc::Region<'static, PageData>] = &[];

pub fn initialize() {
    phys_allocator().initialize();
    phys_allocator().lock().as_mut().initialize();
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

unsafe impl AllocMappedPage for Cx {
    const MAX_ORDER: Order = Order(brutos_memory_phys_alloc::MAX_ORDER);

    fn alloc(order: Order) -> Result<NonNull<u8>, ()> {
        phys_allocator()
            .lock()
            .as_mut()
            .allocate(order)
            .expect("Allocation is too large")
            .ok_or(())
            .and_then(|(addr, _)| crate::arch::memory::map_phys_ident(addr, order.size()))
    }

    unsafe fn dealloc(ptr: NonNull<u8>, _: Order) {
        phys_allocator()
            .lock()
            .as_mut()
            .free(crate::arch::memory::phys_ident_addr(ptr))
            .expect("Failed to deallocate")
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

slab_allocator!(task_allocator, 1, ArcInner<Task<Cx>>);
pub fn initialize_task_allocator() {
    task_allocator().initialize();
    task_allocator().lock().as_mut().initialize();
}
slab_allocator!(mapping_allocator, 1, ArcInner<vm::Mapping<Cx>>);
pub fn initialize_mapping_allocator() {
    mapping_allocator().initialize();
    mapping_allocator().lock().as_mut().initialize();
}
slab_allocator!(addr_space_allocator, 1, ArcInner<AddressSpace>);
pub fn initialize_addr_space_allocator() {
    addr_space_allocator().initialize();
    addr_space_allocator().lock().as_mut().initialize();
}

pub struct AddressSpace {
    is_alive: AtomicBool,
    vm: vm::Space<Cx>,
}

static mut KERNEL_ADDR_SPACE: core::mem::MaybeUninit<Pin<Arc<AddressSpace, Cx>>> =
    core::mem::MaybeUninit::uninit();

pub unsafe fn create_kernel_address_space() -> Result<(), OutOfMemory> {
    let addr_space = KERNEL_ADDR_SPACE.write(
        Arc::pin(AddressSpace {
            is_alive: AtomicBool::new(true),
            vm: vm::Space::new(
                crate::arch::memory::KERNEL_ADDR_SPACE_RANGE,
                crate::arch::memory::create_kernel_mmu_tables()?,
            ),
        })
        .map_err(|(e, space)| {
            crate::arch::memory::destroy_kernel_mmu_tables(Mutex::into_inner(space.vm.mmu_tables));
            e
        })?,
    );
    addr_space.vm().initialize();
    crate::arch::memory::create_kernel_mappings(addr_space);
    Ok(())
}

pub fn create_user_address_space() -> Result<Pin<Arc<AddressSpace, Cx>>, vm::mmu::MapError> {
    let addr_space = Arc::pin(AddressSpace {
        is_alive: AtomicBool::new(true),
        vm: vm::Space::new(crate::arch::memory::USER_ADDR_SPACE_RANGE, unsafe {
            crate::arch::memory::create_user_mmu_tables()?
        }),
    })
    .map_err(|(e, space)| {
        unsafe {
            crate::arch::memory::destroy_user_mmu_tables(Mutex::into_inner(space.vm.mmu_tables));
        }
        e
    })?;
    addr_space.vm().initialize();
    Ok(addr_space)
}

impl AddressSpace {
    pub unsafe fn kernel() -> &'static Pin<Arc<AddressSpace, Cx>> {
        &*KERNEL_ADDR_SPACE.as_ptr()
    }

    pub fn vm<'a>(self: &'a Pin<Arc<AddressSpace, Cx>>) -> Pin<&'a vm::Space<Cx>> {
        unsafe { self.as_ref().map_unchecked(|x| &x.vm) }
    }

    pub fn kill(&self) {
        self.is_alive.store(false, Ordering::Release);
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive.load(Ordering::Acquire)
    }
}

unsafe impl MapPhysPage for Cx {
    type Err = ();

    unsafe fn with_mapped_page<F, R>(addr: PhysAddr, order: Order, f: F) -> Result<R, Self::Err>
    where
        F: FnOnce(*mut u8) -> R,
    {
        crate::arch::memory::map_phys_ident(addr, order.size())
            .map(NonNull::as_ptr)
            .map(f)
    }
}
