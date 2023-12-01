//! Contains different types of words in New Ithkuil.
//!
//! Eleven core word types are defined:
//!
//! - [`Formative`]: an Ithkuil formative, whether its root is normal/numeric/referential/affixual
//! - [`NormalReferential`]: a referential whose first referent is a normal referent
//! - [`SuppletiveReferential`]: a referential whose first referent is a suppletive adjunct
//! - [`AffixualAdjunct`]: an affixual adjunct
//! - [`BiasAdjunct`]: a bias adjunct
//! - [`MCSAdjunct`]: a mood/case-scope adjunct (outdated, but included anyway for completeness)
//! - [`ModularAdjunct`]: a modular adjunct
//! - [`NumericAdjunct`]: a numeric adjunct (nonstandard, but included anyway by convention)
//! - [`ParsingAdjunct`]: a parsing adjunct
//! - [`RegisterAdjunct`]: a register adjunct
//! - [`SuppletiveAdjunct`]: a suppletive adjunct
//!
//! Three odd, but useful choices are made here.
//!
//! First, referentials are split in two based on whether they are normal referentials or have a
//! suppletive adjunct head. For a generalized version that can match either, see [`GeneralReferential`].
//!
//! Second, the outdated mood/case-scope adjunct is included. This is because it's a part of the
//! New Ithkuil language, even if it's no longer in common use. It's important to include both for
//! completeness and to ensure that this can be a full replacement for @ırburučpaızya, which also
//! included mood/case-scope adjuncts (having been developed for an older version of Ithkuil).
//!
//! And third, the so-called "numeric adjunct" is included. A numeric adjunct is just a number, like
//! 22 or 7824. It's included as a word type in order to represent the numerals of Ithkuil script.
//! The Ithkuil script documents detail how to write them, but not where to include them in words or
//! in daily use. This crate allows for numerals to be used as standalone words, as formative roots,
//! or as affix Cs forms, giving great flexibility that would not be possible without a standalone
//! [`NumericAdjunct`] type.

mod adjunct;
pub mod formative;
pub mod referential;

#[cfg(test)]
mod tests;

pub use adjunct::*;
pub use formative::formative::{Formative, GeneralFormative};
pub use referential::aliases::*;
