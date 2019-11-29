use brutos_util::uint::UInt;

pub mod arch;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysAddr(pub usize);

impl core::fmt::Debug for PhysAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl PhysAddr {
    pub const fn add(self, n: usize) -> PhysAddr {
        PhysAddr(self.0 + n)
    }

    pub const fn sub(self, n: usize) -> PhysAddr {
        PhysAddr(self.0 - n)
    }

    pub fn is_aligned(self, align: usize) -> bool {
        self.0.is_aligned(align)
    }

    pub fn align_up(self, align: usize) -> PhysAddr {
        PhysAddr(self.0.align_up(align))
    }

    pub fn align_down(self, align: usize) -> PhysAddr {
        PhysAddr(self.0.align_down(align))
    }
}
