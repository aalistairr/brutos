use core::ops::Range;

use brutos_memory::phys_alloc::bootstrap;
use brutos_memory::PhysAddr;

use crate::memory::{CutRange, FailedToBootstrap};
use crate::Cx;

pub const PHYS_IDENT_OFFSET: usize = 0xffff880000000000;
pub const PHYS_IDENT_SIZE: usize = 0x0000400000000000;
pub const PHYS_IDENT_END: usize = PHYS_IDENT_OFFSET + PHYS_IDENT_SIZE;

unsafe impl bootstrap::Context for Cx {
    type Err = FailedToBootstrap;

    fn map(
        &mut self,
        addr: PhysAddr,
        size: usize,
        align: usize,
    ) -> Result<*mut u8, FailedToBootstrap> {
        debug_assert!(addr.is_aligned(align));
        if addr.0.checked_add(size).ok_or(FailedToBootstrap)? > PHYS_IDENT_SIZE {
            return Err(FailedToBootstrap);
        }
        return Ok((addr.0 + PHYS_IDENT_OFFSET) as *mut u8);
    }
}

#[allow(improper_ctypes)]
extern "C" {
    static _image_start: ();
    static _image_end: ();
}

fn image_range() -> Range<PhysAddr> {
    unsafe {
        PhysAddr(&_image_start as *const _ as usize)..PhysAddr(&_image_end as *const _ as usize)
    }
}

pub fn remove_reserved_memory(
    multiboot_info_range: Range<PhysAddr>,
    mmap: impl Clone + Iterator<Item = Range<PhysAddr>>,
) -> impl Clone + Iterator<Item = Range<PhysAddr>> {
    let mmap = CutRange::new(mmap, PhysAddr(0x0)..PhysAddr(0x500));
    let mmap = CutRange::new(mmap, PhysAddr(0x7c00)..PhysAddr(0x7e00));
    let mmap = CutRange::new(mmap, PhysAddr(0x9fc00)..PhysAddr(0x100000));
    let mmap = CutRange::new(mmap, PhysAddr(0xf00000)..PhysAddr(0x1000000));
    let mmap = CutRange::new(mmap, PhysAddr(0xc0000000)..PhysAddr(0x100000000));
    let mmap = CutRange::new(mmap, multiboot_info_range);
    let mmap = CutRange::new(mmap, image_range());
    let mmap = mmap.filter(|r| r.start.0 < PHYS_IDENT_SIZE).map(|r| {
        if r.end.0 <= PHYS_IDENT_SIZE {
            r
        } else {
            r.start..PhysAddr(PHYS_IDENT_SIZE)
        }
    });
    mmap
}
