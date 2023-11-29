//! A crate to parse, generate, transform, and operate on text in the language of New Ithkuil.

#![forbid(
    unsafe_code,
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs
)]

pub mod affix;
mod as_general;
pub mod category;
pub mod gloss;
mod macros;
pub mod word;

pub use as_general::*;
