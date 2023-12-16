//! Contains different types of words in New Ithkuil.
//!
//! ## Word Types
//!
//! 11 words types are defined.
//!
//! - [`CheckedFormative`]: any Ithkuil formative
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
//! Second, the outdated mood/case-scope adjunct is included. This is because it's a part of the New
//! Ithkuil language, even if it's no longer in common use. It's important to include both for
//! completeness and to ensure that this can be a full replacement for @ırburučpaızya, which also
//! included mood/case-scope adjuncts (having been developed for an older version of Ithkuil).
//!
//! And third, the so-called "numeric adjunct" is included. A numeric adjunct is just a number, like
//! 22 or 7824. It's included as a word type in order to represent the numerals of Ithkuil script.
//! The Ithkuil script documents detail how to write them, but not where to include them in words or
//! in daily use. This crate allows for numerals to be used as standalone words, as formative roots,
//! or as affix Cs forms, giving great flexibility that would not be possible without a standalone
//! [`NumericAdjunct`] type.
//!
//! ## Formatives
//!
//! Because Ithkuil formatives are very complicated to construct and work with, this crate provides
//! three formative types with different levels of strictness and ease of usability. They are
//! [`CheckedFormative`], [`ShortcutCheckedFormative`], and [`UncheckedFormative`].
//!
//! Here's an introduction to each:
//!
//! - [`CheckedFormative`] guarantees that values of its type are valid Ithkuil words at compile
//!   time using several nested structs and enums. However, this means it is very difficult to
//!   construct and use manually.
//!
//! - [`ShortcutCheckedFormative`] guarantees that values of its type have valid shortcuts. This
//!   means that if you have a formative with a Ca shortcut, its Ca will be a valid Ca shortcut. It
//!   accomplishes this using several nested structs and enums, although not as many as
//!   [`CheckedFormative`]. It is still difficult to construct and use manually though.
//!
//! - [`UncheckedFormative`] provides no guarantees about its data other than that its roots have
//!   the correct data. An affixual formative may specify a Cn shortcut formative that still has a
//!   Ca of `MSS.PRX.RPV`, a default mood/case-scope, and slot V affixes. However,
//!   [`UncheckedFormative`] values are always valid when constructed via an `.as_general()` call
//!   from a formative type with more guarantees. They are the easiest to work with, and use barely
//!   any nested enums or structs at all.
//!
//! To summarize, you should probably use [`UncheckedFormative`] if you're constructing formatives
//! manually. If you want to ensure the formative is correct, use [`CheckedFormative`]. For
//! everything else, use [`ShortcutCheckedFormative`].
//!
//! Again, these may change based on your specific use case, and are merely guidelines.

mod adjunct;
pub mod formative;
pub mod referential;

mod impls;
#[cfg(test)]
mod tests;
mod word;

pub use adjunct::*;
pub use formative::{extended::*, formative::*};
pub use referential::aliases::*;
pub use word::*;
