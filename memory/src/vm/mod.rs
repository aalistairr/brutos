use core::ops::Range;
use core::pin::Pin;
use core::sync::atomic::{AtomicUsize, Ordering};

use brutos_alloc::Arc;
use brutos_sync::condvar::Condvar;
use brutos_sync::mutex::{Mutex, PinMutex};
use brutos_sync::waitq;
use brutos_util::{Guard, UInt};

use crate::{AllocPhysPage, MapPhysPage};
use crate::{Order, PhysAddr, VirtAddr};

pub mod mappings;
pub mod mmu;

pub use self::mappings::Location;
use self::mappings::{MapError, UnmapError};

pub type Mapping<Cx> = mappings::Mapping<MappingData<Cx>, Cx>;
pub type ArcMapping<Cx> = Arc<Mapping<Cx>, Cx>;
pub type Mappings<Cx> = mappings::Mappings<MappingData<Cx>, Cx>;

pub trait Context:
    Default
    + brutos_sync::Critical
    + mappings::Context<MappingData<Self>>
    + mmu::arch::Context
    + AllocPhysPage
    + MapPhysPage
    + waitq::Context
where
    Self::PageData: AsRef<PageRefCount>,
{
    fn shared_empty_page(&mut self, order: Order) -> Option<(PhysAddr, &Self::PageData)>;
}

pub struct Space<Cx: Context>
where
    Cx::PageData: AsRef<PageRefCount>,
{
    mappings: PinMutex<Mappings<Cx>, Cx>,
    pub mmu_tables: Mutex<mmu::Tables, Cx>,
}

pub struct MappingData<Cx: Context>
where
    Cx::PageData: AsRef<PageRefCount>,
{
    status: Mutex<Status, Cx>,
    status_condvar: Condvar<Cx>,
    src: Source,
    page_size: mmu::PageSize,
    mmu_flags: mmu::Flags,
}

#[derive(Clone)]
pub enum Source {
    Raw(PhysAddr),
    Private(Object),
}

#[derive(Clone)]
pub enum Object {
    Anonymous,
}

enum Status {
    Ready,
    Destroyed,
    Busy(usize),
}

impl Status {
    fn is_busy(&self) -> bool {
        match self {
            Status::Busy(_) => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Flags {
    pub mapping: mappings::Flags,
    pub mmu: mmu::Flags,
}

pub struct MappingWasDestroyed;

pub enum FillError<MapPhysPageErr> {
    Map(mmu::MapError),
    MappingWasDestroyed,
    OutOfMemory,
    MapPhysPage(MapPhysPageErr),
}

impl<MapPhysPageErr> From<mmu::MapError> for FillError<MapPhysPageErr> {
    fn from(e: mmu::MapError) -> FillError<MapPhysPageErr> {
        FillError::Map(e)
    }
}

impl<MapPhysPageErr> From<MappingWasDestroyed> for FillError<MapPhysPageErr> {
    fn from(MappingWasDestroyed: MappingWasDestroyed) -> FillError<MapPhysPageErr> {
        FillError::MappingWasDestroyed
    }
}

pub enum PageFaultError<MapPhysPageErr> {
    InvalidAccess,
    Fill(FillError<MapPhysPageErr>),
}

impl<MapPhysPageErr> From<MappingWasDestroyed> for PageFaultError<MapPhysPageErr> {
    fn from(MappingWasDestroyed: MappingWasDestroyed) -> PageFaultError<MapPhysPageErr> {
        PageFaultError::Fill(FillError::MappingWasDestroyed)
    }
}

impl<Cx: Context> MappingData<Cx>
where
    Cx::PageData: AsRef<PageRefCount>,
{
    fn status(self: Pin<&Self>) -> Pin<&Mutex<Status, Cx>> {
        unsafe { self.map_unchecked(|x| &x.status) }
    }

    fn status_condvar(self: Pin<&Self>) -> Pin<&Condvar<Cx>> {
        unsafe { self.map_unchecked(|x| &x.status_condvar) }
    }
}

#[derive(Default)]
pub struct PageRefCount(AtomicUsize);

impl PageRefCount {
    pub fn inc(&self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

pub struct FaultConditions {
    pub was_present: bool,
    pub was_write: bool,
    pub was_instruction_fetch: bool,
    pub was_user_access: bool,
}

impl<Cx: Context> Space<Cx>
where
    Cx::PageData: AsRef<PageRefCount>,
{
    pub fn new(range: Range<VirtAddr>, mmu_tables: mmu::Tables) -> Space<Cx> {
        Space {
            mappings: PinMutex::new(Mappings::new(range)),
            mmu_tables: Mutex::new(mmu_tables),
        }
    }

    fn mappings(self: Pin<&Self>) -> Pin<&PinMutex<Mappings<Cx>, Cx>> {
        unsafe { self.map_unchecked(|x| &x.mappings) }
    }

    fn mmu_tables(self: Pin<&Self>) -> Pin<&Mutex<mmu::Tables, Cx>> {
        unsafe { self.map_unchecked(|x| &x.mmu_tables) }
    }

    pub fn initialize(self: Pin<&Self>) {
        self.mappings().initialize();
        self.mmu_tables().initialize();
        self.mappings().lock().as_mut().initialize();
    }

    pub fn create_mapping(
        self: Pin<&Self>,
        size: usize,
        at: Location,
        src: Source,
        page_size: mmu::PageSize,
        flags: Flags,
    ) -> Result<Pin<Arc<Mapping<Cx>, Cx>>, MapError> {
        assert!(size.is_aligned(page_size.order().size()));
        match at {
            Location::Aligned(align) => assert!(align.is_aligned(page_size.order().size())),
            Location::Fixed(addr) => assert!(addr.is_aligned(page_size.order().size())),
        }
        Ok(self
            .mappings()
            .lock()
            .as_mut()
            .create(
                size,
                at,
                flags.mapping,
                MappingData {
                    status: Mutex::new(Status::Ready),
                    status_condvar: Condvar::new(),
                    src,
                    page_size,
                    mmu_flags: flags.mmu,
                },
            )?
            .clone())
    }

    pub fn remove_mapping(self: Pin<&Self>, mapping_addr: VirtAddr) -> Result<(), UnmapError> {
        let mapping = self
            .mappings()
            .lock()
            .find(mapping_addr)
            .ok_or(UnmapError::NotStartOfMapping)?
            .clone();

        let status = mapping.as_ref().data().status();
        let status_condvar = mapping.as_ref().data().status_condvar();
        match &mut *status_condvar.wait_while(status.lock(), |s| s.is_busy()) {
            Status::Busy(_) => unreachable!(),
            Status::Destroyed => return Ok(()),
            status @ Status::Ready => *status = Status::Destroyed,
        }

        let mut cx = Cx::default();
        let mmu_tables = self.mmu_tables();
        for offset in mapping.page_offsets() {
            let addr = mapping.range.start + offset;
            match mmu_tables.lock().unmap(&mut cx, addr, mapping.page_size) {
                Ok(Some(page)) => unsafe { Self::destroy_page(mapping.as_ref(), offset, page) },
                Ok(None) | Err(mmu::UnmapError::NotAllocated) => (),
                Err(mmu::UnmapError::Obstructed) => panic!("Corrupt page tables"),
            }
        }

        self.mappings().lock().as_mut().remove(mapping_addr)?;
        Ok(())
    }

    fn generate_page(
        mapping: Pin<&Mapping<Cx>>,
        offset: usize,
    ) -> Result<(PhysAddr, mmu::Flags), FillError<<Cx as MapPhysPage>::Err>> {
        let mut cx = Cx::default();
        match mapping.src {
            Source::Raw(addr) => Ok((addr + offset, mapping.mmu_flags)),
            Source::Private(Object::Anonymous) => {
                match cx.shared_empty_page(mapping.page_size.order()) {
                    Some((page, page_data)) => {
                        page_data.as_ref().0.fetch_add(1, Ordering::Release);
                        Ok((
                            page,
                            mmu::Flags {
                                writable: false,
                                ..mapping.mmu_flags
                            },
                        ))
                    }
                    None => {
                        let (page, page_data) =
                            <Cx as AllocPhysPage>::alloc(mapping.page_size.order())
                                .map_err(|()| FillError::OutOfMemory)?;
                        page_data.as_ref().0.store(1, Ordering::Release);
                        let page_guard = Guard::new(|| unsafe {
                            <Cx as AllocPhysPage>::dealloc(page, mapping.page_size.order());
                        });
                        unsafe {
                            Cx::write_bytes(page, 0u8, mapping.page_size.order())
                                .map_err(FillError::MapPhysPage)?;
                        }
                        page_guard.success();
                        Ok((page, mapping.mmu_flags))
                    }
                }
            }
        }
    }

    unsafe fn destroy_allocated_page(
        mapping: Pin<&Mapping<Cx>>,
        page: PhysAddr,
        page_data: &Cx::PageData,
    ) {
        if page_data.as_ref().0.fetch_sub(1, Ordering::Release) == 1 {
            <Cx as AllocPhysPage>::dealloc(page, mapping.page_size.order());
        }
    }

    unsafe fn destroy_page(mapping: Pin<&Mapping<Cx>>, _offset: usize, page: PhysAddr) {
        match mapping.src {
            Source::Raw(_) => (),
            Source::Private(Object::Anonymous) => {
                Self::destroy_allocated_page(mapping, page, Cx::get_data(page))
            }
        }
    }

    fn fill(
        self: Pin<&Self>,
        mapping: Pin<&Mapping<Cx>>,
        offset: usize,
    ) -> Result<(), FillError<<Cx as MapPhysPage>::Err>> {
        let addr = mapping.range.start + offset;
        let (page, flags) = Self::generate_page(mapping, offset)?;
        let did_map = self.mmu_tables().lock().map_keep(
            &mut Cx::default(),
            addr,
            page,
            mapping.page_size,
            true,
            flags,
        )?;
        if !did_map {
            unsafe {
                Self::destroy_page(mapping, offset, page);
            }
        }
        Ok(())
    }

    fn do_cow(
        self: Pin<&Self>,
        mapping: Pin<&Mapping<Cx>>,
        offset: usize,
    ) -> Result<(), FillError<<Cx as MapPhysPage>::Err>> {
        assert!(mapping.is_cow());

        let addr = mapping.range.start + offset;

        let mut mmu_tables = self.mmu_tables().lock();

        let ro_entry = match mmu_tables.get(&mut Cx::default(), addr, mapping.page_size)? {
            None => return Ok(()),
            Some(page) => page,
        };
        let ro_page = ro_entry.address();
        let ro_page_data = <Cx as AllocPhysPage>::get_data(ro_page);

        if ro_page_data.as_ref().0.load(Ordering::Acquire) == 1 {
            mmu_tables
                .map_replace(
                    &mut Cx::default(),
                    addr,
                    ro_page,
                    mapping.page_size,
                    false,
                    mapping.mmu_flags,
                )
                .unwrap()
                .unwrap();
            Ok(())
        } else {
            ro_page_data.as_ref().0.fetch_add(1, Ordering::Release);
            drop(mmu_tables);

            let ro_page_refcount_guard = Guard::new(|| {
                ro_page_data.as_ref().0.fetch_sub(1, Ordering::Release);
            });

            let (new_page, new_page_data) = <Cx as AllocPhysPage>::alloc(mapping.page_size.order())
                .map_err(|()| FillError::OutOfMemory)?;
            new_page_data.as_ref().0.store(1, Ordering::Release);
            let new_page_guard = Guard::new(|| unsafe {
                <Cx as AllocPhysPage>::dealloc(new_page, mapping.page_size.order());
            });

            unsafe {
                Cx::copy(ro_page, new_page, mapping.page_size.order())
                    .map_err(FillError::MapPhysPage)?;
            }

            let did_replace = self.mmu_tables().lock().compare_and_swap(
                &mut Cx::default(),
                addr,
                mapping.page_size,
                ro_entry,
                new_page,
                mapping.mmu_flags,
            )?;

            ro_page_refcount_guard.success();
            new_page_guard.success();

            unsafe {
                match did_replace {
                    true => Self::destroy_allocated_page(mapping, ro_page, ro_page_data),
                    false => Self::destroy_allocated_page(mapping, new_page, new_page_data),
                }
            }
            Ok(())
        }
    }

    pub fn page_fault(
        self: Pin<&Self>,
        fault_addr: VirtAddr,
        fault_conditions: FaultConditions,
    ) -> Result<(), PageFaultError<<Cx as MapPhysPage>::Err>> {
        let mapping = self
            .mappings()
            .lock()
            .find(fault_addr)
            .ok_or(PageFaultError::InvalidAccess)?
            .clone();
        let mapping = mapping.as_ref();
        let fault_addr = fault_addr.align_down(mapping.page_size.order().size());
        let offset = fault_addr - mapping.range.start;

        do_busy_work(mapping.as_ref(), || match fault_conditions {
            FaultConditions {
                was_write: true, ..
            } if !mapping.mmu_flags.writable => Err(PageFaultError::InvalidAccess),
            FaultConditions {
                was_instruction_fetch: true,
                ..
            } if !mapping.mmu_flags.executable => Err(PageFaultError::InvalidAccess),
            FaultConditions {
                was_user_access: true,
                ..
            } if !mapping.mmu_flags.user_accessible => Err(PageFaultError::InvalidAccess),

            FaultConditions {
                was_present: false, ..
            } => self.fill(mapping, offset).map_err(PageFaultError::Fill),
            FaultConditions {
                was_present: true,
                was_write: true,
                was_instruction_fetch: false,
                ..
            } => self.do_cow(mapping, offset).map_err(PageFaultError::Fill),

            _ => Err(PageFaultError::InvalidAccess),
        })
    }

    pub fn prefill(
        self: Pin<&Self>,
        mapping: Pin<&Mapping<Cx>>,
    ) -> Result<(), FillError<<Cx as MapPhysPage>::Err>> {
        do_busy_work(mapping, || {
            for offset in mapping.page_offsets() {
                self.fill(mapping, offset)?;
            }
            Ok(())
        })
    }
}

fn do_busy_work<Cx, F, R, E>(mapping: Pin<&Mapping<Cx>>, f: F) -> Result<R, E>
where
    Cx: Context,
    F: FnOnce() -> Result<R, E>,
    E: From<MappingWasDestroyed>,
    Cx::PageData: AsRef<PageRefCount>,
{
    let mapping_data = mapping.as_ref().data();
    match &mut *mapping_data.status().lock() {
        status @ Status::Ready => *status = Status::Busy(0),
        Status::Busy(n) => *n += 1,
        Status::Destroyed => return Err(MappingWasDestroyed.into()),
    }
    let x = f();
    match &mut *mapping_data.status().lock() {
        Status::Ready | Status::Destroyed => unreachable!(),
        status @ Status::Busy(0) => {
            *status = Status::Ready;
            mapping_data.status_condvar().notify_all();
        }
        Status::Busy(n) => *n -= 1,
    }
    x
}

impl<Cx: Context> Mapping<Cx>
where
    Cx::PageData: AsRef<PageRefCount>,
{
    pub fn page_offsets(&self) -> impl Iterator<Item = usize> {
        (0..self.range.end.0 - self.range.start.0).step_by(self.page_size.order().size())
    }

    fn is_cow(&self) -> bool {
        match self.src {
            Source::Private(Object::Anonymous) => true,
            Source::Raw(_) => false,
        }
    }
}
