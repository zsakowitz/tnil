//! Contains types related to bias adjuncts.

use crate::{
    category::Bias,
    gloss::{GlossFlags, GlossStatic},
    prelude::{token::Token, IntoToken},
    romanize::{
        flags::FromTokenFlags,
        stream::{ParseError, TokenStream},
        traits::FromTokens,
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

impl FromTokens for BiasAdjunct {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        Ok(BiasAdjunct {
            bias: stream.parse(flags)?,
        })
    }
}

impl IntoToken for BiasAdjunct {
    fn into_token(self) -> Token {
        self.bias.into_token()
    }
}
