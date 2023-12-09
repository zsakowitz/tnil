use super::{
    types::{
        CaStackingAffix, CaseAccessorAffix, CaseStackingAffix, PlainAffix, ThematicReferentialAffix,
    },
    NumericAffix,
};
use crate::{
    category::{
        AffixDegree, AffixType, Ca, Case, CaseAccessorMode, ThematicCase, VowelFormDegree,
        VowelFormSequence,
    },
    gloss::{Gloss, GlossFlags},
    prelude::{token::Token, IntoVxCs},
    romanize::{stream::ParseError, token::VowelForm},
};

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// Any affix other than appositive referential affixes.
pub enum RegularAffix {
    /// A plain affix.
    Plain(PlainAffix),

    /// A numeric affix.
    Numeric(NumericAffix),

    /// A Ca-stacking affix.
    Ca(CaStackingAffix),

    /// A case-stacking affix.
    CaseStacking(CaseStackingAffix),

    /// A case-accessor affix.
    CaseAccessor(CaseAccessorAffix),

    /// A referential affix with a thematic case.
    Referential(ThematicReferentialAffix),
}

impl RegularAffix {
    /// Parses a [`RegularAffix`] from a VxCs pair.
    pub fn from_vxcs(vx: VowelForm, cs: &str) -> Result<Self, ParseError> {
        if matches!(
            vx,
            VowelForm {
                has_glottal_stop: _,
                sequence: VowelFormSequence::S4,
                degree: VowelFormDegree::D0
            }
        ) {
            return Ok(RegularAffix::Ca(CaStackingAffix {
                ca: Ca::from_ungeminated_string(cs).ok_or(ParseError::ExpectedCa)?,
            }));
        }

        match cs {
            "lw" | "ly" => {
                let mut vx = vx;
                vx.has_glottal_stop = cs == "ly";
                return Ok(RegularAffix::CaseStacking(CaseStackingAffix {
                    case: Case::from_vc(vx)?,
                }));
            }

            "sw" | "zw" | "čw" | "šw" | "žw" | "jw" | "sy" | "zy" | "čy" | "šy" | "žy" | "jy" =>
            {
                let mut vx = vx;
                vx.has_glottal_stop = cs == "ly";
                let case = Case::from_vc(vx)?;
                let mode = if cs.starts_with(['š', 'ž', 'j']) {
                    CaseAccessorMode::Inverse
                } else {
                    CaseAccessorMode::Normal
                };
                let r#type = if cs.starts_with(['s', 'š']) {
                    AffixType::T1
                } else if cs.starts_with(['z', 'ž']) {
                    AffixType::T2
                } else {
                    AffixType::T3
                };

                return Ok(RegularAffix::CaseAccessor(CaseAccessorAffix {
                    case,
                    mode,
                    r#type,
                }));
            }

            _ => {}
        }

        let r#type = match vx.sequence {
            VowelFormSequence::S1 => AffixType::T1,
            VowelFormSequence::S2 => AffixType::T2,
            VowelFormSequence::S3 => AffixType::T3,
            VowelFormSequence::S4 => {
                let case = match vx.degree {
                    VowelFormDegree::D0 => {
                        unreachable!("should be registered as a Ca-stacking affix")
                    }

                    VowelFormDegree::D1 => ThematicCase::THM,
                    VowelFormDegree::D2 => ThematicCase::INS,
                    VowelFormDegree::D3 => ThematicCase::ABS,
                    VowelFormDegree::D4 => ThematicCase::AFF,
                    VowelFormDegree::D5 => ThematicCase::STM,
                    VowelFormDegree::D6 => ThematicCase::EFF,
                    VowelFormDegree::D7 => ThematicCase::ERG,
                    VowelFormDegree::D8 => ThematicCase::DAT,
                    VowelFormDegree::D9 => ThematicCase::IND,
                };

                return Ok(RegularAffix::Referential(super::ReferentialAffix {
                    referents: cs.parse()?,
                    case,
                }));
            }
        };

        let degree = match vx.degree {
            VowelFormDegree::D0 => AffixDegree::D0,
            VowelFormDegree::D1 => AffixDegree::D1,
            VowelFormDegree::D2 => AffixDegree::D2,
            VowelFormDegree::D3 => AffixDegree::D3,
            VowelFormDegree::D4 => AffixDegree::D4,
            VowelFormDegree::D5 => AffixDegree::D5,
            VowelFormDegree::D6 => AffixDegree::D6,
            VowelFormDegree::D7 => AffixDegree::D7,
            VowelFormDegree::D8 => AffixDegree::D8,
            VowelFormDegree::D9 => AffixDegree::D9,
        };

        Ok(RegularAffix::Plain(PlainAffix {
            cs: cs.to_owned(),
            r#type,
            degree,
        }))
    }
}

impl Gloss for RegularAffix {
    fn gloss(&self, flags: GlossFlags) -> String {
        match self {
            Self::Plain(value) => value.gloss(flags),
            Self::Numeric(value) => value.gloss(flags),
            Self::Ca(value) => value.gloss(flags),
            Self::CaseStacking(value) => value.gloss(flags),
            Self::CaseAccessor(value) => value.gloss(flags),
            Self::Referential(value) => value.gloss(flags),
        }
    }
}

impl IntoVxCs for RegularAffix {
    fn into_vx_cs(&self) -> (VowelForm, Token) {
        match self {
            Self::Plain(value) => value.into_vx_cs(),
            Self::Numeric(value) => value.into_vx_cs(),
            Self::Ca(value) => value.into_vx_cs(),
            Self::CaseStacking(value) => value.into_vx_cs(),
            Self::CaseAccessor(value) => value.into_vx_cs(),
            Self::Referential(value) => value.into_vx_cs(),
        }
    }
}
