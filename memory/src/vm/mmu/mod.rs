use crate::{PhysAddr, VirtAddr};
use brutos_alloc::OutOfMemory;

pub mod arch;

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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MapError {
    NotAllocated,
    OutOfMemory,
    Obstructed,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum UnmapError {
    NotAllocated,
    Obstructed,
}

impl From<OutOfMemory> for MapError {
    fn from(OutOfMemory: OutOfMemory) -> MapError {
        MapError::OutOfMemory
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct Flags {
    pub user_accessible: bool,
    pub writable: bool,
    pub executable: bool,
    pub global: bool,
    pub cache_disabled: bool,
    pub writethrough: bool,
}

pub struct Tables {
    root: self::arch::EntryCell,
}

impl Tables {
    pub fn new() -> Tables {
        Tables {
            root: self::arch::EntryCell::new(),
        }
    }

    // pub fn map<Cx: self::arch::Context>(
    //     &mut self,
    //     cx: &mut Cx,
    //     virt_addr: VirtAddr,
    //     phys_addr: PhysAddr,
    //     page_size: PageSize,
    //     alloc: bool,
    //     flags: Flags,
    // ) -> Result<Option<PhysAddr>, MapError> {
    //     if alloc {
    //         self::arch::map_entry::<Cx, true>(
    //             cx,
    //             &mut self.root,
    //             page_size.level(),
    //             virt_addr,
    //             phys_addr,
    //             flags,
    //         )
    //     } else {
    //         self::arch::map_entry::<Cx, false>(
    //             cx,
    //             &mut self.root,
    //             page_size.level(),
    //             virt_addr,
    //             phys_addr,
    //             flags,
    //         )
    //     }
    // }

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
}
