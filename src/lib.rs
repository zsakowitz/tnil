//! A crate to parse, generate, transform, and operate on text in the language of New Ithkuil.
//!
//! If you already know the name of what you are looking for, the fastest way to find it is to use
//! the <a href="#" onclick="window.searchState.focus();">search bar</a> at the top of the page.
//!
//! If you don't know what you're looking for, it's probably in the [prelude]. The prelude
//! re-exports many common things, mostly traits. Check it out.
//!
//! If you still don't know what you're looking for, here are the main things this crate does:
//!
//! - It provides several words types, all of which can be found in the [word] module.
//! - It lets you gloss words with the [`Gloss`][prelude::Gloss] trait.
//! - It lets you parse words with the [`FromTokens`][prelude::FromTokens] trait.
//! - It lets you turn words into Ithkuil with the [`IntoTokens`][prelude::IntoTokens] trait.
//! - It lets you turn words into script with the [`IntoScript`][prelude::IntoScript] trait.
//!
//! More functionality is expected in the future, mostly to do with converting into Ithkuil script.

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
pub mod script;
pub mod specificity;
pub mod word;
