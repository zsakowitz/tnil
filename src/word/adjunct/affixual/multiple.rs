use crate::{
    affix::RegularAffix,
    category::{AffixualAdjunctMode, AffixualAdjunctScope, Stress},
    gloss::{Gloss, GlossFlags, GlossHelpers, GlossStatic},
    romanize::{
        flags::FromTokenFlags,
        segment::{CsVxCz, VxCs, Vz},
        stream::{ParseError, TokenStream},
        token::Schwa,
        traits::FromTokens,
    },
};
use vec1::Vec1;

/// An affixual adjunct containing multiple affixes.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MultipleAffixAdjunct {
    /// The first affix of this adjunct.
    pub first_affix: RegularAffix,

    /// The scope of this adjunct (dominant over slot V, subordinate to slot VII, etc.).
    pub first_scope: AffixualAdjunctScope,

    /// The other affixes of this adjunct.
    pub other_affixes: Vec1<RegularAffix>,

    /// The scope of this adjunct (dominant over slot V, subordinate to slot VII, etc.).
    pub other_scope: Option<AffixualAdjunctScope>,

    /// The mode of this adjunct (full or concatenated stem only).
    pub mode: AffixualAdjunctMode,
}

impl Gloss for MultipleAffixAdjunct {
    fn gloss(&self, flags: GlossFlags) -> String {
        let mut output = self.first_affix.gloss(flags);
        output.add_dashed(self.first_scope.gloss_static_non_default(flags));
        for affix in &self.other_affixes {
            output.add_dashed(&affix.gloss(flags));
        }
        if let Some(other_scope) = self.other_scope {
            output.add_dashed(other_scope.gloss_static_non_default(flags));
        }
        output.add_dashed(self.mode.gloss_static_non_default(flags));
        output
    }
}

impl FromTokens for MultipleAffixAdjunct {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        let _ = stream.next::<Schwa>();

        let CsVxCz {
            affix: first_affix,
            scope: first_scope,
        } = stream.parse(flags)?;

        let VxCs {
            affix: second_affix,
        } = stream.parse(flags)?;

        let mut other_affixes = Vec1::new(second_affix);

        while let Ok(VxCs { affix }) = stream.parse(flags) {
            other_affixes.push(affix);
        }

        let Vz { scope: other_scope } = stream.parse(flags)?;

        let mode = match stream.stress() {
            Some(Stress::Ultimate) => AffixualAdjunctMode::Concatenated,
            Some(Stress::Antepenultimate) => return Err(ParseError::AntepenultimateStress),
            _ => AffixualAdjunctMode::Full,
        };

        Ok(MultipleAffixAdjunct {
            first_affix,
            first_scope,
            other_affixes,
            other_scope,
            mode,
        })
    }
}
