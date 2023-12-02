use crate::{
    gloss::{Gloss, GlossFlags},
    macros::invalid_type_error,
    romanize::{
        parse::{FromTokenStream, Result},
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

invalid_type_error!(
    /// a numeric adjunct
    enum NumericTokenError {
        ExpectedNn,
        TooManyTokens,
    }
);

impl FromTokenStream for NumericAdjunct {
    type TypeErr = NumericTokenError;
    type ValueErr = Infallible;

    fn from_token_stream(mut stream: TokenStream) -> Result<Self, Self::TypeErr, Self::ValueErr> {
        // Numeric adjunct: Nn
        let nn: NumeralForm = stream.next_or_err(NumericTokenError::ExpectedNn)?;
        stream.done_or_err(NumericTokenError::TooManyTokens)?;
        Ok(NumericAdjunct {
            integer_part: nn.integer_part,
        })
    }
}
