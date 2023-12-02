//! Contains types related to parsing adjuncts.

use crate::{
    category::Stress,
    gloss::{GlossFlags, GlossStatic},
    macros::invalid_type_error,
    romanize::{
        parse::{FromTokenStream, Result},
        stream::TokenStream,
        token::GlottalStop,
    },
};
use std::convert::Infallible;

/// A parsing adjunct.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParsingAdjunct {
    /// The stress indicated by this adjunct.
    pub stress: Stress,
}

impl GlossStatic for ParsingAdjunct {
    fn gloss_static(&self, flags: GlossFlags) -> &'static str {
        self.stress.gloss_static(flags)
    }
}

invalid_type_error!(
    /// a parsing adjunct
    enum ParsingTokenError {
        ExpectedVp,
        ExpectedGs,
        TooManyTokens,
    }
);

impl FromTokenStream for ParsingAdjunct {
    type TypeErr = ParsingTokenError;
    type ValueErr = Infallible;

    fn from_token_stream(mut stream: TokenStream) -> Result<Self, Self::TypeErr, Self::ValueErr> {
        // Parsing adjunct: Vp Gs
        let stress = stream.next_or_err(ParsingTokenError::ExpectedVp)?;
        stream.next_or_err::<GlottalStop, _, _>(ParsingTokenError::ExpectedGs)?;
        stream.done_or_err(ParsingTokenError::ExpectedGs)?;
        Ok(ParsingAdjunct { stress })
    }
}
