use crate::TermIter;
use crate::Update::{self, Next, Terminate};
use either::Either::{self, *};

/// Compose a pair of [TermIter] values in sequence, producing all of `U`'s items and then all of `D`'s
///
/// The [TermIter::Terminal] value of `U` is held until the entire [AndThen] terminates with both constituent terminals.
pub struct AndThen<U, D>
where
    U: TermIter,
{
    upstate: Either<U, <U as TermIter>::Terminal>,
    down: D,
}

impl<U, D> AndThen<U, D>
where
    U: TermIter,
{
    pub(crate) fn new(upstream: U, downstream: D) -> Self {
        AndThen::new_inner(Left(upstream), downstream)
    }

    fn new_inner(upstate: Either<U, <U as TermIter>::Terminal>, down: D) -> Self {
        AndThen { upstate, down }
    }
}

impl<U, D, O> TermIter for AndThen<U, D>
where
    U: TermIter<Item = O>,
    D: TermIter<Item = O>,
{
    type Item = O;
    type Terminal = (<U as TermIter>::Terminal, <D as TermIter>::Terminal);

    fn into_next(self) -> Update<Self, Self::Item, Self::Terminal> {
        let AndThen { upstate, down } = self;
        match upstate {
            Left(up) => match up.into_next() {
                Next(up_new, item) => Next(
                    AndThen {
                        upstate: Left(up_new),
                        down,
                    },
                    item,
                ),
                Terminate(up_term) => AndThen {
                    upstate: Right(up_term),
                    down,
                }
                .into_next(),
            },
            Right(up_term) => match down.into_next() {
                Next(down_next, item) => Next(
                    AndThen {
                        upstate: Right(up_term),
                        down: down_next,
                    },
                    item,
                ),
                Terminate(down_term) => Terminate((up_term, down_term)),
            },
        }
    }
}
