//! Provides the [`FromTokens`], [`IntoTokens`], and [`TokenType`] traits.

use super::{
    flags::FromTokenFlags,
    stream::{ParseError, TokenStream},
    token::Token,
    token_list::TokenList,
};
use std::str::FromStr;

/// Allows types to be extracted from a single token.
pub trait FromToken: Sized {
    /// Creates this item from a [`Token`], returning [`None`] if it is not a valid token of this
    /// type.
    fn from_token(token: &Token) -> Option<Self>;
}

/// Allows types to be converted into a single token.
pub trait IntoToken: Sized {
    /// Turns `self` into a [`Token`].
    fn into_token(self) -> Token;
}

/// Allows types to be created from a sequence of tokens.
pub trait FromTokens: Sized {
    /// Creates this item from a [`TokenStream`], returning [`Err`] if it fails.
    ///
    /// The [`TokenStream`] will advance correctly if an [`Ok`] is returned, but will advance an
    /// unspecified number of tokens if an [`Err`] is returned. If an [`Err`] is detected, it is
    /// best to reset the underlying [`TokenStream`]'s `start` and `end` indices.
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError>;

    /// Creates this item from a [`TokenStream`], returning [`Err`] if it fails.
    ///
    /// The [`TokenStream`] will advance correctly if an [`Ok`] is returned and will advance no
    /// tokens if an [`Err`] is returned.
    fn parse(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        let start = stream.start;
        let end = stream.end;

        match Self::parse_volatile(stream, flags) {
            Ok(value) => Ok(value),
            Err(error) => {
                stream.start = start;
                stream.end = end;
                Err(error)
            }
        }
    }

    /// Parses this item from a string.
    fn parse_str(source: &str, flags: FromTokenFlags) -> Result<Self, ParseError> {
        let list = TokenList::from_str(source)?;
        let mut stream = list.stream();
        let result = Self::parse_volatile(&mut stream, flags)?;
        if stream.is_done() {
            Ok(result)
        } else {
            Err(ParseError::TooManyTokens)
        }
    }
}

/// Allows types to be turned into a sequence of tokens.
pub trait IntoTokens {
    /// Appends `self` as a list of tokens into the passed [`TokenList`].
    fn append_to(&self, list: &mut TokenList);
}

impl<T> IntoTokens for T
where
    T: Clone + IntoToken,
{
    fn append_to(&self, list: &mut TokenList) {
        list.push(self.clone().into_token());
    }
}
