//! A borrowed form of [`TokenList`] used to aid in constructing parsers.

use super::{
    token::{Token, TokenType},
    token_list::TokenList,
};
use crate::category::Stress;

/// A borrowed [`TokenList`] used to facilitate constructing parsers.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct TokenStream<'a> {
    /// The referenced [`TokenList`].
    pub(super) list: &'a TokenList,

    /// The cursor index into the list from the front.
    pub(super) start: usize,

    /// The cursor index into the list from the back.
    pub(super) end: usize,
}

impl<'a> TokenStream<'a> {
    /// Checks if this [`TokenStream`] has no tokens left to parse.
    pub const fn is_done(&self) -> bool {
        self.start >= self.end
    }

    /// Gets the stress of the corresponding [`TokenList`].
    pub const fn stress(&self) -> Option<Stress> {
        self.list.stress
    }

    /// Returns the next token as a specialized token type.
    pub fn next<T: TokenType>(&mut self) -> Option<T> {
        if self.is_done() {
            return None;
        }
        let token = self.list.tokens.get(self.start)?;
        let token = T::try_from_token(&token)?;
        self.start += 1;
        Some(token)
    }

    /// Returns the next token as a specialized token type.
    pub fn next_any(&mut self) -> Option<&Token> {
        if self.is_done() {
            return None;
        }
        let token = self.list.tokens.get(self.start)?;
        self.start += 1;
        Some(token)
    }

    /// Returns the next token from the back as a specialized token type.
    pub fn next_back<T: TokenType>(&mut self) -> Option<T> {
        if self.is_done() {
            return None;
        }
        let token = self.list.tokens.get(self.end)?;
        let token = T::try_from_token(&token)?;
        self.end -= 1;
        Some(token)
    }

    /// Returns the next token from the back as a specialized token type.
    pub fn next_any_back(&mut self) -> Option<&Token> {
        if self.is_done() {
            return None;
        }
        let token = self.list.tokens.get(self.end)?;
        self.end -= 1;
        Some(token)
    }

    /// Returns a pair of the next two tokens as specialized token types.
    pub fn next_pair<A: TokenType, B: TokenType>(&mut self) -> Option<(A, B)> {
        if self.is_done() {
            return None;
        }
        let a = self.list.tokens.get(self.start)?;
        let b = self.list.tokens.get(self.start + 1)?;
        let a = A::try_from_token(&a)?;
        let b = B::try_from_token(&b)?;
        self.start += 2;
        Some((a, b))
    }

    /// Returns a pair of the next two tokens as specialized token types.
    ///
    /// Returns `(second_to_last_token, last_token)`.
    pub fn next_pair_back<A: TokenType, B: TokenType>(&mut self) -> Option<(A, B)> {
        if self.is_done() {
            return None;
        }
        if self.end == 0 {
            return None;
        }
        let b = self.list.tokens.get(self.end)?;
        let a = self.list.tokens.get(self.end - 1)?;
        let b = B::try_from_token(&b)?;
        let a = A::try_from_token(&a)?;
        self.start -= 2;
        Some((a, b))
    }

    /// Returns a slice into the remaining tokens.
    pub fn tokens_left(self) -> &'a [Token] {
        &self.list.tokens[self.start..self.end]
    }
}
