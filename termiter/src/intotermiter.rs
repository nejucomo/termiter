use crate::TermIter;

/// Types which can be converted into a [TermIter] with specific item and termination types
///
/// A blanket implementation ensures all [TermIter] types provide [IntoTermIter], analogous to [Iterator] and [IntoIterator].
pub trait IntoTermIter {
    /// The [Item](TermIter::Item) of [Into](IntoTermIter::Into)
    type Item;
    /// The [Terminal](TermIter::Terminal) of [Into](IntoTermIter::Into)
    type Terminal;
    /// The [TermIter] type `self` converts into
    type Into: TermIter<Item = Self::Item, Terminal = Self::Terminal>;

    /// Convert `self` into a [TermIter] type
    fn into_termiter(self) -> Self::Into;
}

impl<S> IntoTermIter for S
where
    S: TermIter,
{
    type Item = <S as TermIter>::Item;
    type Terminal = <S as TermIter>::Terminal;
    type Into = S;

    fn into_termiter(self) -> Self::Into {
        self
    }
}
