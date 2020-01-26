use core::marker::PhantomData;

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
