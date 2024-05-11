use crate::{TermIter, Update};

/// Map the [Terminal](TermIter::Terminal) of a [TermIter]
pub struct MapTerminal<S, F, U>
where
    S: TermIter,
    F: Fn(S::Terminal) -> U,
{
    seq: S,
    f: F,
}

impl<S, F, U> MapTerminal<S, F, U>
where
    S: TermIter,
    F: Fn(S::Terminal) -> U,
{
    pub(crate) fn new(seq: S, f: F) -> Self {
        MapTerminal { seq, f }
    }
}

impl<S, F, U> TermIter for MapTerminal<S, F, U>
where
    S: TermIter,
    F: Fn(S::Terminal) -> U,
{
    type Item = S::Item;
    type Terminal = U;

    fn into_next(self) -> Update<Self, Self::Item, Self::Terminal> {
        let MapTerminal { seq, f } = self;
        seq.into_next()
            .map_terminal(&f)
            .map_state(|next| MapTerminal::new(next, f))
    }
}
