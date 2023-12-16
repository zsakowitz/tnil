use super::{AppositiveReferentialAffix, RegularAffix};
use crate::{
    category::{VowelFormDegree, VowelFormSequence},
    gloss::{Gloss, GlossFlags},
    romanize::{
        stream::ParseError,
        token::{OwnedConsonantForm, VowelForm},
    },
};

/// A list of affixes in a formative slot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AffixList {
    /// A list of normal affixes.
    Normal(Vec<RegularAffix>),

    /// A single appositive referential affix.
    AppositiveReferential(AppositiveReferentialAffix),
}

impl AffixList {
    /// Creates an [`AffixList`] from a slice of VxCs pairs. If the slice contains a single affix
    /// and the affix is a type-3 affix, it will be interpreted as an appositive referential.
    /// Otherwise, the affix list will be interpreted normally.
    pub fn from_vxcs_slice(forms: &[(VowelForm, OwnedConsonantForm)]) -> Result<Self, ParseError> {
        'a: {
            if forms.len() == 1 {
                let first = &forms[0];

                if first.0.sequence == VowelFormSequence::S3
                    && !matches!(
                        &**first.1,
                        "lw" | "ly"
                            | "sw"
                            | "zw"
                            | "čw"
                            | "šw"
                            | "žw"
                            | "jw"
                            | "sy"
                            | "zy"
                            | "čy"
                            | "šy"
                            | "žy"
                            | "jy"
                    )
                {
                    return Ok(AffixList::AppositiveReferential(
                        AppositiveReferentialAffix {
                            case: match first.0.degree {
                                VowelFormDegree::D0 => break 'a,
                                VowelFormDegree::D1 => crate::category::AppositiveCase::POS,
                                VowelFormDegree::D2 => crate::category::AppositiveCase::PRP,
                                VowelFormDegree::D3 => crate::category::AppositiveCase::GEN,
                                VowelFormDegree::D4 => crate::category::AppositiveCase::ATT,
                                VowelFormDegree::D5 => crate::category::AppositiveCase::PDC,
                                VowelFormDegree::D6 => crate::category::AppositiveCase::ITP,
                                VowelFormDegree::D7 => crate::category::AppositiveCase::OGN,
                                VowelFormDegree::D8 => crate::category::AppositiveCase::IDP,
                                VowelFormDegree::D9 => crate::category::AppositiveCase::PAR,
                            },
                            referents: first.1.parse()?,
                        },
                    ));
                }
            }
        }

        let mut items = Vec::new();

        for (vx, cs) in forms {
            items.push(RegularAffix::from_vxcs(*vx, &**cs)?);
        }

        Ok(Self::Normal(items))
    }

    /// Gets the number of affixes in this list.
    pub fn len(&self) -> usize {
        match self {
            Self::Normal(items) => items.len(),
            Self::AppositiveReferential(_) => 1,
        }
    }
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
