//! Contains types related to bias adjuncts.

use crate::{
    category::Bias,
    gloss::{GlossFlags, GlossStatic},
    romanize::{
        stream::FromTokenStream,
        stream::{ParseError, TokenStream},
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

impl FromTokenStream for BiasAdjunct {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        Ok(BiasAdjunct {
            bias: stream.parse()?,
        })
    }
}
