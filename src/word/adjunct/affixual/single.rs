use crate::{
    affix::RegularAffix,
    category::{AffixualAdjunctMode, AffixualAdjunctScope, Stress},
    gloss::{Gloss, GlossFlags, GlossHelpers, GlossStatic},
    romanize::{
        flags::FromTokenFlags,
        segment::{Vs, VxCs},
        stream::{FromTokenStream, ParseError, TokenStream},
    },
};

/// An affixual adjunct containing a single affix.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SingleAffixAdjunct {
    /// The affix of this adjunct.
    pub affix: RegularAffix,

    /// The scope of this adjunct (dominant over slot V, subordinate to slot VII, etc.).
    pub scope: AffixualAdjunctScope,

    /// The mode of this adjunct (full or concatenated stem only).
    pub mode: AffixualAdjunctMode,
}

impl Gloss for SingleAffixAdjunct {
    fn gloss(&self, flags: GlossFlags) -> String {
        let mut output = self.affix.gloss(flags);
        output.add_dashed(self.scope.gloss_static_non_default(flags));
        output.add_dashed(self.mode.gloss_static_non_default(flags));
        output
    }
}

impl FromTokenStream for SingleAffixAdjunct {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        let VxCs { affix } = stream.parse(flags)?;
        let Vs { scope } = stream.parse(flags)?;
        let mode = match stream.stress() {
            Some(Stress::Ultimate) => AffixualAdjunctMode::Concatenated,
            Some(Stress::Antepenultimate) => return Err(ParseError::AntepenultimateStress),
            _ => AffixualAdjunctMode::Full,
        };
        Ok(Self { affix, scope, mode })
    }
}
