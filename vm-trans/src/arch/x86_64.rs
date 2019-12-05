use core::cell::UnsafeCell;
use core::cmp::max;
use core::marker::PhantomData;
use core::ops::Range;
use core::ptr;

use brutos_memory::{PhysAddr, VirtAddr};

use brutos_util::iter::{RangeExt, RangeIterable};
use brutos_util::uint::UInt;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Level {
    Pt,
    Pd,
    Pdp,
    Pml4,
    Root,
}

impl Level {
    fn entry_size(self) -> usize {
        assert!(self <= Level::Pml4);
        1 << (12 + (self as u32 * 9))
    }
    fn table_index(self, addr: VirtAddr) -> usize {
        match self {
            Level::Root => 0,
            _ => addr
                .0
                .bits(12 + 9 * self as u32..12 + 9 * (self as u32 + 1)),
        }
    }
}

impl RangeIterable for Level {
    fn up(&self) -> Level {
        match self {
            Level::Pt => Level::Pd,
            Level::Pd => Level::Pdp,
            Level::Pdp => Level::Pml4,
            Level::Pml4 => Level::Root,
            Level::Root => panic!(),
        }
    }

    fn down(&self) -> Level {
        match self {
            Level::Pt => panic!(),
            Level::Pd => Level::Pt,
            Level::Pdp => Level::Pd,
            Level::Pml4 => Level::Pdp,
            Level::Root => Level::Pml4,
        }
    }
}

pub type Table = [EntryStorage; 512];

#[repr(transparent)]
pub struct EntryStorage(UnsafeCell<usize>);

impl EntryStorage {
    fn load(&self) -> Entry {
        unsafe { Entry(ptr::read_volatile(self.0.get())) }
    }

    fn store(&mut self, entry: Entry) {
        unsafe {
            ptr::write_volatile(self.0.get(), entry.0);
        }
    }

    fn map<F>(&mut self, f: F)
    where
        F: FnOnce(Entry) -> Entry,
    {
        self.store(f(self.load()));
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Entry(usize);

impl Entry {
    const IS_PRESENT: u32 = 0;
    const IS_PS: u32 = 7;
    const ADDRESS: Range<u32> = 12..48;
    const POPULATION: Range<u32> = 52..52 + 9;

    pub fn clear(&mut self) {
        self.0 = 0;
    }

    pub fn is_present(&self) -> bool {
        self.0.bit(Self::IS_PRESENT)
    }

    pub fn set_present(&mut self, present: bool) {
        self.0.set_bit(Self::IS_PRESENT, present);
    }

    fn is_ps(&self) -> bool {
        self.0.bit(Self::IS_PS)
    }

    fn set_ps(&mut self, ps: bool) {
        self.0.set_bit(Self::IS_PS, ps);
    }

    pub fn address(&self) -> PhysAddr {
        PhysAddr(self.0 & usize::mask_range(Self::ADDRESS))
    }

    pub fn set_address(&mut self, addr: PhysAddr) {
        assert_eq!(addr.0 & !usize::mask_range(Self::ADDRESS), 0);
        self.0.set_bits(Self::ADDRESS, addr.0.bits(Self::ADDRESS))
    }

    fn population(&self) -> usize {
        self.0.bits(Self::POPULATION)
    }

    fn set_population(&mut self, population: usize) {
        self.0.set_bits(Self::POPULATION, population);
    }

    fn inc_population(&mut self) {
        self.set_population(self.population() + 1);
    }

    fn dec_population(&mut self) {
        self.set_population(self.population() - 1)
    }

    fn is_populated(&self, level: Level) -> bool {
        self.is_present()
            && match level {
                Level::Pt => true,
                Level::Pd | Level::Pdp => self.is_ps() || self.population() > 0,
                Level::Pml4 => self.population() > 0,
                Level::Root => unreachable!(),
            }
    }
}

pub enum EntryTable {
    Available(PhysAddr),
    NotPresent,
    IsPage,
}

impl Entry {
    fn table(&self) -> EntryTable {
        if !self.is_present() {
            EntryTable::NotPresent
        } else if self.is_ps() {
            EntryTable::IsPage
        } else {
            EntryTable::Available(self.address())
        }
    }
}

pub unsafe trait TableOps {
    type AllocErr;

    fn alloc(&mut self) -> Result<PhysAddr, Self::AllocErr>;
    unsafe fn dealloc(&mut self, addr: PhysAddr);
    fn map(&mut self, addr: PhysAddr) -> *mut Table;

    fn new_table(&mut self) -> Result<(PhysAddr, *mut Table), Self::AllocErr> {
        let addr = self.alloc()?;
        let table = self.map(addr);
        unsafe {
            ptr::write_bytes(table, 0u8, 1usize);
        }
        Ok((addr, table))
    }
}

pub struct Trail<'a, TblOps: TableOps> {
    tbl_ops: TblOps,
    crumbs: [*mut EntryStorage; 5],
    valid_lvl: Level,
    addr: VirtAddr,
    _marker: PhantomData<&'a mut Entry>,
}

pub struct TrailEntry<'a: 'b, 'b, TblOps: TableOps> {
    trail: &'b mut Trail<'a, TblOps>,
}

pub enum Error<TblOps: TableOps> {
    AllocTable(TblOps::AllocErr),
    Obstructed,
}

impl<TblOps: TableOps> core::fmt::Debug for Error<TblOps>
where
    TblOps::AllocErr: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::AllocTable(e) => f.debug_tuple("Error::AllocTable").field(e).finish(),
            Error::Obstructed => write!(f, "Error::Obstructed"),
        }
    }
}

impl<'a, TblOps: TableOps> Trail<'a, TblOps> {
    pub fn with_root_entry(tbl_ops: TblOps, root_entry: &mut EntryStorage) -> Trail<TblOps> {
        Trail {
            tbl_ops,
            crumbs: [
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                root_entry,
            ],
            valid_lvl: Level::Root,
            addr: VirtAddr(0),
            _marker: PhantomData,
        }
    }

    fn entry_ptr(&self, level: Level) -> *mut EntryStorage {
        assert!(level >= self.valid_lvl);
        let i = level.table_index(self.addr);
        unsafe { self.crumbs[level as usize].add(i) }
    }

    fn entry(&self, level: Level) -> &EntryStorage {
        unsafe { &*self.entry_ptr(level) }
    }

    fn entry_mut(&mut self, level: Level) -> &mut EntryStorage {
        unsafe { &mut *self.entry_ptr(level) }
    }

    pub fn find<'b>(
        &'b mut self,
        allocate: bool,
        leaf_lvl: Level,
        addr: VirtAddr,
    ) -> Result<Option<TrailEntry<'a, 'b, TblOps>>, Error<TblOps>> {
        debug_assert!(addr.is_aligned(leaf_lvl.entry_size()));

        let next_valid_lvl = self.valid_level_to_find(leaf_lvl, addr);
        if next_valid_lvl > self.valid_lvl {
            self.pop_to(next_valid_lvl);
        }
        self.dig_to(allocate, leaf_lvl, addr)
            .map(move |available| available.map(move |()| TrailEntry { trail: self }))
    }

    fn valid_level_to_find(&self, leaf_lvl: Level, addr: VirtAddr) -> Level {
        let mut next_valid_lvl = Level::Root;
        for lvl in (max(self.valid_lvl, leaf_lvl)..Level::Root).iter().rev() {
            next_valid_lvl = lvl;
            if lvl.table_index(self.addr) != lvl.table_index(addr) {
                break;
            }
        }
        next_valid_lvl
    }

    fn pop_to(&mut self, to_lvl: Level) {
        assert!(to_lvl >= self.valid_lvl);
        for lvl in (self.valid_lvl..to_lvl).iter() {
            self.valid_lvl = lvl;
            if !self.entry(lvl).load().is_populated(lvl) {
                self.clear_entry(lvl);
            } else {
                break;
            }
        }
        self.valid_lvl = to_lvl;
    }

    fn clear_entry(&mut self, level: Level) {
        let parent_entry_s = self.entry_mut(level.up());
        let mut parent_entry = parent_entry_s.load();
        parent_entry.dec_population();
        if parent_entry.population() > 0 {
            parent_entry_s.store(parent_entry);
        } else {
            let table_addr = parent_entry.address();
            parent_entry_s.store(Entry(0));
            unsafe {
                self.tbl_ops.dealloc(table_addr);
            }
        }
    }

    fn dig_to(
        &mut self,
        allocate: bool,
        leaf_lvl: Level,
        addr: VirtAddr,
    ) -> Result<Option<()>, Error<TblOps>> {
        assert!(leaf_lvl <= self.valid_lvl);
        self.addr = addr;
        for lvl in (leaf_lvl.up()..=self.valid_lvl).iter().rev() {
            let next_table = match self.entry(lvl).load().table() {
                EntryTable::IsPage => return Err(Error::Obstructed),
                EntryTable::NotPresent if !allocate => return Ok(None),
                EntryTable::Available(table_addr) => self.tbl_ops.map(table_addr),
                EntryTable::NotPresent => self.create_table(lvl)?,
            };
            let next_lvl = lvl.down();
            self.crumbs[next_lvl as usize] = unsafe { &mut (*next_table)[0] as *mut _ };
            self.valid_lvl = next_lvl;
        }
        Ok(Some(()))
    }

    fn create_table(&mut self, lvl: Level) -> Result<*mut Table, Error<TblOps>> {
        let (table_addr, table) = self.tbl_ops.new_table().map_err(Error::AllocTable)?;
        let mut entry = Entry(0);
        entry.set_address(table_addr);
        entry.inc_population();
        entry.set_present(true);
        self.entry_mut(lvl).store(entry);
        Ok(table)
    }
}

impl<'a, TblOps: TableOps> Drop for Trail<'a, TblOps> {
    fn drop(&mut self) {
        self.pop_to(Level::Root);
    }
}

impl<'a, 'b, TblOps: TableOps> TrailEntry<'a, 'b, TblOps> {
    pub fn map_page(&mut self, addr: PhysAddr) {
        let lvl = self.trail.valid_lvl;
        assert!(addr.is_aligned(lvl.entry_size()));

        let entry_s = self.trail.entry_mut(lvl);
        let was_present = entry_s.load().is_present();

        let mut entry = Entry(0);
        entry.set_present(true);
        entry.set_address(addr);
        if lvl > Level::Pt {
            entry.set_ps(true);
        }
        entry_s.store(entry);

        if !was_present {
            self.trail.entry_mut(lvl.up()).map(|mut parent_entry| {
                parent_entry.inc_population();
                parent_entry
            });
        }
    }

    pub fn unmap_page(&mut self) {
        let lvl = self.trail.valid_lvl;
        let entry_s = self.trail.entry_mut(lvl);
        if !entry_s.load().is_present() {
            return;
        }
        entry_s.store(Entry(0));
        self.trail.entry_mut(lvl.up()).map(|mut parent_entry| {
            parent_entry.dec_population();
            parent_entry
        });
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use std::collections::HashMap;
    use test::black_box;

    use brutos_memory::arch::PAGE_SIZE;

    struct State {
        tables: HashMap<PhysAddr, *mut Table>,
        addr: PhysAddr,
    }

    unsafe impl<'a> TableOps for &'a mut State {
        type AllocErr = !;

        #[inline(never)]
        fn alloc(&mut self) -> Result<PhysAddr, !> {
            const EMPTY_ENTRY_STORAGE: EntryStorage = EntryStorage(UnsafeCell::new(0));
            self.tables
                .insert(self.addr, Box::leak(Box::new([EMPTY_ENTRY_STORAGE; 512])));
            let addr = self.addr;
            self.addr = self.addr + PAGE_SIZE;
            Ok(addr)
        }

        #[inline(never)]
        unsafe fn dealloc(&mut self, addr: PhysAddr) {
            let ptr = self.tables.remove(&addr).unwrap();
            drop(Box::from_raw(ptr));
        }

        #[inline(never)]
        fn map(&mut self, addr: PhysAddr) -> *mut Table {
            self.tables.get(&addr).cloned().unwrap()
        }
    }

    impl Drop for State {
        fn drop(&mut self) {
            for &ptr in self.tables.values() {
                drop(unsafe { Box::from_raw(ptr) });
            }
        }
    }

    #[test]
    fn find() {
        let mut state = State {
            tables: HashMap::new(),
            addr: PhysAddr(0),
        };
        let mut root = EntryStorage(UnsafeCell::new(0));
        let mut trail = Trail::with_root_entry(black_box(&mut state), &mut root);
        let mut entry = trail
            .find(
                true,
                black_box(Level::Pt),
                black_box(VirtAddr(
                    (1 << (12 + 3 * 9))
                        | (2 << (12 + 2 * 9))
                        | (3 << (12 + 1 * 9))
                        | (4 << (12 + 0 * 9)),
                )),
            )
            .unwrap()
            .unwrap();
        entry.map_page(PhysAddr(0x1000));
        drop(entry);
        drop(trail);
    }
}
