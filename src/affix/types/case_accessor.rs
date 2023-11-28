use crate::{
    category::{AffixType, Case, CaseAccessorMode},
    gloss::{Gloss, GlossFlags, GlossStatic},
};

/// A case-accessor affix.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CaseAccessorAffix {
    /// The case this affix accesses.
    pub case: Case,

    /// The mode of this case accessor.
    pub mode: CaseAccessorMode,

    /// The type of this affix.
    pub r#type: AffixType,
}

impl CaseAccessorAffix {
    /// Constructs a new [`CaseAccessorAffix`].
    pub fn new(case: Case, mode: CaseAccessorMode, r#type: AffixType) -> Self {
        Self { case, mode, r#type }
    }
}

impl Gloss for CaseAccessorAffix {
    fn gloss(&self, flags: GlossFlags) -> String {
        let mut output = "(".to_owned();
        output += self.mode.gloss_static(flags);
        output += ":";
        output += self.case.gloss_static(flags);
        output += ")";
        output += self.r#type.gloss_static(flags);
        output
    }
}
