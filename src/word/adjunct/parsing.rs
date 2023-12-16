//! Contains types related to parsing adjuncts.

use crate::{
    category::Stress,
    gloss::{GlossFlags, GlossStatic},
    prelude::{IntoTokens, IntoTokensFlags, TokenList},
    romanize::{
        flags::FromTokenFlags,
        stream::{ParseError, TokenStream},
        token::GlottalStop,
        traits::FromTokens,
    },
};

/// A parsing adjunct.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParsingAdjunct {
    /// The stress indicated by this adjunct.
    pub stress: Stress,
}

impl GlossStatic for ParsingAdjunct {
    fn gloss_static(&self, flags: GlossFlags) -> &'static str {
        self.stress.gloss_static(flags)
    }
}

impl FromTokens for ParsingAdjunct {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        let stress = stream.parse(flags)?;
        let _: GlottalStop = stream.next().ok_or(ParseError::ExpectedGlottalStop)?;
        Ok(ParsingAdjunct { stress })
    }
}

impl IntoTokens for ParsingAdjunct {
    fn append_tokens_to(&self, list: &mut TokenList, _flags: IntoTokensFlags) {
        list.push(self.stress);
        list.push(GlottalStop);
    }
}
