//! Contains types and traits related to parsing romanized Ithkuil text.

use super::stream::TokenStream;

/// The kind of error returned from [`FromTokenStream`].
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum FromTokensError<T, V> {
    /// An variant returned when this word cannot be constructed from a set of tokens because the
    /// token types are invalid (e.g. a VCV stream is passed to [`BiasAdjunct`][crate::word::BiasAdjunct]).
    ///
    /// This type of error can be recovered from.
    Type(T),

    /// An variant returned when this word cannot be constructed from a set of tokens because the
    /// token values are invalid (e.g. C("llkp") is passed to [`BiasAdjunct`][crate::word::BiasAdjunct]).
    ///
    /// This type of error cannot be recovered from.
    Value(V),
}

/// A result returned from [`FromTokenStream::from_token_stream`].
pub type Result<T, ET, EV> = std::result::Result<T, FromTokensError<ET, EV>>;

/// Allows types to be created from a sequence of tokens.
pub trait FromTokenStream: Sized {
    /// An error returned when this word cannot be constructed from a set of tokens because the
    /// token types are invalid (e.g. a VCV stream is passed to [`BiasAdjunct`][crate::word::BiasAdjunct]).
    ///
    /// This type of error can be recovered from by parsing the token stream as a different word
    /// type.
    type TypeErr;

    /// An error returned when this word cannot be constructed from a set of tokens because the
    /// token values are invalid (e.g. C("llkp") is passed to [`BiasAdjunct`][crate::word::BiasAdjunct]).
    ///
    /// This type of error cannot be recovered from.
    type ValueErr;

    /// Creates this word from a [`TokenStream`], returning an error if it fails.
    ///
    /// The [`TokenStream`] should be exhausted if the function returns an [`Ok`] variant.
    fn from_token_stream(stream: TokenStream) -> Result<Self, Self::TypeErr, Self::ValueErr>;
}
