use crate::{
    affix::RegularAffix,
    category::{AffixualAdjunctMode, AffixualAdjunctScope},
    gloss::{Gloss, GlossFlags, GlossHelpers, GlossStatic},
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
