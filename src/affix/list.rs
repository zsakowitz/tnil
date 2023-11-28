use super::{AppositiveReferentialAffix, NormalAffix};
use crate::gloss::{Gloss, GlossFlags};

/// A list of affixes in a formative slot.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AffixList {
    /// A list of normal affixes.
    Normal(Vec<NormalAffix>),

    /// A single appositive referential affix.
    AppositiveReferential(AppositiveReferentialAffix),
}

impl Default for AffixList {
    fn default() -> Self {
        AffixList::Normal(Vec::new())
    }
}

impl Gloss for AffixList {
    fn gloss(&self, flags: GlossFlags) -> String {
        match self {
            Self::AppositiveReferential(value) => value.gloss(flags),

            Self::Normal(value) => {
                let mut output = String::new();
                let mut is_first = true;

                for affix in value {
                    if !is_first {
                        output += "-";
                    }

                    is_first = false;

                    output += &affix.gloss(flags);
                }

                output
            }
        }
    }
}
