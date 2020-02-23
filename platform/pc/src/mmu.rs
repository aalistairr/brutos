use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::mem::{forget, ManuallyDrop};
use core::ops::Range;
use core::ptr::NonNull;

use bitbash::bitfield;

use brutos_alloc::OutOfMemory;
use brutos_memory_traits::{MmuEntry, MmuFlags, MmuMap, PageSize};
use brutos_memory_units::{PhysAddr, VirtAddr};
use brutos_util::UInt;

pub unsafe trait Context {
    fn alloc_table() -> Result<PhysAddr, OutOfMemory>;
    unsafe fn dealloc_table(table: PhysAddr);
    fn map_table(table: PhysAddr) -> NonNull<Table>;

    fn new_table() -> Result<(PhysAddr, NonNull<Table>), OutOfMemory> {
        let addr = Self::alloc_table()?;
        let ptr = Self::map_table(addr);
        unsafe {
            core::ptr::write_bytes(ptr.as_ptr(), 0u8, 1usize);
        }
        Ok((addr, ptr))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Level {
    Pt,
    Pd,
    Pdpt,
    Pml4,
    Root,
}

impl From<PageSize> for Level {
    fn from(page_size: PageSize) -> Level {
        match page_size {
            PageSize::Normal => Level::Pt,
            PageSize::Large => Level::Pd,
            PageSize::Huge => Level::Pdpt,
        }
    }
}

impl Level {
    pub fn addr_bits(&self) -> Range<u32> {
        assert!(*self <= Level::Pml4);
        12 + 9 * (*self as u32)..12 + 9 * (*self as u32 + 1)
    }

    pub fn entry_size(&self) -> usize {
        1 << self.addr_bits().start
    }

    pub fn down(&self) -> Option<Level> {
        match self {
            Level::Root => Some(Level::Pml4),
            Level::Pml4 => Some(Level::Pdpt),
            Level::Pdpt => Some(Level::Pd),
            Level::Pd => Some(Level::Pt),
            Level::Pt => None,
        }
    }

    pub fn up(&self) -> Option<Level> {
        match self {
            Level::Pt => Some(Level::Pd),
            Level::Pd => Some(Level::Pdpt),
            Level::Pdpt => Some(Level::Pml4),
            Level::Pml4 => Some(Level::Root),
            Level::Root => None,
        }
    }

    pub fn table_index(&self, addr: VirtAddr) -> usize {
        addr.0.bits(self.addr_bits())
    }

    pub const fn is_pt(&self) -> bool {
        match self {
            Level::Pt => true,
            _ => false,
        }
    }

    pub const fn is_le_pdpt(&self) -> bool {
        match self {
            Level::Pt | Level::Pd | Level::Pdpt => true,
            _ => false,
        }
    }
}

type CacheType = usize;

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    #[repr(transparent)]
    pub struct Entry(usize);

    pub new();
    derive_debug;

    pub field present: bool = [0];
    pub field writable: bool = [1];
    pub field user_accessible: bool = [2];
    pub field accessed: bool = [5];
    field ps_dirty: bool = [6];
    field pml4e_pdpte_pde_ps: bool = [7];
    field ps_global: bool = [8];
    pub field address: PhysAddr { [12..48] => [12..48] }
    pub field not_executable: bool = [63];

    field pte_pat_index: usize = [3] ~ [4] ~ [7];
    field nonps_pdpte_pde_pat_index: usize = [3] ~ [4];
    field ps_pdpte_pde_pat_index: usize = [3] ~ [4] ~ [12];

    pub field copied: bool = [9];
    field population: usize = [52..62];
}

impl Entry {
    const PERMANENT: usize = (1 << 10) - 1;

    pub const fn ps(&self, level: Level) -> bool {
        level.is_pt() || (level.is_le_pdpt() && self.pml4e_pdpte_pde_ps())
    }

    pub const fn set_ps(&mut self, level: Level, ps: bool) {
        if level.is_pt() {
            return;
        } else {
            assert!(level.is_le_pdpt());
            self.set_pml4e_pdpte_pde_ps(ps);
        }
    }

    pub const fn with_ps(mut self, level: Level, ps: bool) -> Self {
        self.set_ps(level, ps);
        self
    }

    pub const fn dirty(&self, level: Level) -> bool {
        assert!(self.ps(level));
        self.ps_dirty()
    }

    pub const fn set_dirty(&mut self, level: Level, dirty: bool) {
        assert!(self.ps(level));
        self.set_ps_dirty(dirty);
    }

    pub const fn with_dirty(mut self, level: Level, dirty: bool) -> Self {
        self.set_dirty(level, dirty);
        self
    }

    pub const fn global(&self, level: Level) -> bool {
        assert!(self.ps(level));
        self.ps_global()
    }

    pub const fn set_global(&mut self, level: Level, global: bool) {
        assert!(self.ps(level));
        self.set_ps_global(global);
    }

    pub const fn with_global(mut self, level: Level, global: bool) -> Self {
        self.set_global(level, global);
        self
    }

    pub const fn cache_type(&self, level: Level) -> CacheType {
        if level.is_pt() {
            self.pte_pat_index()
        } else {
            assert!(level.is_le_pdpt());
            if self.pml4e_pdpte_pde_ps() {
                self.ps_pdpte_pde_pat_index()
            } else {
                self.nonps_pdpte_pde_pat_index()
            }
        }
    }

    pub const fn set_cache_type(&mut self, level: Level, cache_type: CacheType) {
        if level.is_pt() {
            self.set_pte_pat_index(cache_type);
        } else {
            assert!(level.is_le_pdpt());
            if self.pml4e_pdpte_pde_ps() {
                self.set_ps_pdpte_pde_pat_index(cache_type);
            } else {
                self.set_nonps_pdpte_pde_pat_index(cache_type);
            }
        }
    }

    pub const fn with_cache_type(mut self, level: Level, cache_type: CacheType) -> Self {
        self.set_cache_type(level, cache_type);
        self
    }

    pub const fn permanent(&self) -> bool {
        self.population() == Self::PERMANENT
    }

    pub const fn set_permanent(&mut self, permanent: bool) {
        assert!(self.population() == 0 || self.population() == Self::PERMANENT);
        self.set_population(if permanent { Self::PERMANENT } else { 0 });
    }

    pub const fn with_permanent(mut self, permanent: bool) -> Self {
        self.set_permanent(permanent);
        self
    }

    pub const fn inc_population(mut self) -> Self {
        if !self.permanent() {
            self.set_population(self.population() + 1);
        }
        self
    }

    pub const fn dec_population(mut self) -> Self {
        if !self.permanent() {
            self.set_population(self.population() - 1);
        }
        self
    }
}

impl MmuEntry for Entry {
    type CacheType = CacheType;

    fn address(&self, _page_size: PageSize) -> PhysAddr {
        self.address()
    }

    fn flags(&self, page_size: PageSize) -> MmuFlags<CacheType> {
        let level = page_size.into();
        MmuFlags {
            user_accessible: self.user_accessible(),
            writable: self.writable(),
            executable: !self.not_executable(),
            copied: self.copied(),
            global: self.global(level),
            cache_type: self.cache_type(level),
        }
    }
}

#[repr(transparent)]
pub struct EntryCell(UnsafeCell<Entry>);

impl EntryCell {
    pub const fn with_entry(entry: Entry) -> EntryCell {
        EntryCell(UnsafeCell::new(entry))
    }

    pub const fn new() -> EntryCell {
        EntryCell::with_entry(Entry::new())
    }

    pub fn read(&self) -> Entry {
        unsafe { core::ptr::read_volatile(self.0.get()) }
    }

    pub fn write(&self, entry: Entry) {
        unsafe { core::ptr::write_volatile(self.0.get(), entry) }
    }

    pub fn map<F>(&self, f: F) -> Entry
    where
        F: FnOnce(Entry) -> Entry,
    {
        let entry = f(self.read());
        self.write(entry);
        entry
    }
}

#[repr(C, align(4096))]
pub struct Table([EntryCell; 512]);

struct Trail<'a, Cx: Context> {
    roote: Option<NonNull<EntryCell>>,
    pml4e: Option<NonNull<EntryCell>>,
    pdpte: Option<NonNull<EntryCell>>,
    pde: Option<NonNull<EntryCell>>,
    pte: Option<NonNull<EntryCell>>,
    _marker: PhantomData<(Cx, &'a EntryCell)>,
}

enum TrailError {
    TableNotAllocated,
    OutOfMemory,
    Obstructed,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct TrailArgs<'a> {
    alloc: bool,
    flags: &'a MmuFlags<CacheType>,
    addr: VirtAddr,
}

impl<'a, Cx: Context> Trail<'a, Cx> {
    fn new(roote: &'a EntryCell) -> Trail<'a, Cx> {
        Trail {
            roote: Some(roote.into()),
            pml4e: None,
            pdpte: None,
            pde: None,
            pte: None,
            _marker: PhantomData,
        }
    }

    fn find(
        &mut self,
        alloc: bool,
        flags: &MmuFlags<CacheType>,
        entry_level: Level,
        addr: VirtAddr,
    ) -> Result<(&EntryCell, Option<&EntryCell>), TrailError> {
        let roote = match self.roote {
            Some(roote) => unsafe { &*roote.as_ptr() },
            None => unreachable!(),
        };
        if entry_level == Level::Root {
            return Ok((roote, None));
        }

        let args = TrailArgs { alloc, flags, addr };

        let pml4e = Self::dig(args, Level::Pml4, &mut self.pml4e, roote, None)?;
        if entry_level == Level::Pml4 {
            return Ok((pml4e, Some(roote)));
        }

        let pdpte = Self::dig(args, Level::Pdpt, &mut self.pdpte, pml4e, Some(roote))?;
        if entry_level == Level::Pdpt {
            return Ok((pdpte, Some(pml4e)));
        }

        let pde = Self::dig(args, Level::Pd, &mut self.pde, pdpte, Some(pml4e))?;
        if entry_level == Level::Pd {
            return Ok((pde, Some(pdpte)));
        }

        let pte = Self::dig(args, Level::Pt, &mut self.pte, pde, Some(pdpte))?;
        if entry_level == Level::Pt {
            return Ok((pte, Some(pde)));
        }

        unreachable!()
    }

    #[inline]
    fn dig(
        args: TrailArgs,
        child_level: Level,
        trail_child_entry_cell: &mut Option<NonNull<EntryCell>>,
        entry_cell: &EntryCell,
        parent_entry_cell: Option<&EntryCell>,
    ) -> Result<&'a EntryCell, TrailError> {
        assert!(trail_child_entry_cell.is_none());
        let mut entry = entry_cell.read();
        if !entry.present() {
            if args.alloc {
                entry = Self::create_table(args.flags, entry_cell, parent_entry_cell)?;
            } else {
                return Err(TrailError::TableNotAllocated);
            }
        } else if entry.pml4e_pdpte_pde_ps() {
            return Err(TrailError::Obstructed);
        }

        let table_addr = entry.address();
        let table_ptr = Cx::map_table(table_addr);
        let table = unsafe { &*table_ptr.as_ptr() };
        let table_i = child_level.table_index(args.addr);
        let child_entry_cell = &table.0[table_i];
        *trail_child_entry_cell = Some(child_entry_cell.into());
        Ok(child_entry_cell)
    }

    #[cold]
    fn create_table(
        flags: &MmuFlags<CacheType>,
        entry_cell: &EntryCell,
        parent_entry_cell: Option<&EntryCell>,
    ) -> Result<Entry, TrailError> {
        let (table_addr, _) = Cx::new_table().map_err(|OutOfMemory| TrailError::OutOfMemory)?;
        let entry = Entry::new()
            .with_present(true)
            .with_address(table_addr)
            .with_user_accessible(flags.user_accessible);
        entry_cell.write(entry);
        if let Some(parent_entry_cell) = parent_entry_cell {
            parent_entry_cell.map(Entry::inc_population);
        }
        Ok(entry)
    }

    fn pop_entry(
        level: Level,
        entry_cell: &mut Option<NonNull<EntryCell>>,
        parent_entry_cell: Option<&Option<NonNull<EntryCell>>>,
    ) -> bool {
        if let Some(entry_cell) = entry_cell.take() {
            let entry_cell = unsafe { entry_cell.as_ref() };
            let entry = entry_cell.read();
            if entry.present() {
                if entry.ps(level) || entry.population() > 0 {
                    return false;
                }
                entry_cell.write(Entry::new());
                unsafe {
                    Cx::dealloc_table(entry.address());
                }
                match parent_entry_cell {
                    None => (),
                    Some(Some(parent_entry_cell)) => {
                        let parent_entry_cell = unsafe { parent_entry_cell.as_ref() };
                        parent_entry_cell.map(Entry::dec_population);
                    }
                    Some(None) => unreachable!(),
                }
            }
        }
        true
    }
}

impl<'a, Cx: Context> Drop for Trail<'a, Cx> {
    fn drop(&mut self) {
        if !Self::pop_entry(Level::Pt, &mut self.pte, Some(&self.pde)) {
            return;
        }
        if !Self::pop_entry(Level::Pd, &mut self.pde, Some(&self.pdpte)) {
            return;
        }
        if !Self::pop_entry(Level::Pdpt, &mut self.pdpte, Some(&self.pml4e)) {
            return;
        }
        if !Self::pop_entry(Level::Pml4, &mut self.pml4e, Some(&self.roote)) {
            return;
        }
        if !Self::pop_entry(Level::Root, &mut self.roote, None) {
            return;
        }
    }
}

pub struct Map<Cx: Context> {
    roote: EntryCell,
    _marker: PhantomData<Cx>,
}

impl<Cx: Context> Map<Cx> {
    pub const fn with_root(root: Entry) -> Map<Cx> {
        Map {
            roote: EntryCell::with_entry(root),
            _marker: PhantomData,
        }
    }

    pub const fn new() -> Map<Cx> {
        Map {
            roote: EntryCell::new(),
            _marker: PhantomData,
        }
    }

    fn trail(&self) -> Trail<Cx> {
        Trail::new(&self.roote)
    }

    pub fn page_tables(&self) -> Option<PhysAddr> {
        let entry = self.roote.read();
        if entry.present() {
            Some(entry.address())
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MapError {
    OutOfMemory,
    Obstructed,
}

impl From<TrailError> for MapError {
    fn from(e: TrailError) -> MapError {
        match e {
            TrailError::OutOfMemory => MapError::OutOfMemory,
            TrailError::Obstructed => MapError::Obstructed,
            TrailError::TableNotAllocated => unreachable!(),
        }
    }
}

impl From<OutOfMemory> for MapError {
    fn from(OutOfMemory: OutOfMemory) -> MapError {
        MapError::OutOfMemory
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum UnmapError {
    NotPresent,
    Obstructed,
}

impl From<TrailError> for UnmapError {
    fn from(e: TrailError) -> UnmapError {
        match e {
            TrailError::OutOfMemory => unreachable!(),
            TrailError::TableNotAllocated => UnmapError::NotPresent,
            TrailError::Obstructed => UnmapError::Obstructed,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GetError {
    NotPresent,
    Obstructed,
}

impl From<TrailError> for GetError {
    fn from(e: TrailError) -> GetError {
        match e {
            TrailError::OutOfMemory => unreachable!(),
            TrailError::TableNotAllocated => GetError::NotPresent,
            TrailError::Obstructed => GetError::Obstructed,
        }
    }
}

fn make_entry(flags: &MmuFlags<CacheType>, level: Level, addr: PhysAddr) -> Entry {
    Entry::new()
        .with_present(true)
        .with_ps(level, true)
        .with_address(addr)
        .with_user_accessible(flags.user_accessible)
        .with_writable(flags.writable)
        .with_not_executable(!flags.executable)
        .with_global(level, flags.global)
        .with_cache_type(level, flags.cache_type)
}

#[cfg(target_os = "bare")]
fn invlpg(addr: VirtAddr) {
    unsafe {
        asm!("invlpg ($0)" :: "r" (addr.0) : "memory" : "volatile");
    }
}

#[cfg(not(target_os = "bare"))]
fn invlpg(_addr: VirtAddr) {}

impl<Cx: Context> MmuMap for Map<Cx> {
    type MapErr = MapError;
    type UnmapErr = UnmapError;
    type GetErr = GetError;
    type Entry = Entry;
    type CacheType = CacheType;

    fn map_keep(
        &mut self,
        flags: MmuFlags<CacheType>,
        page_size: PageSize,
        virt_addr: VirtAddr,
        phys_addr: PhysAddr,
    ) -> Result<bool, MapError> {
        let level = page_size.into();
        let mut trail = self.trail();
        let (entry_cell, parent_entry_cell) = trail.find(true, &flags, level, virt_addr)?;
        let entry = entry_cell.read();
        if entry.present() {
            return Ok(false);
        }
        entry_cell.write(make_entry(&flags, level, phys_addr));
        parent_entry_cell.unwrap().map(Entry::inc_population);
        invlpg(virt_addr);
        forget(trail);
        Ok(true)
    }

    fn map_replace(
        &mut self,
        flags: MmuFlags<CacheType>,
        page_size: PageSize,
        virt_addr: VirtAddr,
        phys_addr: PhysAddr,
    ) -> Result<Option<Entry>, MapError> {
        let level = page_size.into();
        let mut trail = self.trail();
        let (entry_cell, parent_entry_cell) = trail.find(true, &flags, level, virt_addr)?;
        let prev_entry = entry_cell.read();
        entry_cell.write(make_entry(&flags, level, phys_addr));
        if prev_entry.present() {
            invlpg(virt_addr);
            forget(trail);
            Ok(Some(prev_entry))
        } else {
            parent_entry_cell.unwrap().map(Entry::inc_population);
            forget(trail);
            Ok(None)
        }
    }

    fn get_entry(
        &self,
        page_size: PageSize,
        virt_addr: VirtAddr,
    ) -> Result<Option<Entry>, GetError> {
        let level = page_size.into();
        let mut trail = ManuallyDrop::new(self.trail());
        let (entry_cell, _) = trail.find(false, &MmuFlags::default(), level, virt_addr)?;
        let entry = entry_cell.read();
        if entry.present() {
            Ok(Some(entry))
        } else {
            Ok(None)
        }
    }

    fn compare_and_map(
        &mut self,
        flags: MmuFlags<CacheType>,
        page_size: PageSize,
        virt_addr: VirtAddr,
        compare_entry: Entry,
        phys_addr: PhysAddr,
    ) -> Result<bool, MapError> {
        assert!(compare_entry.present());

        let level = page_size.into();
        let mut trail = ManuallyDrop::new(self.trail());
        let (entry_cell, _) = trail.find(false, &flags, level, virt_addr)?;
        if entry_cell.read() == compare_entry {
            entry_cell.write(make_entry(&flags, level, phys_addr));
            invlpg(virt_addr);
            forget(trail);
            Ok(true)
        } else {
            forget(trail);
            Ok(false)
        }
    }

    fn unmap(
        &mut self,
        page_size: PageSize,
        virt_addr: VirtAddr,
    ) -> Result<Option<Entry>, UnmapError> {
        let level = page_size.into();
        let mut trail = self.trail();
        let (entry_cell, parent_entry_cell) =
            trail.find(false, &MmuFlags::default(), level, virt_addr)?;
        let entry = entry_cell.read();
        if entry.present() {
            entry_cell.write(Entry::new());
            parent_entry_cell.unwrap().map(Entry::dec_population);
            invlpg(virt_addr);
            Ok(Some(entry))
        } else {
            Ok(None)
        }
    }
}

impl<Cx: Context> Map<Cx> {
    pub fn create_permanent_table(
        &mut self,
        flags: MmuFlags<CacheType>,
        level: Level,
        virt_addr: VirtAddr,
    ) -> Result<Entry, MapError> {
        assert!(level < Level::Root);
        assert!(level > Level::Pt);
        let mut trail = self.trail();
        let (_, parent_entry_cell) = trail.find(true, &flags, level.down().unwrap(), virt_addr)?;
        let entry = parent_entry_cell.unwrap().map(|e| e.with_permanent(true));
        forget(trail);
        Ok(entry)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SetError {
    OutOfMemory,
    Obstructed,
    Exists,
}

impl From<TrailError> for SetError {
    fn from(e: TrailError) -> SetError {
        match e {
            TrailError::OutOfMemory => SetError::OutOfMemory,
            TrailError::Obstructed => SetError::Obstructed,
            TrailError::TableNotAllocated => unreachable!(),
        }
    }
}

impl From<OutOfMemory> for SetError {
    fn from(OutOfMemory: OutOfMemory) -> SetError {
        SetError::OutOfMemory
    }
}

impl<Cx: Context> Map<Cx> {
    pub fn set_entry(
        &mut self,
        flags: MmuFlags<CacheType>,
        level: Level,
        virt_addr: VirtAddr,
        entry: Entry,
    ) -> Result<(), SetError> {
        assert!(level < Level::Root);
        assert!(entry.present());
        let mut trail = self.trail();
        let (entry_cell, parent_entry_cell) = trail.find(true, &flags, level, virt_addr)?;
        if entry_cell.read().present() {
            return Err(SetError::Exists);
        }
        entry_cell.write(entry);
        parent_entry_cell.unwrap().map(Entry::inc_population);
        forget(trail);
        Ok(())
    }

    pub fn clear_entry(&mut self, level: Level, virt_addr: VirtAddr) -> Result<(), UnmapError> {
        assert!(level < Level::Root);
        let mut trail = self.trail();
        let (entry_cell, parent_entry_cell) =
            trail.find(false, &Default::default(), level, virt_addr)?;
        if !entry_cell.read().present() {
            forget(trail);
            return Err(UnmapError::NotPresent);
        }
        entry_cell.write(Entry::new());
        parent_entry_cell.unwrap().map(Entry::dec_population);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[cfg(not(miri))]
    extern crate test;
    use super::*;

    use std::cell::RefCell;
    use std::ptr::NonNull;

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

    #[derive(Debug)]
    struct State {
        tables: HashMap<PhysAddr, NonNull<Table>>,
        addr: PhysAddr,
    }

    thread_local! {
        static STATE: RefCell<State> = RefCell::new(State { tables: new_hashmap(), addr: PhysAddr(0) });
    }

    unsafe impl Context for State {
        #[inline(never)]
        fn alloc_table() -> Result<PhysAddr, OutOfMemory> {
            STATE.with(|state| {
                const EMPTY_ENTRY_CELL: EntryCell = EntryCell(UnsafeCell::new(Entry::new()));
                let mut state = state.borrow_mut();
                let addr = state.addr;
                state.tables.insert(
                    addr,
                    Box::leak(Box::new(Table([EMPTY_ENTRY_CELL; 512]))).into(),
                );
                state.addr = addr + brutos_memory_units::arch::PAGE_SIZE;
                Ok(addr)
            })
        }

        #[inline(never)]
        unsafe fn dealloc_table(addr: PhysAddr) {
            STATE.with(|state| {
                let mut state = state.borrow_mut();
                let ptr = state.tables.remove(&addr).unwrap();
                drop(Box::from_raw(ptr.as_ptr()));
            })
        }

        #[inline(never)]
        fn map_table(addr: PhysAddr) -> NonNull<Table> {
            STATE.with(|state| state.borrow_mut().tables.get(&addr).cloned().unwrap())
        }
    }

    impl Drop for State {
        fn drop(&mut self) {
            for &ptr in self.tables.values() {
                drop(unsafe { Box::from_raw(ptr.as_ptr()) });
            }
        }
    }

    fn clear_state() {
        STATE.with(|state| {
            *state.borrow_mut() = State {
                tables: new_hashmap(),
                addr: PhysAddr(0),
            }
        });
    }

    impl State {
        fn to_hash_map(&self) -> HashMap<PhysAddr, HashMap<usize, Entry>> {
            let mut x = new_hashmap();
            for (&addr, &table) in &self.tables {
                let table = unsafe { table.as_ref() };
                let mut entries = new_hashmap();
                for (i, entry) in table.0.iter().map(EntryCell::read).enumerate() {
                    if entry.0 != 0 {
                        entries.insert(i, entry);
                    }
                }
                x.insert(addr, entries);
            }
            x
        }
    }

    use std::iter::FromIterator;

    fn sorted(x: HashMap<PhysAddr, HashMap<usize, Entry>>) -> Vec<(PhysAddr, Vec<(usize, Entry)>)> {
        let mut x: Vec<_> = x
            .into_iter()
            .map(|(k, v)| {
                (k, {
                    let mut v: Vec<_> = v.into_iter().collect();
                    v.sort_by_key(|&(k, _)| k);
                    v
                })
            })
            .collect();
        x.sort_by_key(|&(k, _)| k);
        x
    }

    use super::PageSize::*;

    fn state_to_hash_map() -> HashMap<PhysAddr, HashMap<usize, Entry>> {
        STATE.with(|s| s.borrow().to_hash_map())
    }

    #[test]
    fn check() {
        clear_state();
        let mut map = Map::<State>::new();

        assert_eq!(
            map.get_entry(Normal, VirtAddr(0)),
            Err(GetError::NotPresent)
        );
        assert_eq!(state_to_hash_map(), new_hashmap());

        {
            assert_eq!(
                map.map_replace(Default::default(), Normal, VirtAddr(0), PhysAddr(0x1234000)),
                Ok(None)
            );
            assert_eq!(
                map.roote.read(),
                Entry::new()
                    .with_present(true)
                    .with_address(PhysAddr(0x0))
                    .with_population(1)
            );
            assert_eq!(
                sorted(state_to_hash_map()),
                sorted(HashMap::from_iter(vec![
                    (
                        PhysAddr(0x0000), // PML4
                        HashMap::from_iter(vec![(
                            0usize,
                            Entry::new()
                                .with_present(true)
                                .with_address(PhysAddr(0x1000))
                                .with_population(1)
                        )])
                    ),
                    (
                        PhysAddr(0x1000), // PDP
                        HashMap::from_iter(vec![(
                            0usize,
                            Entry::new()
                                .with_present(true)
                                .with_address(PhysAddr(0x2000))
                                .with_population(1)
                        )])
                    ),
                    (
                        PhysAddr(0x2000), // PD
                        HashMap::from_iter(vec![(
                            0usize,
                            Entry::new()
                                .with_present(true)
                                .with_address(PhysAddr(0x3000))
                                .with_population(1)
                        )])
                    ),
                    (
                        PhysAddr(0x3000), // PT
                        HashMap::from_iter(vec![(
                            0usize,
                            Entry::new()
                                .with_present(true)
                                .with_address(PhysAddr(0x1234000))
                                .with_not_executable(true)
                        )])
                    )
                ]))
            );
        }

        {
            assert_eq!(
                map.map_replace(
                    Default::default(),
                    Normal,
                    VirtAddr(0x1000),
                    PhysAddr(0x2345000)
                ),
                Ok(None)
            );
            assert_eq!(
                map.roote.read(),
                Entry::new()
                    .with_present(true)
                    .with_address(PhysAddr(0x0))
                    .with_population(1)
            );
            assert_eq!(
                sorted(state_to_hash_map()),
                sorted(HashMap::from_iter(vec![
                    (
                        PhysAddr(0x0000), // PML4
                        HashMap::from_iter(vec![(
                            0usize,
                            Entry::new()
                                .with_present(true)
                                .with_address(PhysAddr(0x1000))
                                .with_population(1)
                        )])
                    ),
                    (
                        PhysAddr(0x1000), // PDP
                        HashMap::from_iter(vec![(
                            0usize,
                            Entry::new()
                                .with_present(true)
                                .with_address(PhysAddr(0x2000))
                                .with_population(1)
                        )])
                    ),
                    (
                        PhysAddr(0x2000), // PD
                        HashMap::from_iter(vec![(
                            0usize,
                            Entry::new()
                                .with_present(true)
                                .with_address(PhysAddr(0x3000))
                                .with_population(2)
                        )])
                    ),
                    (
                        PhysAddr(0x3000), // PT
                        HashMap::from_iter(vec![
                            (
                                0usize,
                                Entry::new()
                                    .with_present(true)
                                    .with_address(PhysAddr(0x1234000))
                                    .with_not_executable(true)
                            ),
                            (
                                1usize,
                                Entry::new()
                                    .with_present(true)
                                    .with_address(PhysAddr(0x2345000))
                                    .with_not_executable(true)
                            )
                        ])
                    )
                ]))
            );
        }

        {
            assert_eq!(
                map.map_replace(
                    Default::default(),
                    Normal,
                    VirtAddr(0x200000),
                    PhysAddr(0x3456000)
                ),
                Ok(None)
            );
            assert_eq!(
                map.roote.read(),
                Entry::new()
                    .with_present(true)
                    .with_address(PhysAddr(0x0))
                    .with_population(1)
            );
            assert_eq!(
                sorted(state_to_hash_map()),
                sorted(HashMap::from_iter(vec![
                    (
                        PhysAddr(0x0000), // PML4
                        HashMap::from_iter(vec![(
                            0usize,
                            Entry::new()
                                .with_present(true)
                                .with_address(PhysAddr(0x1000))
                                .with_population(1)
                        )])
                    ),
                    (
                        PhysAddr(0x1000), // PDP
                        HashMap::from_iter(vec![(
                            0usize,
                            Entry::new()
                                .with_present(true)
                                .with_address(PhysAddr(0x2000))
                                .with_population(2)
                        )])
                    ),
                    (
                        PhysAddr(0x2000), // PD
                        HashMap::from_iter(vec![
                            (
                                0usize,
                                Entry::new()
                                    .with_present(true)
                                    .with_address(PhysAddr(0x3000))
                                    .with_population(2)
                            ),
                            (
                                1usize,
                                Entry::new()
                                    .with_present(true)
                                    .with_address(PhysAddr(0x4000))
                                    .with_population(1)
                            )
                        ])
                    ),
                    (
                        PhysAddr(0x3000), // PT
                        HashMap::from_iter(vec![
                            (
                                0usize,
                                Entry::new()
                                    .with_present(true)
                                    .with_address(PhysAddr(0x1234000))
                                    .with_not_executable(true)
                            ),
                            (
                                1usize,
                                Entry::new()
                                    .with_present(true)
                                    .with_address(PhysAddr(0x2345000))
                                    .with_not_executable(true)
                            )
                        ])
                    ),
                    (
                        PhysAddr(0x4000), // PT
                        HashMap::from_iter(vec![(
                            0usize,
                            Entry::new()
                                .with_present(true)
                                .with_address(PhysAddr(0x3456000))
                                .with_not_executable(true)
                        )])
                    )
                ]))
            );
        }

        {
            assert_eq!(
                map.map_replace(
                    Default::default(),
                    Large,
                    VirtAddr(0x400000),
                    PhysAddr(0x46800000)
                ),
                Ok(None)
            );
            assert_eq!(
                map.roote.read(),
                Entry::new()
                    .with_present(true)
                    .with_address(PhysAddr(0x0))
                    .with_population(1)
            );
            assert_eq!(
                sorted(state_to_hash_map()),
                sorted(HashMap::from_iter(vec![
                    (
                        PhysAddr(0x0000), // PML4
                        HashMap::from_iter(vec![(
                            0usize,
                            Entry::new()
                                .with_present(true)
                                .with_address(PhysAddr(0x1000))
                                .with_population(1)
                        )])
                    ),
                    (
                        PhysAddr(0x1000), // PDP
                        HashMap::from_iter(vec![(
                            0usize,
                            Entry::new()
                                .with_present(true)
                                .with_address(PhysAddr(0x2000))
                                .with_population(3)
                        )])
                    ),
                    (
                        PhysAddr(0x2000), // PD
                        HashMap::from_iter(vec![
                            (
                                0usize,
                                Entry::new()
                                    .with_present(true)
                                    .with_address(PhysAddr(0x3000))
                                    .with_population(2)
                            ),
                            (
                                1usize,
                                Entry::new()
                                    .with_present(true)
                                    .with_address(PhysAddr(0x4000))
                                    .with_population(1)
                            ),
                            (
                                2usize,
                                Entry::new()
                                    .with_present(true)
                                    .with_ps(Level::Pd, true)
                                    .with_address(PhysAddr(0x46800000))
                                    .with_not_executable(true)
                            )
                        ])
                    ),
                    (
                        PhysAddr(0x3000), // PT
                        HashMap::from_iter(vec![
                            (
                                0usize,
                                Entry::new()
                                    .with_present(true)
                                    .with_address(PhysAddr(0x1234000))
                                    .with_not_executable(true)
                            ),
                            (
                                1usize,
                                Entry::new()
                                    .with_present(true)
                                    .with_address(PhysAddr(0x2345000))
                                    .with_not_executable(true)
                            )
                        ])
                    ),
                    (
                        PhysAddr(0x4000), // PT
                        HashMap::from_iter(vec![(
                            0usize,
                            Entry::new()
                                .with_present(true)
                                .with_address(PhysAddr(0x3456000))
                                .with_not_executable(true)
                        )])
                    )
                ]))
            );
        }

        {
            assert_eq!(
                map.unmap(Normal, VirtAddr(0))
                    .map(|x| x.map(|e| e.address())),
                Ok(Some(PhysAddr(0x1234000)))
            );
            assert_eq!(
                map.roote.read(),
                Entry::new()
                    .with_present(true)
                    .with_address(PhysAddr(0x0))
                    .with_population(1)
            );
            assert_eq!(
                sorted(state_to_hash_map()),
                sorted(HashMap::from_iter(vec![
                    (
                        PhysAddr(0x0000), // PML4
                        HashMap::from_iter(vec![(
                            0usize,
                            Entry::new()
                                .with_present(true)
                                .with_address(PhysAddr(0x1000))
                                .with_population(1)
                        )])
                    ),
                    (
                        PhysAddr(0x1000), // PDP
                        HashMap::from_iter(vec![(
                            0usize,
                            Entry::new()
                                .with_present(true)
                                .with_address(PhysAddr(0x2000))
                                .with_population(3)
                        )])
                    ),
                    (
                        PhysAddr(0x2000), // PD
                        HashMap::from_iter(vec![
                            (
                                0usize,
                                Entry::new()
                                    .with_present(true)
                                    .with_address(PhysAddr(0x3000))
                                    .with_population(1)
                            ),
                            (
                                1usize,
                                Entry::new()
                                    .with_present(true)
                                    .with_address(PhysAddr(0x4000))
                                    .with_population(1)
                            ),
                            (
                                2usize,
                                Entry::new()
                                    .with_present(true)
                                    .with_ps(Level::Pd, true)
                                    .with_address(PhysAddr(0x46800000))
                                    .with_not_executable(true)
                            )
                        ])
                    ),
                    (
                        PhysAddr(0x3000), // PT
                        HashMap::from_iter(vec![(
                            1usize,
                            Entry::new()
                                .with_present(true)
                                .with_address(PhysAddr(0x2345000))
                                .with_not_executable(true)
                        )])
                    ),
                    (
                        PhysAddr(0x4000), // PT
                        HashMap::from_iter(vec![(
                            0usize,
                            Entry::new()
                                .with_present(true)
                                .with_address(PhysAddr(0x3456000))
                                .with_not_executable(true)
                        )])
                    )
                ]))
            );
        }

        {
            assert_eq!(
                map.unmap(Normal, VirtAddr(0x1000))
                    .map(|x| x.map(|e| e.address())),
                Ok(Some(PhysAddr(0x2345000)))
            );
            assert_eq!(
                map.roote.read(),
                Entry::new()
                    .with_present(true)
                    .with_address(PhysAddr(0x0))
                    .with_population(1)
            );
            assert_eq!(
                sorted(state_to_hash_map()),
                sorted(HashMap::from_iter(vec![
                    (
                        PhysAddr(0x0000), // PML4
                        HashMap::from_iter(vec![(
                            0usize,
                            Entry::new()
                                .with_present(true)
                                .with_address(PhysAddr(0x1000))
                                .with_population(1)
                        )])
                    ),
                    (
                        PhysAddr(0x1000), // PDP
                        HashMap::from_iter(vec![(
                            0usize,
                            Entry::new()
                                .with_present(true)
                                .with_address(PhysAddr(0x2000))
                                .with_population(2)
                        )])
                    ),
                    (
                        PhysAddr(0x2000), // PD
                        HashMap::from_iter(vec![
                            (
                                1usize,
                                Entry::new()
                                    .with_present(true)
                                    .with_address(PhysAddr(0x4000))
                                    .with_population(1)
                            ),
                            (
                                2usize,
                                Entry::new()
                                    .with_present(true)
                                    .with_ps(Level::Pd, true)
                                    .with_address(PhysAddr(0x46800000))
                                    .with_not_executable(true)
                            )
                        ])
                    ),
                    (
                        PhysAddr(0x4000), // PT
                        HashMap::from_iter(vec![(
                            0usize,
                            Entry::new()
                                .with_present(true)
                                .with_address(PhysAddr(0x3456000))
                                .with_not_executable(true)
                        )])
                    )
                ]))
            );
        }

        {
            assert_eq!(
                map.unmap(Normal, VirtAddr(0x200000))
                    .map(|x| x.map(|e| e.address())),
                Ok(Some(PhysAddr(0x3456000)))
            );
            assert_eq!(
                map.roote.read(),
                Entry::new()
                    .with_present(true)
                    .with_address(PhysAddr(0x0))
                    .with_population(1)
            );
            assert_eq!(
                sorted(state_to_hash_map()),
                sorted(HashMap::from_iter(vec![
                    (
                        PhysAddr(0x0000), // PML4
                        HashMap::from_iter(vec![(
                            0usize,
                            Entry::new()
                                .with_present(true)
                                .with_address(PhysAddr(0x1000))
                                .with_population(1)
                        )])
                    ),
                    (
                        PhysAddr(0x1000), // PDP
                        HashMap::from_iter(vec![(
                            0usize,
                            Entry::new()
                                .with_present(true)
                                .with_address(PhysAddr(0x2000))
                                .with_population(1)
                        )])
                    ),
                    (
                        PhysAddr(0x2000), // PD
                        HashMap::from_iter(vec![(
                            2usize,
                            Entry::new()
                                .with_present(true)
                                .with_ps(Level::Pd, true)
                                .with_address(PhysAddr(0x46800000))
                                .with_not_executable(true)
                        )])
                    )
                ]))
            );
        }

        {
            assert_eq!(
                map.unmap(Large, VirtAddr(0x400000))
                    .map(|x| x.map(|e| e.address())),
                Ok(Some(PhysAddr(0x46800000)))
            );
            assert_eq!(map.roote.read(), Entry::new());
            assert_eq!(sorted(state_to_hash_map()), vec![]);
        }
    }

    #[cfg(not(miri))]
    use test::black_box;
    #[cfg(miri)]
    fn black_box<T>(x: T) -> T {
        x
    }

    struct BenchState {
        tables: HashMap<PhysAddr, BenchTable>,
    }

    struct BenchTable(*mut Table);

    impl Drop for BenchTable {
        fn drop(&mut self) {
            unsafe {
                let _ = Box::from_raw(self.0);
            }
        }
    }

    thread_local! {
        static BENCH_STATE: RefCell<BenchState> = RefCell::new(BenchState { tables: new_hashmap() });
    }

    fn clear_bench_state() {
        BENCH_STATE.with(|bench_state| {
            *bench_state.borrow_mut() = BenchState {
                tables: new_hashmap(),
            }
        });
    }

    unsafe impl Context for BenchState {
        fn alloc_table() -> Result<PhysAddr, OutOfMemory> {
            BENCH_STATE.with(|bench_state| {
                let mut bench_state = bench_state.borrow_mut();
                const NEW_ENTRY_CELL: EntryCell = EntryCell::new();
                let table = Box::leak(Box::new(Table([NEW_ENTRY_CELL; 512])));
                let addr =
                    PhysAddr((table as *mut Table as usize).wrapping_add(0xffff880000000000));
                bench_state.tables.insert(addr, BenchTable(table));
                Ok(black_box(addr))
            })
        }

        unsafe fn dealloc_table(addr: PhysAddr) {
            BENCH_STATE.with(|bench_state| {
                let _ = bench_state.borrow_mut().tables.remove(&addr);
            });
        }

        fn map_table(addr: PhysAddr) -> NonNull<Table> {
            unsafe { NonNull::new_unchecked(addr.0.wrapping_sub(0xffff880000000000) as *mut Table) }
        }
    }

    #[cfg(not(miri))]
    #[bench]
    fn bench_one(b: &mut test::Bencher) {
        clear_bench_state();
        let mut map = Map::<BenchState>::new();

        assert_eq!(
            map.map_replace(Default::default(), Normal, VirtAddr(0), PhysAddr(0)),
            Ok(None)
        );

        b.iter(|| {
            black_box(&mut map)
                .map_replace(
                    Default::default(),
                    Normal,
                    black_box(VirtAddr(0)),
                    black_box(PhysAddr(0)),
                )
                .unwrap();
        });
    }

    #[cfg(not(miri))]
    #[bench]
    fn bench_one_optimal(b: &mut test::Bencher) {
        clear_bench_state();
        let mut map = Map::<BenchState>::new();

        assert_eq!(
            map.map_replace(Default::default(), Normal, VirtAddr(0), PhysAddr(0)),
            Ok(None)
        );

        let addr = black_box(VirtAddr(0));
        let root = black_box(&mut map.roote);
        b.iter(|| {
            assert!(addr.is_aligned(Level::Pt.entry_size()));

            let root_e = root.read();
            if !root_e.present() {
                panic!()
            }

            let pml4: NonNull<EntryCell> = BenchState::map_table(root_e.address()).cast();
            let pml4_i = Level::Pml4.table_index(addr);
            let pml4_ec = unsafe { &*pml4.as_ptr().add(pml4_i) };
            let pml4_e = pml4_ec.read();
            if !pml4_e.present() {
                panic!()
            }

            let pdpt: NonNull<EntryCell> = BenchState::map_table(pml4_e.address()).cast();
            let pdpt_i = Level::Pdpt.table_index(addr);
            let pdpt_ec = unsafe { &*pdpt.as_ptr().add(pdpt_i) };
            let pdpt_e = pdpt_ec.read();
            if !pdpt_e.present() || pdpt_e.pml4e_pdpte_pde_ps() {
                panic!()
            }

            let pd: NonNull<EntryCell> = BenchState::map_table(pdpt_e.address()).cast();
            let pd_i = Level::Pd.table_index(addr);
            let pd_ec = unsafe { &mut *pd.as_ptr().add(pd_i) };
            let pd_e = pd_ec.read();
            if !pd_e.present() || pd_e.pml4e_pdpte_pde_ps() {
                panic!()
            }

            let pt: NonNull<EntryCell> = BenchState::map_table(pd_e.address()).cast();
            let pt_i = Level::Pt.table_index(addr);
            let pt_ec = unsafe { &mut *pt.as_ptr().add(pt_i) };
            let was_present = pt_ec.read().present();
            pt_ec.write(
                Entry::new()
                    .with_present(true)
                    .with_address(black_box(PhysAddr(0x0))),
            );
            if !was_present {
                pd_ec.map(Entry::inc_population);
            }
        })
    }
}
