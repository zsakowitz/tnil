//! Contains types related to bias adjuncts.

use std::convert::Infallible;

use crate::{
    category::Bias,
    gloss::{GlossFlags, GlossStatic},
    macros::invalid_type_error,
    romanize::{
        parse::{FromTokenStream, Result},
        stream::TokenStream,
    },
};

/// A bias adjunct.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BiasAdjunct {
    /// The bias of this adjunct.
    pub bias: Bias,
}

impl GlossStatic for BiasAdjunct {
    fn gloss_static(&self, flags: GlossFlags) -> &'static str {
        self.bias.gloss_static(flags)
    }
}

invalid_type_error!(
    /// a bias adjunct
    enum BiasTokenError {
        ExpectedCb,
        TooManyTokens,
    }
);

impl FromTokenStream for BiasAdjunct {
    type TypeErr = BiasTokenError;
    type ValueErr = Infallible;

    fn from_token_stream(mut stream: TokenStream) -> Result<Self, Self::TypeErr, Self::ValueErr> {
        // Bias adjunct: Cb
        let bias = stream.next_or_err(BiasTokenError::ExpectedCb)?;
        stream.done_or_err(BiasTokenError::TooManyTokens)?;
        Ok(BiasAdjunct { bias })
    }
}
