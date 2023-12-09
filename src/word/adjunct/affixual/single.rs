use crate::{
    affix::RegularAffix,
    category::{AffixualAdjunctMode, AffixualAdjunctScope, Stress},
    prelude::*,
    romanize::{
        segment::{Vs, VxCs},
        stream::TokenStream,
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

impl FromTokens for SingleAffixAdjunct {
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

impl IntoTokens for SingleAffixAdjunct {
    fn append_to(&self, list: &mut TokenList) {
        let (vx, cs) = self.affix.into_vx_cs();
        let vs = if self.mode == AffixualAdjunctMode::Concatenated
            || self.scope != AffixualAdjunctScope::default()
            || !cs.is_valid_word_final()
        {
            Some(Vs { scope: self.scope })
        } else {
            None
        };
        list.push(vx);
        list.push(cs);
        if let Some(vs) = vs {
            list.push(vs);
        }
        list.set_stress(match self.mode {
            AffixualAdjunctMode::Concatenated => Some(Stress::Ultimate),
            AffixualAdjunctMode::Full => Some(Stress::Penultimate),
        });
    }
}
