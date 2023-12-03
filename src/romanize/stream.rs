//! Contains types and traits used to parse Ithkuil from token streams.

use super::{token::Token, token_list::TokenList};
use crate::category::Stress;
use std::{error::Error, fmt};

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
        let token = T::parse(&token)?;
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

    /// Parses an item.
    pub fn parse<T: FromTokenStream>(&mut self) -> Result<T, ParseError> {
        T::parse(self)
    }

    /// Returns a slice into the remaining tokens.
    pub fn tokens_left(self) -> &'a [Token] {
        &self.list.tokens[self.start..self.end]
    }
}

/// Allows types to be created from a sequence of tokens.
pub trait FromTokenStream: Sized {
    /// Creates this item from a [`TokenStream`], returning [`Err`] if it fails.
    ///
    /// The [`TokenStream`] will advance correctly if an [`Ok`] is returned, but will advance an
    /// unspecified number of tokens if an [`Err`] is returned. If an [`Err`] is detected, it is
    /// best to reset the underlying [`TokenStream`]'s `start` and `end` indices.
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError>;

    /// Creates this item from a [`TokenStream`], returning [`Err`] if it fails.
    ///
    /// The [`TokenStream`] will advance correctly if an [`Ok`] is returned and will advance no
    /// tokens if an [`Err`] is returned.
    fn parse(stream: &mut TokenStream) -> Result<Self, ParseError> {
        let start = stream.start;
        let end = stream.end;

        match Self::parse_volatile(stream) {
            Ok(value) => Ok(value),
            Err(error) => {
                stream.start = start;
                stream.end = end;
                Err(error)
            }
        }
    }
}

/// Allows token types to be extracted from a single token.
pub trait TokenType: Sized {
    /// Creates this item from a [`TokenStream`], returning [`None`] if it fails.
    fn parse(token: &Token) -> Option<Self>;
}

macro_rules! parse_error_defn {
    (match self { $($variant:ident => $message:literal,)+ }) => {
        ::paste::paste! {
            /// The error type returned when an item cannot be parsed.
            #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
            pub enum ParseError {
                $(
                    #[doc = "The error \"" $message "\"."]
                    $variant
                ),+
            }
        }

        impl fmt::Display for ParseError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(match self {
                    $(Self::$variant => $message,)+
                })
            }
        }

        impl Error for ParseError {}
    };
}

parse_error_defn!(match self {
    ExpectedCa => "expected an ungeminated Ca form (e.g. r, lňn, řţgw)",
    ExpectedCaGeminated => "expected a geminated Ca form (e.g. rr, lňňn, řţţgw)",
    ExpectedCb => "expected a Cb bias (e.g. pļļ, lçp, kšš)",
    ExpectedCn => "expected a Cn mood/case-scope (e.g. h, hm, hňw)",
    ExpectedCm => "expected a Cm form (n/ň)",
    ExpectedCp => "expected a Cp suppletive adjunct mode (hl/hm/hn/hň)",
    ExpectedCs => "expected a Cs affix form (e.g. t, kb, ltř)",
    ExpectedCy => "expected a Cy mood/case-scope adjunct vowel (e.g. a, oi, iu)",
    ExpectedGs => "expected a word-final glottal stop",
    ExpectedHh => "expected a single ‘h’ at the beginning of a register",
    ExpectedHr => "expected ‘hr’ at the beginning of a mood/case-scope adjunct",
    ExpectedNn => "expected a Nn numeric form (e.g. 4, 23, 7832)",
    ExpectedVc => "expected a Vc case form (e.g. ü, ai, io)",
    ExpectedVh => "expected a Vh modular adjunct scope form (a/e/i/o/u)",
    ExpectedVm => "expected a Vm register type (e.g. a, o, ei)",
    ExpectedVn => "expected a Vn form (e.g. a, ou, ie)",
    ExpectedVp => "expected a Vp parsing adjunct type (a/e/o/u)",
    ExpectedVx => "expected a Vx affix degree (e.g. a, ou, ie)",
    AntepenultimateStress => "antepenultimate stress cannot appear except in formatives",
    GlottalizedVn => "Vn forms cannot have glottal stops except in formatives",
    ReferentEmpty => "expected at least one referent",
    ReferentInvalid => "invalid referent list",
});
