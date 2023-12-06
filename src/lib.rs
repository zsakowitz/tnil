#![doc = include_str!("../README.md")]
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
