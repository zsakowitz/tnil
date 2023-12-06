//! A prelude containing common types and traits used in this crate.
//!
//! This _will_ be expanded over time, mostly to add traits.

pub use crate::{
    category::Category,
    gloss::{Gloss, GlossFlags, GlossHelpers, GlossStatic},
    romanize::{
        flags::FromTokenFlags,
        stream::ParseError,
        token,
        token_list::TokenList,
        traits::{FromTokens, TokenType},
    },
    specificity::{AsGeneral, AsSpecific, TryAsGeneral, TryAsSpecific},
    word::{self, Word},
};
