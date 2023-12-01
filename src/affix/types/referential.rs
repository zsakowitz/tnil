use crate::{
    category::{AffixualReferentList, AppositiveCase, ThematicCase},
    gloss::{Gloss, GlossFlags, GlossStatic},
};

/// A referential affix. The `CaseType` generic specifies the type of case this affix uses, to allow
/// this type to be reused between thematic and appositive referential affixes.
///
/// Two common variants of this type, based on thematic and appositive cases, are exposed under the
/// aliases [`ThematicReferentialAffix`] and [`AppositiveReferentialAffix`] respectively.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReferentialAffix<CaseType> {
    /// The referents and perspective of this affix.
    pub referents: AffixualReferentList,

    /// The case of this affix.
    pub case: CaseType,
}

impl<CaseType> ReferentialAffix<CaseType> {
    /// Creates a new [`ReferentialAffix`].
    pub fn new(referents: AffixualReferentList, case: CaseType) -> Self {
        Self { referents, case }
    }
}

/// A referential affix containing a [`ThematicCase`].
pub type ThematicReferentialAffix = ReferentialAffix<ThematicCase>;

/// A referential affix containing an [`AppositiveCase`].
pub type AppositiveReferentialAffix = ReferentialAffix<AppositiveCase>;

impl<CaseType> Gloss for ReferentialAffix<CaseType>
where
    CaseType: Copy + GlossStatic,
{
    fn gloss(&self, flags: GlossFlags) -> String {
        let mut output = "(".to_owned();
        output += &self.referents.gloss(flags);
        output += "-";
        output += self.case.gloss_static(flags);
        output += ")";
        output
    }
}
