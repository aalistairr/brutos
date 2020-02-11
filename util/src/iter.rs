use core::marker::PhantomData;

pub struct Unfold_<T, U, F> {
    seed: T,
    f: F,
    _marker: PhantomData<U>,
}

impl<T, U, F> Iterator for Unfold_<T, U, F>
where
    F: FnMut(&mut T) -> Option<U>,
{
    type Item = U;

    fn next(&mut self) -> Option<U> {
        (self.f)(&mut self.seed)
    }
}

pub fn unfold<T, U, F>(seed: T, f: F) -> Unfold_<T, U, F>
where
    F: FnMut(&mut T) -> Option<U>,
{
    Unfold_ {
        seed,
        f,
        _marker: PhantomData,
    }
}

pub trait UnfoldValueRet<T, U> {
    fn unpack(self) -> (Option<T>, Option<U>);
}

impl<T, U> UnfoldValueRet<T, U> for (Option<T>, U) {
    fn unpack(self) -> (Option<T>, Option<U>) {
        match self {
            (Some(t), u) => (Some(t), Some(u)),
            (None, u) => (None, Some(u)),
        }
    }
}

impl<T, U> UnfoldValueRet<T, U> for Option<(T, U)> {
    fn unpack(self) -> (Option<T>, Option<U>) {
        match self {
            Some((t, u)) => (Some(t), Some(u)),
            None => (None, None),
        }
    }
}

pub fn unfold_value_opt<T, U, F, R>(
    seed: Option<T>,
    mut f: F,
) -> Unfold_<Option<T>, U, impl FnMut(&mut Option<T>) -> Option<U>>
where
    F: FnMut(T) -> R,
    R: UnfoldValueRet<T, U>,
{
    let f = move |seed: &mut Option<T>| {
        seed.take().and_then(|s_0| {
            let (s_1, u) = f(s_0).unpack();
            *seed = s_1;
            u
        })
    };
    Unfold_ {
        seed,
        f,
        _marker: PhantomData,
    }
}

pub fn unfold_value<T, U, F, R>(
    seed: T,
    f: F,
) -> Unfold_<Option<T>, U, impl FnMut(&mut Option<T>) -> Option<U>>
where
    F: FnMut(T) -> R,
    R: UnfoldValueRet<T, U>,
{
    unfold_value_opt(Some(seed), f)
}
