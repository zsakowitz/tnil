//! Contains types related to bias adjuncts.

use crate::{
    category::{Bias, ParseBiasError},
    gloss::{GlossFlags, GlossStatic},
    macros::invalid_tokens_error,
    romanize::{
        parse::{FromTokenStream, FromTokensError, Result},
        stream::TokenStream,
        token::ConsonantForm,
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

invalid_tokens_error!(
    /// a bias adjunct
    enum BiasTokensError {
        ExpectedConsonant = "bias adjuncts should have a single consonant form",
        TooManyTokens = "too many tokens",
    }
);

impl FromTokenStream for BiasAdjunct {
    type TypeErr = BiasTokensError;
    type ValueErr = ParseBiasError;

    fn from_token_stream(mut stream: TokenStream) -> Result<Self, Self::TypeErr, Self::ValueErr> {
        let bias: ConsonantForm = stream
            .next()
            .ok_or(FromTokensError::Type(BiasTokensError::ExpectedConsonant))?;

        if !stream.is_done() {
            return Err(FromTokensError::Type(BiasTokensError::TooManyTokens));
        }

        let bias = bias.source.parse().map_err(FromTokensError::Value)?;

        Ok(BiasAdjunct { bias })
    }
}
