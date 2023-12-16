//! Contains types related to affixual adjuncts.

mod multiple;
mod single;

pub use multiple::MultipleAffixAdjunct;
pub use single::SingleAffixAdjunct;

use crate::{
    gloss::{Gloss, GlossFlags},
    prelude::{IntoTokens, TokenList},
    romanize::{
        flags::{FromTokenFlags, IntoTokensFlags},
        stream::{ParseError, TokenStream},
        token::Token,
        traits::FromTokens,
    },
};

/// An affixual adjunct.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AffixualAdjunct {
    /// A variant containing a single affix.
    Single(SingleAffixAdjunct),

    /// A variant containing multiple affixes.
    Multiple(MultipleAffixAdjunct),
}

impl Gloss for AffixualAdjunct {
    fn gloss(&self, flags: GlossFlags) -> String {
        match self {
            AffixualAdjunct::Single(value) => value.gloss(flags),
            AffixualAdjunct::Multiple(value) => value.gloss(flags),
        }
    }
}

impl FromTokens for AffixualAdjunct {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        match stream.peek() {
            Some(Token::V(_)) => Ok(AffixualAdjunct::Single(stream.parse(flags)?)),
            Some(Token::Schwa | Token::C(_)) => Ok(AffixualAdjunct::Multiple(stream.parse(flags)?)),
            _ => Err(ParseError::ExpectedCsOrVx),
        }
    }
}

impl IntoTokens for AffixualAdjunct {
    fn append_tokens_to(&self, list: &mut TokenList, flags: IntoTokensFlags) {
        match self {
            Self::Single(value) => value.append_tokens_to(list, flags),
            Self::Multiple(value) => value.append_tokens_to(list, flags),
        }
    }
}
