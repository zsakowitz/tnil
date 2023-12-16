//! Defines a [`TokenList`] type which can be used to parse Ithkuil tokens and stress markings.

use super::{
    flags::IntoTokensFlags,
    stream::{ParseError, TokenStream},
    token::Token,
    traits::{IntoToken, IntoTokens},
    transform::{
        add_stress, detect_stress, normalize, tokenize, tokens_to_string, unstress_vowels,
    },
};
use crate::category::Stress;
use std::str::FromStr;

/// A tokenized word with stress marked.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TokenList {
    /// The tokens of this word.
    pub tokens: Vec<Token>,

    /// The stress of this word.
    pub stress: Option<Stress>,
}

impl TokenList {
    /// Creates an empty [`TokenList`].
    pub const fn new() -> Self {
        TokenList {
            tokens: Vec::new(),
            stress: None,
        }
    }

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

    /// Pushes a token into `self`.
    pub fn push<T: IntoToken>(&mut self, token: T) {
        self.tokens.push(token.into_token());
    }

    /// Appends an item as tokens to `self`.
    pub fn append<T: IntoTokens>(&mut self, item: &T, flags: IntoTokensFlags) {
        item.append_tokens_to(self, flags);
    }

    /// Modifies the stress of `self`.
    pub fn set_stress(&mut self, stress: Stress) {
        self.stress = Some(stress);
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

impl ToString for TokenList {
    fn to_string(&self) -> String {
        let word = tokens_to_string(&self.tokens);
        if let Some(stress) = self.stress {
            return add_stress(&word, stress).unwrap_or(word);
        }
        return word;
    }
}
