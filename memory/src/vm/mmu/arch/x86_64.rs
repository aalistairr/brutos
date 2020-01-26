use core::cell::UnsafeCell;
use core::fmt;
use core::ops::{Not, Range};
use core::ptr;

use crate::arch::PAGE_SIZE;
use crate::{PhysAddr, VirtAddr};
use brutos_alloc::OutOfMemory;
use brutos_util::uint::UInt;

use super::super::{Flags, MapError, PageSize, UnmapError};

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct Entry(usize);

impl Entry {
    const IS_PRESENT: u32 = 0;
    const IS_PS: u32 = 7;
    const IS_USER_ACCESSIBLE: u32 = 2;
    const IS_WRITABLE: u32 = 1;
    const IS_NOT_EXECUTABLE: u32 = 63;
    const IS_GLOBAL: u32 = 8;
    const IS_CACHE_DISABLED: u32 = 4;
    const IS_WRITETHROUGH: u32 = 3;
    const ADDRESS: Range<u32> = 12..48;
    const POPULATION: Range<u32> = 52..52 + 11;

    pub fn new() -> Entry {
        Entry(0)
    }

    pub fn is_present(&self) -> bool {
        self.0.bit(Self::IS_PRESENT)
    }

    pub fn with_present(self, present: bool) -> Entry {
        Entry(self.0.with_bit(Self::IS_PRESENT, present))
    }

    pub fn is_ps(&self) -> bool {
        self.0.bit(Self::IS_PS)
    }

    pub fn with_ps(self, ps: bool) -> Entry {
        Entry(self.0.with_bit(Self::IS_PS, ps))
    }

    pub fn is_user_accessible(&self) -> bool {
        self.0.bit(Self::IS_USER_ACCESSIBLE)
    }

    pub fn with_user_accessible(self, user_accessible: bool) -> Entry {
        Entry(self.0.with_bit(Self::IS_USER_ACCESSIBLE, user_accessible))
    }

    pub fn is_writable(&self) -> bool {
        self.0.bit(Self::IS_WRITABLE)
    }

    pub fn with_writable(self, writable: bool) -> Entry {
        Entry(self.0.with_bit(Self::IS_WRITABLE, writable))
    }

    pub fn is_not_executable(&self) -> bool {
        self.0.bit(Self::IS_NOT_EXECUTABLE)
    }

    pub fn with_not_executable(self, not_executable: bool) -> Entry {
        Entry(self.0.with_bit(Self::IS_NOT_EXECUTABLE, not_executable))
    }

    pub fn is_global(&self) -> bool {
        self.0.bit(Self::IS_GLOBAL)
    }

    pub fn with_global(self, global: bool) -> Entry {
        Entry(self.0.with_bit(Self::IS_GLOBAL, global))
    }

    pub fn is_cache_disabled(&self) -> bool {
        self.0.bit(Self::IS_CACHE_DISABLED)
    }

    pub fn with_cache_disabled(self, cache_disabled: bool) -> Entry {
        Entry(self.0.with_bit(Self::IS_CACHE_DISABLED, cache_disabled))
    }

    pub fn is_writethrough(&self) -> bool {
        self.0.bit(Self::IS_WRITETHROUGH)
    }

    pub fn with_writethrough(self, writethrough: bool) -> Entry {
        Entry(self.0.with_bit(Self::IS_WRITETHROUGH, writethrough))
    }

    pub fn address(&self) -> PhysAddr {
        PhysAddr(self.0 & usize::mask_range(Self::ADDRESS))
    }

    pub fn with_address(self, addr: PhysAddr) -> Entry {
        assert_eq!(addr.0 & !usize::mask_range(Self::ADDRESS), 0);
        Entry(self.0.with_bits(Self::ADDRESS, addr.0.bits(Self::ADDRESS)))
    }

    pub fn population(&self) -> usize {
        self.0.bits(Self::POPULATION)
    }

    pub fn with_population(self, population: usize) -> Entry {
        Entry(self.0.with_bits(Self::POPULATION, population))
    }

    pub fn with_inc_population(self) -> Entry {
        self.with_population(self.population() + 1)
    }

    pub fn with_dec_population(self) -> Entry {
        self.with_population(self.population() - 1)
    }
}

impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Entry({:#018x})", self.0)
    }
}

#[repr(C, align(4096))]
pub struct Table([EntryCell; 512]);

#[repr(transparent)]
pub struct EntryCell(UnsafeCell<usize>);

impl EntryCell {
    pub const fn new() -> EntryCell {
        EntryCell(UnsafeCell::new(0))
    }

    #[allow(dead_code)]
    fn load(&self) -> Entry {
        unsafe { Entry(ptr::read_volatile(self.0.get())) }
    }

    fn store(&mut self, entry: Entry) {
        unsafe { ptr::write_volatile(self.0.get(), entry.0) }
    }

    #[allow(dead_code)]
    fn map<F: FnOnce(Entry) -> Entry>(&mut self, f: F) {
        self.store(f(self.load()))
    }

    fn load_nonvolatile(&self) -> Entry {
        unsafe { Entry(ptr::read(self.0.get())) }
    }

    fn store_nonvolatile(&mut self, entry: Entry) {
        unsafe { ptr::write(self.0.get(), entry.0) }
    }

    fn map_nonvolatile<F: FnOnce(Entry) -> Entry>(&mut self, f: F) {
        self.store_nonvolatile(f(self.load_nonvolatile()))
    }
}

impl PageSize {
    pub fn order(&self) -> u8 {
        match self {
            PageSize::Normal => 0,
            PageSize::Large => 9,
            PageSize::Huge => 9 * 2,
        }
    }

    pub fn size(&self) -> usize {
        PAGE_SIZE << self.order()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Level {
    Pt = 0,
    Pd,
    Pdp,
    Pml4,
    Root,
}

fn addr_bits(lvl: Level) -> Range<u32> {
    12 + 9 * lvl as u32..12 + 9 * (lvl as u32 + 1)
}

fn entry_size(lvl: Level) -> usize {
    1 << addr_bits(lvl).start
}

fn table_index(lvl: Level, addr: VirtAddr) -> usize {
    addr.0.bits(addr_bits(lvl))
}

pub unsafe trait Context {
    fn alloc_table(&mut self) -> Result<PhysAddr, OutOfMemory>;
    unsafe fn dealloc_table(&mut self, addr: PhysAddr);
    fn map_table(&mut self, addr: PhysAddr) -> *mut Table;

    fn new_table(&mut self) -> Result<(PhysAddr, *mut Table), OutOfMemory> {
        let addr = self.alloc_table()?;
        let table = self.map_table(addr);
        unsafe {
            ptr::write_bytes(table, 0u8, 1usize);
        }
        Ok((addr, table))
    }
}

struct Trail<'cx, 'root, Cx: Context, const ALLOC: bool, const SKIP_DROP: bool> {
    cx: &'cx mut Cx,
    root: &'root mut EntryCell,
    pml4e: *mut EntryCell,
    pdpe: *mut EntryCell,
    pde: *mut EntryCell,
    pte: *mut EntryCell,
    valid_lvl: Level,
    skip_drop: bool,
}

impl<'cx, 'root, Cx: Context, const ALLOC: bool, const SKIP_DROP: bool>
    Trail<'cx, 'root, Cx, ALLOC, SKIP_DROP>
{
    fn new(cx: &'cx mut Cx, root: &'root mut EntryCell) -> Trail<'cx, 'root, Cx, ALLOC, SKIP_DROP> {
        Trail {
            cx,
            root,
            pml4e: ptr::null_mut(),
            pdpe: ptr::null_mut(),
            pde: ptr::null_mut(),
            pte: ptr::null_mut(),
            valid_lvl: Level::Root,
            skip_drop: false,
        }
    }

    fn find_entry(
        &mut self,
        lvl: Level,
        addr: VirtAddr,
    ) -> Result<(&mut EntryCell, Option<&mut EntryCell>), MapError> {
        fn create_table<Cx: Context>(
            cx: &mut Cx,
            entry_cell: &mut EntryCell,
            parent_entry_cell: Option<&mut EntryCell>,
        ) -> Result<Entry, MapError> {
            let (addr, _) = cx.new_table()?;
            let entry = Entry::new().with_present(true).with_address(addr);
            entry_cell.store(entry);
            if let Some(parent_entry_cell) = parent_entry_cell {
                parent_entry_cell.map_nonvolatile(Entry::with_inc_population);
            }
            Ok(entry)
        }
        fn dig<'a, Cx: Context, const ALLOC: bool>(
            cx: &mut Cx,
            entry_cell: &mut EntryCell,
            parent_entry_cell: Option<&mut EntryCell>,
        ) -> Result<&'a mut Table, MapError> {
            let mut entry = entry_cell.load_nonvolatile();
            if !entry.is_present() {
                if ALLOC {
                    entry = create_table(cx, entry_cell, parent_entry_cell)?;
                } else {
                    return Err(MapError::NotAllocated);
                }
            }
            let table = cx.map_table(entry.address());
            let table = unsafe { &mut *table };
            Ok(table)
        }

        self.valid_lvl = Level::Root;
        if lvl == Level::Root {
            if SKIP_DROP {
                self.skip_drop = true;
            }
            return Ok((self.root, None));
        }

        let pml4 = dig::<_, ALLOC>(self.cx, &mut self.root, None)?;
        let pml4e = &mut pml4.0[table_index(Level::Pml4, addr)];
        self.pml4e = pml4e;
        self.valid_lvl = Level::Pml4;
        if lvl == Level::Pml4 {
            if SKIP_DROP {
                self.skip_drop = true;
            }
            return Ok((pml4e, Some(self.root)));
        }

        let pdp = dig::<_, ALLOC>(self.cx, pml4e, Some(&mut self.root))?;
        let pdpe = &mut pdp.0[table_index(Level::Pdp, addr)];
        self.pdpe = pdpe;
        self.valid_lvl = Level::Pdp;
        if lvl == Level::Pdp {
            if SKIP_DROP {
                self.skip_drop = true;
            }
            return Ok((pdpe, Some(pml4e)));
        }

        let pd = dig::<_, ALLOC>(self.cx, pdpe, Some(pml4e))?;
        let pde = &mut pd.0[table_index(Level::Pd, addr)];
        self.pde = pde;
        self.valid_lvl = Level::Pd;
        if lvl == Level::Pd {
            if SKIP_DROP {
                self.skip_drop = true;
            }
            return Ok((pde, Some(pdpe)));
        }

        let pt = dig::<_, ALLOC>(self.cx, pde, Some(pdpe))?;
        let pte = &mut pt.0[table_index(Level::Pt, addr)];
        self.pte = pte;
        self.valid_lvl = Level::Pt;
        if lvl == Level::Pt {
            if SKIP_DROP {
                self.skip_drop = true;
            }
            return Ok((pte, Some(pde)));
        }

        unreachable!();
    }
}

impl<'cx, 'root, Cx: Context, const ALLOC: bool, const SKIP_DROP: bool> Drop
    for Trail<'cx, 'root, Cx, ALLOC, SKIP_DROP>
{
    fn drop(&mut self) {
        fn pop<Cx: Context>(
            cx: &mut Cx,
            is_ps: fn(Entry) -> bool,
            entry_cell: &mut EntryCell,
            parent_entry_cell: Option<&mut EntryCell>,
        ) -> bool {
            let entry = entry_cell.load_nonvolatile();
            if entry.is_present() {
                if is_ps(entry) || entry.population() > 0 {
                    return true;
                }
                entry_cell.store(Entry::new());
                unsafe {
                    cx.dealloc_table(entry.address());
                }
                if let Some(parent_entry_cell) = parent_entry_cell {
                    parent_entry_cell.map_nonvolatile(Entry::with_dec_population);
                }
            }
            false
        }

        fn pde_pdpe_check(entry: Entry) -> bool {
            entry.is_ps()
        }

        fn pml4e_roote_check(_: Entry) -> bool {
            false
        }

        if SKIP_DROP && (!ALLOC || self.skip_drop) {
            return;
        }

        if self.valid_lvl <= Level::Pd {
            let (pde, pdpe) = unsafe { (&mut *self.pde, &mut *self.pdpe) };
            if pop(self.cx, pde_pdpe_check, pde, Some(pdpe)) {
                return;
            }
        }
        if self.valid_lvl <= Level::Pdp {
            let (pdpe, pml4e) = unsafe { (&mut *self.pdpe, &mut *self.pml4e) };
            if pop(self.cx, pde_pdpe_check, pdpe, Some(pml4e)) {
                return;
            }
        }
        if self.valid_lvl <= Level::Pml4 {
            let pml4e = unsafe { &mut *self.pml4e };
            if pop(self.cx, pml4e_roote_check, pml4e, Some(self.root)) {
                return;
            }
        }
        if self.valid_lvl <= Level::Root {
            if pop(self.cx, pml4e_roote_check, self.root, None) {
                return;
            }
        }
    }
}

pub fn map_entry_replace<Cx: Context, const ALLOC: bool>(
    cx: &mut Cx,
    root: &mut EntryCell,
    lvl: Level,
    virt_addr: VirtAddr,
    phys_addr: PhysAddr,
    flags: Flags,
) -> Result<Option<PhysAddr>, MapError> {
    assert!(lvl < Level::Root);
    assert!(virt_addr.is_aligned(entry_size(lvl)));

    let mut trail = Trail::<_, ALLOC, true>::new(cx, root);
    let (entry_cell, parent_entry_cell) = trail.find_entry(lvl, virt_addr)?;

    let old_entry = entry_cell.load_nonvolatile();
    entry_cell.store(
        Entry::new()
            .with_present(true)
            .with_address(phys_addr)
            .with_ps(lvl > Level::Pt)
            .with_user_accessible(flags.user_accessible)
            .with_writable(flags.writable)
            .with_not_executable(flags.executable.not())
            .with_global(flags.global)
            .with_cache_disabled(flags.cache_disabled)
            .with_writethrough(flags.writethrough),
    );

    if !old_entry.is_present() {
        parent_entry_cell
            .unwrap()
            .map_nonvolatile(Entry::with_inc_population);
        Ok(None)
    } else {
        invlpg(virt_addr);
        Ok(Some(old_entry.address()))
    }
}

pub use self::map_entry_replace as map_entry;

pub fn map_entry_keep<Cx: Context, const ALLOC: bool>(
    cx: &mut Cx,
    root: &mut EntryCell,
    lvl: Level,
    virt_addr: VirtAddr,
    phys_addr: PhysAddr,
    flags: Flags,
) -> Result<bool, MapError> {
    assert!(lvl < Level::Root);
    assert!(virt_addr.is_aligned(entry_size(lvl)));

    let mut trail = Trail::<_, ALLOC, true>::new(cx, root);
    let (entry_cell, parent_entry_cell) = trail.find_entry(lvl, virt_addr)?;

    let old_entry = entry_cell.load_nonvolatile();
    if old_entry.is_present() {
        return Ok(false);
    }

    entry_cell.store(
        Entry::new()
            .with_present(true)
            .with_address(phys_addr)
            .with_ps(lvl > Level::Pt)
            .with_user_accessible(flags.user_accessible)
            .with_writable(flags.writable)
            .with_not_executable(flags.executable.not())
            .with_global(flags.global)
            .with_cache_disabled(flags.cache_disabled)
            .with_writethrough(flags.writethrough),
    );

    parent_entry_cell
        .unwrap()
        .map_nonvolatile(Entry::with_inc_population);
    Ok(true)
}

pub fn unmap_entry<Cx: Context, const ALLOC: bool>(
    cx: &mut Cx,
    root: &mut EntryCell,
    lvl: Level,
    virt_addr: VirtAddr,
) -> Result<Option<PhysAddr>, UnmapError> {
    assert!(lvl < Level::Root);
    assert!(virt_addr.is_aligned(entry_size(lvl)));

    let mut trail = Trail::<_, ALLOC, false>::new(cx, root);
    let (entry_cell, parent_entry_cell) =
        trail.find_entry(lvl, virt_addr).map_err(|e| match e {
            MapError::OutOfMemory => unreachable!(),
            MapError::NotAllocated => UnmapError::NotAllocated,
            MapError::Obstructed => UnmapError::Obstructed,
        })?;

    let old_entry = entry_cell.load_nonvolatile();
    entry_cell.store(Entry::new());

    if old_entry.is_present() {
        invlpg(virt_addr);
        parent_entry_cell
            .unwrap()
            .map_nonvolatile(Entry::with_dec_population);
        Ok(Some(old_entry.address()))
    } else {
        Ok(None)
    }
}

pub fn get_entry<Cx: Context>(
    cx: &mut Cx,
    root: &mut EntryCell,
    lvl: Level,
    virt_addr: VirtAddr,
) -> Result<Option<Entry>, MapError> {
    assert!(lvl < Level::Root);
    assert!(virt_addr.is_aligned(entry_size(lvl)));

    let mut trail = Trail::<_, false, false>::new(cx, root);
    let (entry_cell, _) = trail.find_entry(lvl, virt_addr)?;
    let entry = entry_cell.load_nonvolatile();
    if entry.is_present() {
        Ok(Some(entry))
    } else {
        Ok(None)
    }
}

pub fn compare_and_swap<Cx: Context>(
    cx: &mut Cx,
    root: &mut EntryCell,
    virt_addr: VirtAddr,
    lvl: Level,
    current: Entry,
    phys_addr: PhysAddr,
    flags: Flags,
) -> Result<bool, MapError> {
    assert!(lvl < Level::Root);
    assert!(virt_addr.is_aligned(entry_size(lvl)));
    assert!(current.is_present());

    let mut trail = Trail::<_, false, true>::new(cx, root);
    let (entry_cell, _) = trail.find_entry(lvl, virt_addr)?;

    let old_entry = entry_cell.load_nonvolatile();
    if old_entry != current {
        return Ok(false);
    }

    entry_cell.store(
        Entry::new()
            .with_present(true)
            .with_address(phys_addr)
            .with_ps(lvl > Level::Pt)
            .with_user_accessible(flags.user_accessible)
            .with_writable(flags.writable)
            .with_not_executable(flags.executable.not())
            .with_global(flags.global)
            .with_cache_disabled(flags.cache_disabled)
            .with_writethrough(flags.writethrough),
    );

    invlpg(virt_addr);
    Ok(true)
}

#[cfg(not(any(test, miri, feature = "std")))]
fn invlpg(addr: VirtAddr) {
    unsafe {
        asm!("invlpg ($0)" :: "r" (addr.0) :: "volatile");
    }
}
#[cfg(any(test, miri, feature = "std"))]
fn invlpg(_: VirtAddr) {}

impl PageSize {
    pub fn level(&self) -> Level {
        match self {
            PageSize::Normal => Level::Pt,
            PageSize::Large => Level::Pd,
            PageSize::Huge => Level::Pdp,
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(not(miri))]
    extern crate test;
    use super::*;

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
        tables: HashMap<PhysAddr, *mut Table>,
        addr: PhysAddr,
    }

    unsafe impl Context for State {
        #[inline(never)]
        fn alloc_table(&mut self) -> Result<PhysAddr, OutOfMemory> {
            const EMPTY_ENTRY_CELL: EntryCell = EntryCell(UnsafeCell::new(0));
            self.tables.insert(
                self.addr,
                Box::leak(Box::new(Table([EMPTY_ENTRY_CELL; 512]))),
            );
            let addr = self.addr;
            self.addr = self.addr + PAGE_SIZE;
            Ok(addr)
        }

        #[inline(never)]
        unsafe fn dealloc_table(&mut self, addr: PhysAddr) {
            let ptr = self.tables.remove(&addr).unwrap();
            drop(Box::from_raw(ptr));
        }

        #[inline(never)]
        fn map_table(&mut self, addr: PhysAddr) -> *mut Table {
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

    impl State {
        fn to_hash_map(&self) -> HashMap<PhysAddr, HashMap<usize, Entry>> {
            let mut x = new_hashmap();
            for (&addr, &table) in &self.tables {
                let table = unsafe { &*table };
                let mut entries = new_hashmap();
                for (i, entry) in table.0.iter().map(EntryCell::load).enumerate() {
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

    const ALLOC: bool = true;
    const NO_ALLOC: bool = false;

    const PT: Level = Level::Pt;
    const PD: Level = Level::Pd;
    #[allow(dead_code)]
    const PDP: Level = Level::Pdp;
    #[allow(dead_code)]
    const PML4: Level = Level::Pml4;
    #[test]
    fn check() {
        let mut state = State {
            tables: new_hashmap(),
            addr: PhysAddr(0),
        };
        let mut root = EntryCell(UnsafeCell::new(0));

        let cx = &mut state;
        let root = &mut root;

        assert_eq!(
            map_entry::<_, NO_ALLOC>(cx, root, PT, VirtAddr(0), PhysAddr(0), Default::default()),
            Err(MapError::NotAllocated)
        );
        assert_eq!(root.load(), Entry::new());
        assert_eq!(cx.to_hash_map(), new_hashmap());

        {
            assert_eq!(
                map_entry::<_, ALLOC>(
                    cx,
                    root,
                    PT,
                    VirtAddr(0),
                    PhysAddr(0x1234000),
                    Default::default()
                ),
                Ok(None)
            );
            assert_eq!(
                root.load(),
                Entry::new()
                    .with_present(true)
                    .with_address(PhysAddr(0x0))
                    .with_population(1)
            );
            assert_eq!(
                sorted(cx.to_hash_map()),
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
                map_entry::<_, NO_ALLOC>(
                    cx,
                    root,
                    PT,
                    VirtAddr(0x1000),
                    PhysAddr(0x2345000),
                    Default::default()
                ),
                Ok(None)
            );
            assert_eq!(
                root.load(),
                Entry::new()
                    .with_present(true)
                    .with_address(PhysAddr(0x0))
                    .with_population(1)
            );
            assert_eq!(
                sorted(cx.to_hash_map()),
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
                map_entry::<_, ALLOC>(
                    cx,
                    root,
                    PT,
                    VirtAddr(0x200000),
                    PhysAddr(0x3456000),
                    Default::default()
                ),
                Ok(None)
            );
            assert_eq!(
                root.load(),
                Entry::new()
                    .with_present(true)
                    .with_address(PhysAddr(0x0))
                    .with_population(1)
            );
            assert_eq!(
                sorted(cx.to_hash_map()),
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
                map_entry::<_, ALLOC>(
                    cx,
                    root,
                    PD,
                    VirtAddr(0x400000),
                    PhysAddr(0x46800000),
                    Default::default()
                ),
                Ok(None)
            );
            assert_eq!(
                root.load(),
                Entry::new()
                    .with_present(true)
                    .with_address(PhysAddr(0x0))
                    .with_population(1)
            );
            assert_eq!(
                sorted(cx.to_hash_map()),
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
                                    .with_ps(true)
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
                unmap_entry::<_, NO_ALLOC>(cx, root, PT, VirtAddr(0)),
                Ok(Some(PhysAddr(0x1234000)))
            );
            assert_eq!(
                root.load(),
                Entry::new()
                    .with_present(true)
                    .with_address(PhysAddr(0x0))
                    .with_population(1)
            );
            assert_eq!(
                sorted(cx.to_hash_map()),
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
                                    .with_ps(true)
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
                unmap_entry::<_, NO_ALLOC>(cx, root, PT, VirtAddr(0x1000)),
                Ok(Some(PhysAddr(0x2345000)))
            );
            assert_eq!(
                root.load(),
                Entry::new()
                    .with_present(true)
                    .with_address(PhysAddr(0x0))
                    .with_population(1)
            );
            assert_eq!(
                sorted(cx.to_hash_map()),
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
                                    .with_ps(true)
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
                unmap_entry::<_, NO_ALLOC>(cx, root, PT, VirtAddr(0x200000)),
                Ok(Some(PhysAddr(0x3456000)))
            );
            assert_eq!(
                root.load(),
                Entry::new()
                    .with_present(true)
                    .with_address(PhysAddr(0x0))
                    .with_population(1)
            );
            assert_eq!(
                sorted(cx.to_hash_map()),
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
                                .with_ps(true)
                                .with_address(PhysAddr(0x46800000))
                                .with_not_executable(true)
                        )])
                    )
                ]))
            );
        }

        {
            assert_eq!(
                unmap_entry::<_, NO_ALLOC>(cx, root, PD, VirtAddr(0x400000)),
                Ok(Some(PhysAddr(0x46800000)))
            );
            assert_eq!(root.load(), Entry::new());
            assert_eq!(sorted(cx.to_hash_map()), vec![]);
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

    unsafe impl Context for BenchState {
        fn alloc_table(&mut self) -> Result<PhysAddr, OutOfMemory> {
            const NEW_ENTRY_CELL: EntryCell = EntryCell::new();
            let table = Box::leak(Box::new(Table([NEW_ENTRY_CELL; 512])));
            let addr = PhysAddr((table as *mut Table as usize).wrapping_add(0xffff880000000000));
            self.tables.insert(addr, BenchTable(table));
            Ok(black_box(addr))
        }

        unsafe fn dealloc_table(&mut self, addr: PhysAddr) {
            let _ = self.tables.remove(&addr);
        }

        fn map_table(&mut self, addr: PhysAddr) -> *mut Table {
            (addr.0.wrapping_sub(0xffff880000000000)) as *mut Table
        }
    }

    #[cfg(not(miri))]
    #[bench]
    fn bench_one(b: &mut test::Bencher) {
        let mut cx = BenchState {
            tables: new_hashmap(),
        };
        let mut root = EntryCell::new();

        let cx = &mut cx;
        let root = &mut root;

        assert_eq!(
            map_entry::<_, ALLOC>(cx, root, PT, VirtAddr(0), PhysAddr(0), Default::default()),
            Ok(None)
        );

        b.iter(|| {
            map_entry::<_, NO_ALLOC>(
                cx,
                black_box(root),
                PT,
                black_box(VirtAddr(0)),
                black_box(PhysAddr(0)),
                Default::default(),
            )
            .unwrap();
        });
    }

    #[cfg(not(miri))]
    #[bench]
    fn bench_one_optimal(b: &mut test::Bencher) {
        let mut cx = BenchState {
            tables: new_hashmap(),
        };
        let mut root = EntryCell::new();

        assert_eq!(
            map_entry::<_, ALLOC>(
                &mut cx,
                &mut root,
                PT,
                VirtAddr(0),
                PhysAddr(0),
                Default::default()
            ),
            Ok(None)
        );

        let addr = VirtAddr(0);
        b.iter(|| {
            let addr = black_box(addr);
            let root = black_box(&mut root);

            assert!(addr.is_aligned(entry_size(PT)));

            let root_e = root.load_nonvolatile();
            if !root_e.is_present() {
                panic!()
            }

            let pml4 = cx.map_table(root_e.address()) as *mut EntryCell;
            let pml4_i = table_index(Level::Pml4, addr);
            let pml4_ec = unsafe { &*pml4.add(pml4_i) };
            let pml4_e = pml4_ec.load();
            if !pml4_e.is_present() {
                panic!()
            }

            let pdp = cx.map_table(pml4_e.address()) as *mut EntryCell;
            let pdp_i = table_index(Level::Pdp, addr);
            let pdp_ec = unsafe { &*pdp.add(pdp_i) };
            let pdp_e = pdp_ec.load();
            if !pdp_e.is_present() || pdp_e.is_ps() {
                panic!()
            }

            let pd = cx.map_table(pdp_e.address()) as *mut EntryCell;
            let pd_i = table_index(Level::Pd, addr);
            let pd_ec = unsafe { &mut *pd.add(pd_i) };
            let pd_e = pd_ec.load();
            if !pd_e.is_present() || pd_e.is_ps() {
                panic!()
            }

            let pt = cx.map_table(pd_e.address()) as *mut EntryCell;
            let pt_i = table_index(Level::Pt, addr);
            let pt_ec = unsafe { &mut *pt.add(pt_i) };
            let was_present = pt_ec.load().is_present();
            pt_ec.store(
                Entry::new()
                    .with_present(true)
                    .with_address(black_box(PhysAddr(0x0))),
            );
            if !was_present {
                pd_ec.map(Entry::with_inc_population);
            }
        })
    }
}
