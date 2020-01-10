use core::marker::PhantomData;
use core::mem;
use core::ops::{Range, RangeInclusive};

pub fn unfold_opt<T, U, F, R>(seed: Option<T>, f: F) -> Unfold<T, U, F, R>
where
    F: FnMut(T) -> R,
    R: UnfoldRet<T, U>,
{
    Unfold {
        seed,
        f,
        _marker: PhantomData,
    }
}

pub fn unfold<T, U, F, R>(seed: T, f: F) -> Unfold<T, U, F, R>
where
    F: FnMut(T) -> R,
    R: UnfoldRet<T, U>,
{
    unfold_opt(Some(seed), f)
}

pub trait UnfoldRet<T, U> {
    fn unpack(self) -> (Option<T>, Option<U>);
}

impl<T, U> UnfoldRet<T, U> for (Option<T>, U) {
    fn unpack(self) -> (Option<T>, Option<U>) {
        match self {
            (Some(t), u) => (Some(t), Some(u)),
            (None, u) => (None, Some(u)),
        }
    }
}

impl<T, U> UnfoldRet<T, U> for Option<(T, U)> {
    fn unpack(self) -> (Option<T>, Option<U>) {
        match self {
            Some((t, u)) => (Some(t), Some(u)),
            None => (None, None),
        }
    }
}

pub struct Unfold<T, U, F, R>
where
    F: FnMut(T) -> R,
    R: UnfoldRet<T, U>,
{
    seed: Option<T>,
    f: F,
    _marker: PhantomData<U>,
}

impl<T, U, F, R> Iterator for Unfold<T, U, F, R>
where
    F: FnMut(T) -> R,
    R: UnfoldRet<T, U>,
{
    type Item = U;

    fn next(&mut self) -> Option<U> {
        self.seed.take().and_then(|x_0| {
            let (x_1, y) = (self.f)(x_0).unpack();
            self.seed = x_1;
            y
        })
    }
}

pub trait RangeIterable: PartialOrd + Clone {
    fn up(&self) -> Self;
    fn down(&self) -> Self;
}

pub struct RangeIter<T> {
    range: Range<T>,
}

impl<T: RangeIterable> Iterator for RangeIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.range.start < self.range.end {
            let next_start = self.range.start.up();
            Some(mem::replace(&mut self.range.start, next_start))
        } else {
            None
        }
    }
}

impl<T: RangeIterable> DoubleEndedIterator for RangeIter<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.range.start < self.range.end {
            self.range.end = self.range.end.down();
            Some(self.range.end.clone())
        } else {
            None
        }
    }
}

pub struct RangeInclusiveIter<T> {
    range: Option<RangeInclusive<T>>,
}

impl<T: RangeIterable> Iterator for RangeInclusiveIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.range
            .take()
            .map(RangeInclusive::into_inner)
            .and_then(|(start, end)| {
                if start < end {
                    self.range = Some(start.up()..=end);
                    Some(start)
                } else if start == end {
                    Some(start)
                } else {
                    None
                }
            })
    }
}

impl<T: RangeIterable> DoubleEndedIterator for RangeInclusiveIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.range
            .take()
            .map(RangeInclusive::into_inner)
            .and_then(|(start, end)| {
                if start < end {
                    self.range = Some(start..=end.down());
                    Some(end)
                } else if start == end {
                    Some(end)
                } else {
                    None
                }
            })
    }
}

pub trait RangeExt<T> {
    type Iter: Iterator<Item = T>;

    fn iter(self) -> Self::Iter;
}

impl<T: RangeIterable> RangeExt<T> for Range<T> {
    type Iter = RangeIter<T>;

    fn iter(self) -> RangeIter<T> {
        RangeIter { range: self }
    }
}

impl<T: RangeIterable> RangeExt<T> for RangeInclusive<T> {
    type Iter = RangeInclusiveIter<T>;

    fn iter(self) -> RangeInclusiveIter<T> {
        RangeInclusiveIter { range: Some(self) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    enum Foo {
        A,
        B,
        C,
        D,
        E,
    }

    impl RangeIterable for Foo {
        fn up(&self) -> Foo {
            use self::Foo::*;
            match self {
                A => B,
                B => C,
                C => D,
                D => E,
                E => panic!(),
            }
        }

        fn down(&self) -> Foo {
            use self::Foo::*;
            match self {
                A => panic!(),
                B => A,
                C => B,
                D => C,
                E => D,
            }
        }
    }

    #[test]
    fn range_iter() {
        use self::Foo::*;
        use std::iter::FromIterator;
        assert_eq!(Vec::from_iter((A..E).iter()), vec![A, B, C, D]);
        assert_eq!(Vec::from_iter((A..E).iter().rev()), vec![D, C, B, A]);
        assert_eq!(Vec::from_iter((A..=E).iter()), vec![A, B, C, D, E]);
        assert_eq!(Vec::from_iter((A..=E).iter().rev()), vec![E, D, C, B, A]);
    }
}
