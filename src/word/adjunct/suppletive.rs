//! Contains types related to suppletive adjuncts.

use crate::{
    category::{Case, SuppletiveAdjunctMode},
    gloss::{Gloss, GlossFlags, GlossHelpers, GlossStatic},
    prelude::{IntoTokens, IntoTokensFlags, TokenList},
    romanize::{
        flags::FromTokenFlags,
        stream::{ParseError, TokenStream},
        traits::FromTokens,
    },
};

/// A suppletive adjunct.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SuppletiveAdjunct {
    /// The mode of this adjunct (CAR, QUO, NAM, or PHR).
    pub mode: SuppletiveAdjunctMode,

    /// The case of this adjunct.
    pub case: Case,
}

impl Gloss for SuppletiveAdjunct {
    fn gloss(&self, flags: GlossFlags) -> String {
        let mut output = self.mode.gloss(flags);
        output.add_dashed(self.case.gloss_static_non_default(flags));
        output
    }
}

impl FromTokens for SuppletiveAdjunct {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        Ok(SuppletiveAdjunct {
            mode: stream.parse(flags)?,
            case: stream.parse(flags)?,
        })
    }
}

impl IntoTokens for SuppletiveAdjunct {
    fn append_to(&self, list: &mut TokenList, _flags: IntoTokensFlags) {
        list.push(self.mode);
        list.push(self.case);
    }
}
