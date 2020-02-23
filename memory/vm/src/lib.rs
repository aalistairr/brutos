#![cfg_attr(not(any(test, feature = "std")), no_std)]

use core::fmt::Debug;
use core::ops::Range;
use core::pin::Pin;
use core::sync::atomic::{AtomicUsize, Ordering};

use brutos_alloc::{AllocOne, Arc, ArcInner, OutOfMemory};
use brutos_memory_traits::{AllocPhysPage, MapPhysPage, MmuEntry, MmuMap};
use brutos_memory_units::{PhysAddr, VirtAddr};
use brutos_sync::condvar::Condvar;
use brutos_sync::mutex::{Mutex, PinMutex};
use brutos_util::linked_list::Node;

mod mappings;

use mappings::Mappings;

pub use brutos_memory_traits::{MmuFlags, PageSize};

pub unsafe trait Context:
    brutos_sync::waitq::Context + AllocOne<ArcInner<Mapping<Self>>> + AllocPhysPage + MapPhysPage
{
    type Obj: Object;
    type MmuMap: MmuMap;
}

pub type CacheType<Cx> = <<Cx as Context>::MmuMap as MmuMap>::CacheType;

pub unsafe trait Object {
    type Meta;
    type GenerateErr: Debug;

    fn writable(&self) -> bool;

    fn generate_page(
        &self,
        offset: usize,
        page_size: PageSize,
    ) -> Result<(PhysAddr, &Self::Meta), Self::GenerateErr>;
    fn destroy_page(&self, page: PhysAddr, offset: usize, page_size: PageSize);

    fn meta(&self, page: PhysAddr) -> &Self::Meta;
    fn unique_page(&self, meta: &Self::Meta) -> bool;
    fn inc_page_refcount(&self, meta: &Self::Meta) -> bool;
    fn dec_page_refcount(&self, meta: &Self::Meta) -> bool;
}

#[derive(Default, Debug)]
pub struct PageRefCount(AtomicUsize);

impl PageRefCount {
    pub const fn new() -> PageRefCount {
        PageRefCount(AtomicUsize::new(0))
    }

    pub fn unique_page(&self) -> bool {
        self.0.load(Ordering::Acquire) == 1
    }

    pub fn inc_page_refcount(&self) -> bool {
        self.0.fetch_add(1, Ordering::AcqRel) == 0
    }

    pub fn dec_page_refcount(&self) -> bool {
        self.0.fetch_sub(1, Ordering::AcqRel) == 1
    }
}

pub struct Space<Cx: Context> {
    mappings: PinMutex<Mappings<Cx>, Cx>,
    pub mmu_map: Mutex<Cx::MmuMap, Cx>,
}

pub struct Mapping<Cx: Context> {
    pub range: Range<VirtAddr>,
    pub src: Source<Cx::Obj>,
    pub page_size: PageSize,
    pub flags: Flags<CacheType<Cx>>,
    status: Mutex<Status, Cx>,
    status_condvar: Condvar<Cx>,
    node: Node<MappingSel<Cx>>,
}

brutos_util::selector!(MappingSel<Cx: Context>: Arc<Mapping<Cx>, Cx> => node);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Location {
    Aligned(usize),
    Fixed(VirtAddr),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Flags<CacheType> {
    pub mapping: MappingFlags,
    pub mmu: MmuFlags<CacheType>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct MappingFlags {
    pub guarded: bool,
    pub wired: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Source<Obj> {
    Private(Obj),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Status {
    Busy(usize),
    Destroyed,
}

impl Status {
    fn is_busy(&mut self) -> bool {
        match self {
            Status::Busy(0) | Status::Destroyed => false,
            Status::Busy(_) => true,
        }
    }
}

impl<Cx: Context> Mapping<Cx> {
    fn initialize(self: Pin<&Self>) {
        self.status().initialize();
        self.status_condvar().initialize();
    }

    fn status(self: Pin<&Self>) -> Pin<&Mutex<Status, Cx>> {
        unsafe { self.map_unchecked(|x| &x.status) }
    }

    fn status_condvar(self: Pin<&Self>) -> Pin<&Condvar<Cx>> {
        unsafe { self.map_unchecked(|x| &x.status_condvar) }
    }

    fn size(&self) -> usize {
        self.range.end - self.range.start
    }

    fn page_offsets(&self) -> impl Iterator<Item = usize> {
        (0..self.size()).step_by(self.page_size.order().size())
    }
}

struct BusyGuard<'a, Cx: Context> {
    mapping: Pin<&'a Mapping<Cx>>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct MappingWasDestroyed;

impl<Cx: Context> Mapping<Cx> {
    fn do_busy_work(self: Pin<&Self>) -> Result<BusyGuard<Cx>, MappingWasDestroyed> {
        match &mut *self.status().lock() {
            Status::Busy(n) => *n += 1,
            Status::Destroyed => return Err(MappingWasDestroyed),
        }
        Ok(BusyGuard { mapping: self })
    }
}

impl<'a, Cx: Context> Drop for BusyGuard<'a, Cx> {
    fn drop(&mut self) {
        match &mut *self.mapping.status().lock() {
            Status::Busy(0) | Status::Destroyed => unreachable!(),
            Status::Busy(n @ 1) => {
                *n = 0;
                self.mapping.status_condvar().notify_all();
            }
            Status::Busy(n) => *n -= 1,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum CreateError {
    OutOfMemory,
    InvalidParameters,
    NoSpace,
}

impl From<OutOfMemory> for CreateError {
    fn from(OutOfMemory: OutOfMemory) -> CreateError {
        CreateError::OutOfMemory
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum DestroyErr<UnmapErr> {
    MappingNotFound,
    Unmap(UnmapErr),
}

pub type DestroyError<Cx> = DestroyErr<UnmapErr<Cx>>;

pub type UnmapErr<Cx> = <<Cx as Context>::MmuMap as MmuMap>::UnmapErr;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum FillErr<GenerateErr, MapErr> {
    Generate(GenerateErr),
    Map(MapErr),
    MappingWasDestroyed,
}

pub type FillError<Cx> = FillErr<GenerateErr<Cx>, MapErr<Cx>>;

pub type GenerateErr<Cx> = <<Cx as Context>::Obj as Object>::GenerateErr;
pub type MapErr<Cx> = <<Cx as Context>::MmuMap as MmuMap>::MapErr;

impl<E1, E2> From<MappingWasDestroyed> for FillErr<E1, E2> {
    fn from(MappingWasDestroyed: MappingWasDestroyed) -> FillErr<E1, E2> {
        FillErr::MappingWasDestroyed
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum CopyErr<MapPhysPageErr> {
    OutOfMemory,
    MapPhysPage(MapPhysPageErr),
}

pub type CopyError<Cx> = CopyErr<MapPhysPageErr<Cx>>;

pub type MapPhysPageErr<Cx> = <Cx as MapPhysPage>::Err;

impl<Cx: Context> Space<Cx>
where
    Cx::PageData: AsRef<PageRefCount>,
{
    pub fn new(range: Range<VirtAddr>, mmu_map: Cx::MmuMap) -> Space<Cx> {
        Space {
            mappings: PinMutex::new(Mappings::new(range)),
            mmu_map: Mutex::new(mmu_map),
        }
    }

    pub fn initialize(self: Pin<&Self>) {
        self.mappings().initialize();
        self.mappings().lock().as_mut().initialize();
        self.mmu_map().initialize();
    }

    fn mappings(self: Pin<&Self>) -> Pin<&PinMutex<Mappings<Cx>, Cx>> {
        unsafe { self.map_unchecked(|x| &x.mappings) }
    }

    pub fn mmu_map(self: Pin<&Self>) -> Pin<&Mutex<Cx::MmuMap, Cx>> {
        unsafe { self.map_unchecked(|x| &x.mmu_map) }
    }

    pub fn create_mapping(
        self: Pin<&Self>,
        size: usize,
        at: Location,
        src: Source<Cx::Obj>,
        page_size: PageSize,
        flags: Flags<CacheType<Cx>>,
    ) -> Result<Pin<Arc<Mapping<Cx>, Cx>>, CreateError> {
        if flags.mmu.copied {
            return Err(CreateError::InvalidParameters);
        }
        Ok(self
            .mappings()
            .lock()
            .as_mut()
            .add(size, at, flags.mapping, |range| {
                let mapping = Arc::pin(Mapping {
                    range,
                    src,
                    page_size,
                    flags,
                    status: Mutex::new(Status::Busy(0)),
                    status_condvar: Condvar::new(),
                    node: Node::new(),
                })
                .map_err(|(e, _)| e)?;
                mapping.as_ref().initialize();
                Ok(mapping)
            })?)
    }

    pub fn destroy_mapping(
        self: Pin<&Self>,
        mapping_addr: VirtAddr,
    ) -> Result<(), DestroyError<Cx>> {
        let mapping = self
            .mappings()
            .lock()
            .as_ref()
            .find(mapping_addr)
            .ok_or(DestroyErr::MappingNotFound)?
            .clone();

        let status_condvar = mapping.as_ref().status_condvar();
        let status = mapping.as_ref().status().lock();
        match &mut *status_condvar.wait_while(status, Status::is_busy) {
            status @ Status::Busy(0) => *status = Status::Destroyed,
            Status::Destroyed => return Ok(()),
            Status::Busy(_) => unreachable!(),
        }

        self.unmap_all_pages(&mapping)?;

        self.mappings().lock().as_mut().remove(mapping_addr)?;
        Ok(())
    }

    fn unmap_all_pages(
        self: Pin<&Self>,
        mapping: &Pin<Arc<Mapping<Cx>, Cx>>,
    ) -> Result<(), DestroyError<Cx>> {
        for offset in mapping.page_offsets() {
            let addr = mapping.range.start + offset;
            if let Some(entry) = self
                .mmu_map()
                .lock()
                .unmap(mapping.page_size, addr)
                .map_err(DestroyErr::Unmap)?
            {
                self.release_entry(&mapping, entry, offset);
            }
        }
        Ok(())
    }

    fn retain_entry(
        self: Pin<&Self>,
        mapping: &Pin<Arc<Mapping<Cx>, Cx>>,
        entry: <Cx::MmuMap as MmuMap>::Entry,
    ) {
        let ps = mapping.page_size;
        let page = entry.address(ps);
        if entry.flags(ps).copied {
            <Cx as AllocPhysPage>::get_data(page)
                .as_ref()
                .inc_page_refcount();
        } else {
            match &mapping.src {
                Source::Private(obj) => {
                    obj.inc_page_refcount(obj.meta(page));
                }
            }
        }
    }

    fn release_entry(
        self: Pin<&Self>,
        mapping: &Pin<Arc<Mapping<Cx>, Cx>>,
        entry: <Cx::MmuMap as MmuMap>::Entry,
        offset: usize,
    ) {
        let ps = mapping.page_size;
        let page = entry.address(ps);
        if entry.flags(ps).copied {
            self.release_copied_page(mapping, page);
        } else {
            self.release_source_page(mapping, page, offset);
        }
    }

    fn generate_source_page(
        self: Pin<&Self>,
        mapping: &Pin<Arc<Mapping<Cx>, Cx>>,
        offset: usize,
    ) -> Result<(PhysAddr, bool), <Cx::Obj as Object>::GenerateErr> {
        match &mapping.src {
            Source::Private(obj) => {
                let (page, meta) = obj.generate_page(offset, mapping.page_size)?;
                let unique_page = obj.inc_page_refcount(meta);
                Ok((page, unique_page && obj.writable()))
            }
        }
    }

    fn release_source_page(
        self: Pin<&Self>,
        mapping: &Pin<Arc<Mapping<Cx>, Cx>>,
        page: PhysAddr,
        offset: usize,
    ) {
        match &mapping.src {
            Source::Private(obj) => {
                let meta = obj.meta(page);
                if obj.dec_page_refcount(meta) {
                    obj.destroy_page(page, offset, mapping.page_size);
                }
            }
        }
    }

    fn copy_page(
        self: Pin<&Self>,
        mapping: &Pin<Arc<Mapping<Cx>, Cx>>,
        src: PhysAddr,
    ) -> Result<PhysAddr, CopyError<Cx>> {
        let order = mapping.page_size.order();
        let (dst, meta) = <Cx as AllocPhysPage>::alloc(order).map_err(|()| CopyErr::OutOfMemory)?;
        meta.as_ref().inc_page_refcount();
        unsafe {
            <Cx as MapPhysPage>::copy(src, dst, order).map_err(CopyErr::MapPhysPage)?;
        }
        Ok(dst)
    }

    fn release_copied_page(self: Pin<&Self>, mapping: &Pin<Arc<Mapping<Cx>, Cx>>, page: PhysAddr) {
        let meta = <Cx as AllocPhysPage>::get_data(page);
        if meta.as_ref().dec_page_refcount() {
            unsafe {
                <Cx as AllocPhysPage>::dealloc(page, mapping.page_size.order());
            }
        }
    }

    fn fill(
        self: Pin<&Self>,
        mapping: &Pin<Arc<Mapping<Cx>, Cx>>,
        addr: VirtAddr,
        offset: usize,
    ) -> Result<MmuFlags<CacheType<Cx>>, FillError<Cx>> {
        assert_eq!(offset % mapping.page_size.order().size(), 0);

        let (page, writable_page) = self
            .generate_source_page(mapping, offset)
            .map_err(FillErr::Generate)?;

        let mmu_flags = MmuFlags {
            writable: writable_page && mapping.flags.mmu.writable,
            ..mapping.flags.mmu
        };
        if !self
            .mmu_map()
            .lock()
            .map_keep(mmu_flags, mapping.page_size, addr, page)
            .map_err(FillErr::Map)?
        {
            self.release_source_page(mapping, page, offset);
        }
        Ok(mmu_flags)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct FaultConditions {
    pub present: bool,
    pub user_access: bool,
    pub write: bool,
    pub instruction_fetch: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum FaultErr<GenerateErr, MapErr, GetErr, MapPhysPageErr> {
    Fill(FillErr<GenerateErr, MapErr>),
    Cow(CowErr<MapErr, GetErr, MapPhysPageErr>),
    MappingNotFound,
    InvalidAccess,
}

pub type FaultError<Cx> = FaultErr<GenerateErr<Cx>, MapErr<Cx>, GetErr<Cx>, MapPhysPageErr<Cx>>;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum CowErr<MapErr, GetErr, MapPhysPageErr> {
    Map(MapErr),
    Get(GetErr),
    Copy(CopyErr<MapPhysPageErr>),
}

pub type GetErr<Cx> = <<Cx as Context>::MmuMap as MmuMap>::GetErr;

pub type CowError<Cx> = CowErr<MapErr<Cx>, GetErr<Cx>, MapPhysPageErr<Cx>>;

impl<Cx: Context> Space<Cx>
where
    Cx::PageData: AsRef<PageRefCount>,
{
    pub fn page_fault(
        self: Pin<&Self>,
        fault_addr: VirtAddr,
        fault_conditions: FaultConditions,
    ) -> Result<(), FaultError<Cx>> {
        let mapping = self
            .mappings()
            .lock()
            .as_ref()
            .find(fault_addr)
            .ok_or(FaultErr::MappingNotFound)?
            .clone();

        let fc = fault_conditions;
        let mf = mapping.flags.mmu;
        if (fc.user_access && !mf.user_accessible)
            || (fc.write && !mf.writable)
            || (fc.instruction_fetch && !mf.executable)
        {
            return Err(FaultErr::InvalidAccess);
        }

        let _busy_guard = mapping.as_ref().do_busy_work();

        let fault_addr = fault_addr.align_down(mapping.page_size.order().size());
        let offset = fault_addr - mapping.range.start;
        if fc.present {
            self.fill(&mapping, fault_addr, offset)
                .map_err(FaultErr::Fill)?;
            Ok(())
        } else if fc.write {
            self.do_cow(&mapping, fault_addr, offset)
                .map_err(FaultErr::Cow)?;
            Ok(())
        } else {
            unreachable!()
        }
    }

    fn do_cow(
        self: Pin<&Self>,
        mapping: &Pin<Arc<Mapping<Cx>, Cx>>,
        addr: VirtAddr,
        offset: usize,
    ) -> Result<(), CowError<Cx>> {
        let ps = mapping.page_size;
        let mut mmu_map = self.mmu_map().lock();
        let entry = match mmu_map.get_entry(ps, addr).map_err(CowErr::Get)? {
            None => return Ok(()),
            Some(entry) if entry.flags(ps).copied => return Ok(()),
            Some(x) => x,
        };

        if self.writable_entry_page(mapping, entry) {
            let mmu_flags = MmuFlags {
                copied: entry.flags(ps).copied,
                ..mapping.flags.mmu
            };
            mmu_map
                .map_replace(mmu_flags, ps, addr, entry.address(ps))
                .map_err(CowErr::Map)?;
            return Ok(());
        }

        let src_page = entry.address(ps);
        self.retain_entry(mapping, entry);

        drop(mmu_map);

        let dst_page = self.copy_page(mapping, src_page);

        self.release_entry(mapping, entry, offset);
        let dst_page = dst_page.map_err(CowErr::Copy)?;

        let mut mmu_map = self.mmu_map().lock();
        let mmu_flags = MmuFlags {
            copied: true,
            ..mapping.flags.mmu
        };
        match mmu_map.compare_and_map(mmu_flags, ps, addr, entry, dst_page) {
            Ok(true) => Ok(()),
            r @ Err(_) | r @ Ok(false) => {
                self.release_copied_page(mapping, dst_page);
                r.map(|_| ()).map_err(CowErr::Map)
            }
        }
    }

    fn writable_entry_page(
        self: Pin<&Self>,
        mapping: &Pin<Arc<Mapping<Cx>, Cx>>,
        entry: <Cx::MmuMap as MmuMap>::Entry,
    ) -> bool {
        let ps = mapping.page_size;
        let page = entry.address(ps);
        if entry.flags(ps).copied {
            <Cx as AllocPhysPage>::get_data(page).as_ref().unique_page()
        } else {
            match &mapping.src {
                Source::Private(obj) => obj.writable() && obj.unique_page(obj.meta(page)),
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PrefillErr<GenerateErr, MapErr, GetErr, MapPhysPageErr> {
    Fill(FillErr<GenerateErr, MapErr>),
    Cow(CowErr<MapErr, GetErr, MapPhysPageErr>),
    MappingWasDestroyed,
}

pub type PrefillError<Cx> = PrefillErr<GenerateErr<Cx>, MapErr<Cx>, GetErr<Cx>, MapPhysPageErr<Cx>>;

impl<E1, E2, E3, E4> From<MappingWasDestroyed> for PrefillErr<E1, E2, E3, E4> {
    fn from(MappingWasDestroyed: MappingWasDestroyed) -> PrefillErr<E1, E2, E3, E4> {
        PrefillErr::MappingWasDestroyed
    }
}

impl<Cx: Context> Space<Cx>
where
    Cx::PageData: AsRef<PageRefCount>,
{
    pub fn prefill(
        self: Pin<&Self>,
        mapping: &Pin<Arc<Mapping<Cx>, Cx>>,
    ) -> Result<(), PrefillError<Cx>> {
        let busy_guard = mapping.as_ref().do_busy_work()?;
        for offset in mapping.page_offsets() {
            let addr = mapping.range.start + offset;
            let page_mmu_flags = self.fill(mapping, addr, offset).map_err(PrefillErr::Fill)?;
            if mapping.flags.mmu.writable && !page_mmu_flags.writable {
                self.do_cow(mapping, addr, offset)
                    .map_err(PrefillErr::Cow)?;
            }
        }
        drop(busy_guard);
        Ok(())
    }
}
