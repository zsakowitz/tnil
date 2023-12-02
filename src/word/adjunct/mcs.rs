//! Defines types related to mood/case-scope adjuncts.

use crate::{
    category::MoodOrCaseScope,
    gloss::{GlossFlags, GlossStatic},
    romanize::{
        stream::{FromTokenStream, ParseError, TokenStream},
        token::Hr,
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

impl FromTokenStream for MCSAdjunct {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        stream.parse::<Hr>()?;

        Ok(MCSAdjunct {
            mcs: stream.parse()?,
        })
    }
}
