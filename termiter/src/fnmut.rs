use crate::TermIter;
use crate::Update::{self, Next, Terminate};
use either::Either;

/// Wraps a [FnMut] which returns [TermIter] items or a terminal
pub struct TermIterFnMut<F>(F);

/// Wrap a [FnMut] which returns [TermIter] items or a terminal
pub fn from_fn_mut<F, O, T>(f: F) -> TermIterFnMut<F>
where
    F: FnMut() -> Either<O, T>,
{
    TermIterFnMut(f)
}

impl<F, O, T> TermIter for TermIterFnMut<F>
where
    F: FnMut() -> Either<O, T>,
{
    type Item = O;
    type Terminal = T;

    fn into_next(mut self) -> Update<Self, Self::Item, Self::Terminal> {
        self.0()
            .map_left(|item| Next(self, item))
            .map_right(Terminate)
            .into_inner()
    }
}
