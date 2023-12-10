//! A crate to parse, generate, transform, and operate on text in the language of New Ithkuil.
//!
//! If you're looking for something, it's probably in the [prelude].

#![forbid(
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs
)]
#![deny(unsafe_code)]

pub mod affix;
pub mod category;
pub mod gloss;
mod macros;
pub mod prelude;
pub mod romanize;
pub mod specificity;
pub mod word;
