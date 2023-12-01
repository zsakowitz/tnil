//! Defines a [`TokenList`] type which can be used to parse Ithkuil tokens and stress markings.

use super::{
    token::Token,
    stream::TokenStream,
    transform::{
        detect_stress, normalize, tokenize, unstress_vowels, StressError, TokenizeWordError,
    },
};
use crate::category::Stress;
use std::str::FromStr;

/// An error returned when parsing stress and tokenizing fails.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParseTokensError {
    /// An error returned on invalid or double-marked stress.
    Stress(StressError),

    /// An error returned on invalid tokenization.
    Tokenize(TokenizeWordError),
}

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
            list: self,
            start: 0,
            end: self.tokens.len(),
        }
    }
}

impl FromStr for TokenList {
    type Err = ParseTokensError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let source = normalize(s);
        let stress = detect_stress(&source).map_err(ParseTokensError::Stress)?;
        let source = unstress_vowels(&source);
        let tokens = tokenize(&source).map_err(ParseTokensError::Tokenize)?;
        Ok(TokenList { tokens, stress })
    }
}
