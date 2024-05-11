use crate::{TermIter, Update};

/// Map each [Item](TermIter::Item) of a [TermIter]
pub struct MapItems<S, F, P>
where
    S: TermIter,
    F: Fn(S::Item) -> P,
{
    seq: S,
    f: F,
}

impl<S, F, P> MapItems<S, F, P>
where
    S: TermIter,
    F: Fn(S::Item) -> P,
{
    pub(crate) fn new(seq: S, f: F) -> Self {
        MapItems { seq, f }
    }
}

impl<S, F, P> TermIter for MapItems<S, F, P>
where
    S: TermIter,
    F: Fn(S::Item) -> P,
{
    type Item = P;
    type Terminal = S::Terminal;

    fn into_next(self) -> Update<Self, Self::Item, Self::Terminal> {
        let MapItems { seq, f } = self;
        seq.into_next()
            .map_item(&f)
            .map_state(|next| MapItems::new(next, f))
    }
}
