//! Combinator types for composing chained transformations of [TermIter](crate::TermIter)

mod andthen;
mod mapitems;
mod mapterminal;
mod termonerr;

pub use self::andthen::AndThen;
pub use self::mapitems::MapItems;
pub use self::mapterminal::MapTerminal;
pub use self::termonerr::TerminateOnErr;
