//! Contains types related to affixual adjuncts.

mod multiple;
mod single;

pub use multiple::*;
pub use single::*;

use self::{multiple::MultipleAffixAdjunct, single::SingleAffixAdjunct};
use crate::{
    gloss::{Gloss, GlossFlags},
    romanize::{
        flags::FromTokenFlags,
        stream::{FromTokenStream, ParseError, TokenStream},
        token::Token,
    },
};

/// An affixual adjunct.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

impl FromTokenStream for AffixualAdjunct {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        match stream.peek() {
            Some(Token::Vowel(_)) => Ok(AffixualAdjunct::Single(stream.parse(flags)?)),
            Some(Token::Schwa(_) | Token::Consonant(_)) => {
                Ok(AffixualAdjunct::Multiple(stream.parse(flags)?))
            }
            _ => Err(ParseError::ExpectedCsOrVx),
        }
    }
}
