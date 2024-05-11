//! The [TermIter] trait and supporting types for abstract termiter emission of items with explicit termination

use std::{
    convert::Infallible,
    ops::{ControlFlow, Try},
};

use crate::{
    combinators::{AndThen, MapItems, MapTerminal, TerminateOnErr},
    Update::{self, Next, Terminate},
};
use either::Either;

/// A [TermIter] produces a sequence of [Item](TermIter::Item) values or a [Terminal](TermIter::Terminal)
///
/// Implementors only need to provide [TermIter::into_next].
pub trait TermIter: Sized {
    /// Each non-terminal step of a sequence produces an `Item`
    type Item;
    /// A `Terminal` is produced when a sequence terminates
    type Terminal;

    /// Consume the [TermIter] to produce either a continuation (type `Self`) with an [Item](TermIter::Item) or else a [Terminal](TermIter::Terminal)
    ///
    /// This uses move semantics (consuming the [TermIter] and potentially producing a new one) to ensure in the case of termination, no inconsistent state remains. This also ensures consuming code cannot "iterate past the end" of a sequence.
    fn into_next(self) -> Update<Self, Self::Item, Self::Terminal>;

    /// After completing `self`, continue with `downstream`, collecting the two terminals into a pair
    fn and_then<D>(self, downstream: D) -> AndThen<Self, D>
    where
        D: TermIter<Item = Self::Item>,
    {
        AndThen::new(self, downstream)
    }

    /// Process each item with `f`, then return [Self::Terminal]
    fn for_each<F>(self, mut f: F) -> Self::Terminal
    where
        F: FnMut(Self::Item),
    {
        self.for_each_ctl(|item| {
            f(item);
            ControlFlow::Continue(())
        })
        .right()
        .unwrap()
    }

    /// Process items with `f` until the sequence terminates or `f` returns [Break](ControlFlow::Break)
    ///
    /// If processing breaks, return the remaining termiter, otherwise the [Terminal](Self::Terminal)
    fn for_each_ctl<F>(self, mut f: F) -> Either<Self, Self::Terminal>
    where
        F: FnMut(Self::Item) -> ControlFlow<()>,
    {
        use either::Either::*;
        use ControlFlow::{Break, Continue};

        let mut seq = self;
        loop {
            match seq.into_next() {
                Next(next, item) => match f(item) {
                    Continue(()) => seq = next,
                    Break(()) => {
                        return Left(next);
                    }
                },
                Terminate(term) => {
                    return Right(term);
                }
            }
        }
    }

    /// Drop all items to return [Self::Terminal]
    fn terminate(self) -> Self::Terminal {
        self.for_each(std::mem::drop)
    }

    /// Map each [Self::Item] another type
    fn map_items<F, P>(self, f: F) -> MapItems<Self, F, P>
    where
        F: Fn(Self::Item) -> P,
    {
        MapItems::new(self, f)
    }

    /// Map the [Self::Terminal] another type
    fn map_terminal<F, P>(self, f: F) -> MapTerminal<Self, F, P>
    where
        F: Fn(Self::Terminal) -> P,
    {
        MapTerminal::new(self, f)
    }

    /// Transform from a [TermIter] with [Result] items to one which terminates on the first [Err] encountered, if any, otherwise it terminates with the original [Terminal](Self::Terminal)
    ///
    /// More concisely, transfrom from:
    ///
    /// `TermIter<Item = Result<X, E>, Terminal = T>` into `TermIter<Item = X, Terminal = Result<T, E>>`
    ///
    /// # Note on `Self::Item`
    ///
    /// If [type equality constraints](https://github.com/rust-lang/rust/issues/20041) were available a clearer definition of this method would be:
    ///
    /// ```ignore
    /// fn terminate_on_err<X, E>(self) -> TerminateOnErr<Self, X, E>
    /// where
    ///     Self::Item = Result<X, E>,
    /// {
    ///     ...
    /// }
    /// ```
    ///
    /// As a work-around we have the given bound on [Try] with [Result] residuals. This works as intended for `Result<X, E>`, yet it also works for other [Try impls](https://doc.rust-lang.org/std/ops/trait.Try.html#implementors).
    fn terminate_on_err<E>(self) -> TerminateOnErr<Self, E>
    where
        Self::Item: Try<Residual = Result<Infallible, E>>,
    {
        TerminateOnErr::from(self)
    }
}

impl<I> TermIter for I
where
    I: Iterator,
{
    type Item = <I as Iterator>::Item;
    type Terminal = ();

    fn into_next(mut self) -> Update<Self, Self::Item, Self::Terminal> {
        self.next().map(|x| Next(self, x)).unwrap_or(Terminate(()))
    }
}
