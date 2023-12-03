//! Contains types related to parsing adjuncts.

use crate::{
    category::Stress,
    gloss::{GlossFlags, GlossStatic},
    romanize::{
        flags::FromTokenFlags,
        stream::{FromTokenStream, ParseError, TokenStream},
        token::GlottalStop,
    },
};

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

impl FromTokenStream for ParsingAdjunct {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        let stress = stream.parse(flags)?;
        stream.parse::<GlottalStop>(flags)?;
        Ok(ParsingAdjunct { stress })
    }
}
