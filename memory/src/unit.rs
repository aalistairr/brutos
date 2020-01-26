use core::fmt;
use core::ops::{Add, Sub};

use brutos_util::UInt;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysAddr(pub usize);

impl fmt::Debug for PhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl Add<usize> for PhysAddr {
    type Output = PhysAddr;

    fn add(self, other: usize) -> PhysAddr {
        PhysAddr(self.0 + other)
    }
}

impl Sub<usize> for PhysAddr {
    type Output = PhysAddr;

    fn sub(self, other: usize) -> PhysAddr {
        PhysAddr(self.0 - other)
    }
}

impl Sub<PhysAddr> for PhysAddr {
    type Output = usize;

    fn sub(self, other: PhysAddr) -> usize {
        self.0 - other.0
    }
}

impl PhysAddr {
    pub fn checked_add(self, other: usize) -> Option<PhysAddr> {
        self.0.checked_add(other).map(PhysAddr)
    }

    pub fn is_aligned(self, align: usize) -> bool {
        self.0.is_aligned(align)
    }

    pub fn align_up(self, align: usize) -> PhysAddr {
        PhysAddr(self.0.align_up(align))
    }

    pub fn checked_align_up(self, align: usize) -> Option<PhysAddr> {
        self.0.checked_align_up(align).map(PhysAddr)
    }

    pub fn align_down(self, align: usize) -> PhysAddr {
        PhysAddr(self.0.align_down(align))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VirtAddr(pub usize);

impl fmt::Debug for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl Add<usize> for VirtAddr {
    type Output = VirtAddr;

    fn add(self, other: usize) -> VirtAddr {
        VirtAddr(self.0 + other)
    }
}

impl Sub<usize> for VirtAddr {
    type Output = VirtAddr;

    fn sub(self, other: usize) -> VirtAddr {
        VirtAddr(self.0 - other)
    }
}

impl Sub<VirtAddr> for VirtAddr {
    type Output = usize;

    fn sub(self, other: VirtAddr) -> usize {
        self.0 - other.0
    }
}

impl VirtAddr {
    pub fn checked_add(self, other: usize) -> Option<VirtAddr> {
        self.0.checked_add(other).map(VirtAddr)
    }

    pub fn is_aligned(self, align: usize) -> bool {
        self.0.is_aligned(align)
    }

    pub fn align_up(self, align: usize) -> VirtAddr {
        VirtAddr(self.0.align_up(align))
    }

    pub fn align_down(self, align: usize) -> VirtAddr {
        VirtAddr(self.0.align_down(align))
    }

    pub fn checked_align_up(self, align: usize) -> Option<VirtAddr> {
        self.0.checked_align_up(align).map(VirtAddr)
    }
}
