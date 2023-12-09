//! Defines types related to mood/case-scope adjuncts.

use crate::{
    category::MoodOrCaseScope,
    gloss::{GlossFlags, GlossStatic},
    prelude::{IntoTokens, TokenList},
    romanize::{
        flags::FromTokenFlags,
        stream::{ParseError, TokenStream},
        token::Hr,
        traits::FromTokens,
    },
};

/// A mood/case-scope adjunct.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MCSAdjunct {
    /// The mood or case-scope represented by this adjunct.
    pub mcs: MoodOrCaseScope,
}

impl GlossStatic for MCSAdjunct {
    fn gloss_static(&self, flags: GlossFlags) -> &'static str {
        self.mcs.gloss_static(flags)
    }
}

impl FromTokens for MCSAdjunct {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        stream.parse::<Hr>(flags)?;

        Ok(MCSAdjunct {
            mcs: stream.parse(flags)?,
        })
    }
}

impl IntoTokens for MCSAdjunct {
    fn append_to(&self, list: &mut TokenList) {
        list.push(Hr);
    }
}
