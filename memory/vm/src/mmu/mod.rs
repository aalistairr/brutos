pub use brutos_memory_units::MmuFlags as Flags;

use crate::{PhysAddr, VirtAddr};

pub mod arch;

pub use self::arch::{MapError, UnmapError};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PageSize {
    Normal,
    Large,
    Huge,
}

impl Default for PageSize {
    fn default() -> PageSize {
        PageSize::Normal
    }
}

pub struct Tables {
    pub root: self::arch::EntryCell,
}

impl Tables {
    pub fn new() -> Tables {
        Tables {
            root: self::arch::EntryCell::new(),
        }
    }

    pub fn get<Cx: self::arch::Context>(
        &mut self,
        cx: &mut Cx,
        virt_addr: VirtAddr,
        page_size: PageSize,
    ) -> Result<Option<self::arch::Entry>, MapError> {
        self::arch::get_entry::<Cx>(cx, &mut self.root, page_size.level(), virt_addr)
    }

    pub fn map_keep<Cx: self::arch::Context>(
        &mut self,
        cx: &mut Cx,
        virt_addr: VirtAddr,
        phys_addr: PhysAddr,
        page_size: PageSize,
        alloc: bool,
        flags: Flags,
    ) -> Result<bool, MapError> {
        if alloc {
            self::arch::map_entry_keep::<Cx, true>(
                cx,
                &mut self.root,
                page_size.level(),
                virt_addr,
                phys_addr,
                flags,
            )
        } else {
            self::arch::map_entry_keep::<Cx, false>(
                cx,
                &mut self.root,
                page_size.level(),
                virt_addr,
                phys_addr,
                flags,
            )
        }
    }

    pub fn map_replace<Cx: self::arch::Context>(
        &mut self,
        cx: &mut Cx,
        virt_addr: VirtAddr,
        phys_addr: PhysAddr,
        page_size: PageSize,
        alloc: bool,
        flags: Flags,
    ) -> Result<Option<PhysAddr>, MapError> {
        if alloc {
            self::arch::map_entry_replace::<Cx, true>(
                cx,
                &mut self.root,
                page_size.level(),
                virt_addr,
                phys_addr,
                flags,
            )
        } else {
            self::arch::map_entry_replace::<Cx, false>(
                cx,
                &mut self.root,
                page_size.level(),
                virt_addr,
                phys_addr,
                flags,
            )
        }
    }

    pub fn unmap<Cx: self::arch::Context>(
        &mut self,
        cx: &mut Cx,
        virt_addr: VirtAddr,
        page_size: PageSize,
    ) -> Result<Option<PhysAddr>, UnmapError> {
        self::arch::unmap_entry::<Cx, false>(cx, &mut self.root, page_size.level(), virt_addr)
    }

    pub fn compare_and_swap<Cx: self::arch::Context>(
        &mut self,
        cx: &mut Cx,
        virt_addr: VirtAddr,
        page_size: PageSize,
        current: self::arch::Entry,
        phys_addr: PhysAddr,
        flags: Flags,
    ) -> Result<bool, MapError> {
        self::arch::compare_and_swap(
            cx,
            &mut self.root,
            virt_addr,
            page_size.level(),
            current,
            phys_addr,
            flags,
        )
    }

    pub fn get_page<Cx: self::arch::Context>(
        &mut self,
        cx: &mut Cx,
        virt_addr: VirtAddr,
        page_size: PageSize,
    ) -> Result<Option<PhysAddr>, MapError> {
        self::arch::get_entry(cx, &mut self.root, page_size.level(), virt_addr)
            .map(|e| e.and_then(|e| if e.present() { Some(e.address()) } else { None }))
    }
}
