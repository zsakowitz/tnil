use crate::{
    gloss::{Gloss, GlossFlags},
    macros::invalid_tokens_error,
    romanize::{
        parse::{FromTokenStream, FromTokensError, Result},
        stream::TokenStream,
        token::NumeralForm,
    },
};
use std::convert::Infallible;

/// A numeric adjunct.
#[non_exhaustive] // TODO: Remove this once we deal with decimals.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumericAdjunct {
    /// The integer part of this adjunct.
    pub integer_part: u64,
}

impl Gloss for NumericAdjunct {
    fn gloss(&self, _flags: GlossFlags) -> String {
        let mut output = "‘".to_owned();
        output += &self.integer_part.to_string();
        output += "’";
        output
    }
}

invalid_tokens_error!(
    /// a numeric adjunct
    enum NumericTokensError {
        ExpectedNumeral = "numeric adjuncts should have a single numeral form",
        TooManyTokens = "too many tokens",
    }
);

impl FromTokenStream for NumericAdjunct {
    type TypeErr = NumericTokensError;
    type ValueErr = Infallible;

    fn from_token_stream(mut stream: TokenStream) -> Result<Self, Self::TypeErr, Self::ValueErr> {
        let numeral: NumeralForm = stream
            .next()
            .ok_or(FromTokensError::Type(NumericTokensError::ExpectedNumeral))?;

        if !stream.is_done() {
            return Err(FromTokensError::Type(NumericTokensError::TooManyTokens));
        }

        Ok(NumericAdjunct {
            integer_part: numeral.integer_part,
        })
    }
}
