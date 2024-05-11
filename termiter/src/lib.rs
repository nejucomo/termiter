#![deny(unused, missing_docs)]
#![forbid(unsafe_code)]
#![feature(try_trait_v2)]
#![doc = include_str!("../README.md")]

pub mod combinators;
mod fnmut;
mod intotermiter;
mod termiter;
mod update;

pub use self::fnmut::{from_fn_mut, TermIterFnMut};
pub use self::intotermiter::IntoTermIter;
pub use self::termiter::TermIter;
pub use self::update::Update;

#[cfg(test)]
mod tests;
