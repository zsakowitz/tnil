use super::types::{
    CaStackingAffix, CaseAccessorAffix, CaseStackingAffix, PlainAffix, ThematicReferentialAffix,
};
use crate::gloss::{Gloss, GlossFlags};

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// Any affix other than appositive referential affixes.
pub enum NormalAffix {
    /// A plain affix.
    Plain(PlainAffix),

    /// A Ca-stacking affix.
    Ca(CaStackingAffix),

    /// A case-stacking affix.
    CaseStacking(CaseStackingAffix),

    /// A case-accessor affix.
    CaseAccessor(CaseAccessorAffix),

    /// A referential affix with a thematic case.
    Referential(ThematicReferentialAffix),
}

impl Gloss for NormalAffix {
    fn gloss(&self, flags: GlossFlags) -> String {
        match self {
            Self::Plain(value) => value.gloss(flags),
            Self::Ca(value) => value.gloss(flags),
            Self::CaseStacking(value) => value.gloss(flags),
            Self::CaseAccessor(value) => value.gloss(flags),
            Self::Referential(value) => value.gloss(flags),
        }
    }
}
