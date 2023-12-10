//! A prelude containing common types and traits used in this crate.

pub use crate::{
    category::Category,
    gloss::{Gloss, GlossFlags, GlossHelpers, GlossStatic},
    romanize::{
        flags::{FromTokenFlags, IntoTokensFlags},
        stream::ParseError,
        token,
        token_list::TokenList,
        traits::{FromToken, FromTokens, IntoToken, IntoTokens},
    },
    script::{
        character::{self, Character},
        traits::IntoScript,
    },
    specificity::{AsGeneral, AsSpecific, TryAsGeneral, TryAsSpecific},
    word::{self, Word},
};
