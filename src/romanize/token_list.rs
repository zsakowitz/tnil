//! Defines a [`TokenList`] type which can be used to parse Ithkuil tokens and stress markings.

use super::{
    stream::{ParseError, TokenStream},
    token::Token,
    transform::{detect_stress, normalize, tokenize, unstress_vowels},
};
use crate::category::Stress;
use std::str::FromStr;

/// A tokenized word with stress marked.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenList {
    /// The tokens of this word.
    pub tokens: Vec<Token>,

    /// The stress of this word.
    pub stress: Option<Stress>,
}

impl TokenList {
    /// Streams `self` as [`TokenStream`], a structure more suitable for parsing than a plain
    /// [`TokenList`].
    pub fn stream(&self) -> TokenStream {
        TokenStream {
            tokens: &self.tokens[..],
            stress: self.stress,
            start: 0,
            end: self.tokens.len(),
        }
    }
}

impl FromStr for TokenList {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let source = normalize(s);
        let stress = detect_stress(&source)?;
        let source = unstress_vowels(&source);
        let tokens = tokenize(&source)?;
        Ok(TokenList { tokens, stress })
    }
}
