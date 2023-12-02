//! A borrowed form of [`TokenList`] used to aid in constructing parsers.

use super::{
    parse::FromTokensError,
    token::{FromToken, Token},
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

    /// Returns an [`Ok`] if this stream is done, and an [`Err`] otherwise with the error wrapped
    /// in [`FromTokensError::Type`].
    pub fn done_or_err<ET, EV>(&self, error: ET) -> Result<(), FromTokensError<ET, EV>> {
        if self.is_done() {
            Ok(())
        } else {
            Err(FromTokensError::Type(error))
        }
    }

    /// Gets the stress of the corresponding [`TokenList`].
    pub const fn stress(&self) -> Option<Stress> {
        self.list.stress
    }

    /// Returns the next token as a specialized token type.
    pub fn next<T: FromToken>(&mut self) -> Option<T> {
        if self.is_done() {
            return None;
        }
        let token = self.list.tokens.get(self.start)?;
        let token = T::try_from_token(&token)?;
        self.start += 1;
        Some(token)
    }

    /// Returns an [`Ok`] containing the next token if it parses successfully, and an [`Err`]
    /// otherwise with the error wrapped in [`FromTokensError::Type`].
    pub fn next_or_err<T: FromToken, ET, EV>(
        &mut self,
        error: ET,
    ) -> Result<T, FromTokensError<ET, EV>> {
        self.next().ok_or(FromTokensError::Type(error))
    }

    /// Returns a slice into the remaining tokens.
    pub fn tokens_left(self) -> &'a [Token] {
        &self.list.tokens[self.start..self.end]
    }
}
