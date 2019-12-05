use core::ops::{self, Range};

pub trait UInt:
    Copy
    + Clone
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + ops::Sub<Self, Output = Self>
    + ops::Add<Self, Output = Self>
    + ops::Not<Output = Self>
    + ops::Shl<u32, Output = Self>
    + ops::Shr<u32, Output = Self>
    + ops::BitOr<Self, Output = Self>
    + ops::BitAnd<Self, Output = Self>
    + ops::Rem<Self, Output = Self>
    + core::fmt::Debug
{
    const BIT_COUNT: u32 = (core::mem::size_of::<Self>() * 8) as u32;
    const ZERO: Self;
    const ONE: Self;

    fn leading_zeros(self) -> u32;
    fn trailing_zeros(self) -> u32;

    fn mask_range(r: Range<u32>) -> Self {
        ((Self::ONE << (r.end - r.start)) - Self::ONE) << r.start
    }

    fn mask(len: u32) -> Self {
        Self::mask_range(0..len)
    }

    fn bits(self, r: Range<u32>) -> Self {
        (self >> r.start) & Self::mask(r.end - r.start)
    }

    fn bit(self, i: u32) -> bool {
        self.bits(i..i + 1) == Self::ONE
    }

    fn with_bits(self, r: Range<u32>, x: Self) -> Self {
        assert_eq!(x & !Self::mask(r.end - r.start), Self::ZERO);
        (self & !(Self::mask(r.end - r.start) << r.start)) | (x << r.start)
    }

    fn with_bit(self, i: u32, x: bool) -> Self {
        self.with_bits(i..i + 1, if x { Self::ONE } else { Self::ZERO })
    }

    fn set_bits(&mut self, r: Range<u32>, x: Self) {
        *self = self.with_bits(r, x);
    }

    fn set_bit(&mut self, i: u32, x: bool) {
        *self = self.with_bit(i, x);
    }

    fn lsb(self) -> Option<u32> {
        if self == Self::ZERO {
            None
        } else {
            Some(self.trailing_zeros())
        }
    }

    fn msb(self) -> Option<u32> {
        if self == Self::ZERO {
            None
        } else {
            Some(Self::BIT_COUNT - 1 - self.leading_zeros())
        }
    }

    fn is_aligned(self, align: Self) -> bool {
        assert!(align > Self::ZERO);
        self % align == Self::ZERO
    }

    fn align_up(self, align: Self) -> Self {
        assert!(align > Self::ZERO);
        match self % align {
            remainder if remainder == Self::ZERO => self,
            remainder => self + (align - remainder),
        }
    }

    fn align_down(self, align: Self) -> Self {
        assert!(align > Self::ZERO);
        self - (self % align)
    }
}

macro_rules! impl_uint {
    ($($t:ty),*) => {$(
        impl UInt for $t {
            const ZERO: Self = 0;
            const ONE: Self = 1;

            fn leading_zeros(self) -> u32 {
                self.leading_zeros()
            }

            fn trailing_zeros(self) -> u32 {
                self.trailing_zeros()
            }
        }
    )*}
}
impl_uint!(u8, u16, u32, u64, usize);
