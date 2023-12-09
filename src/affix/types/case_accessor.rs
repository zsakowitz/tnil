use crate::{
    category::{AffixType, Case, CaseAccessorMode},
    gloss::{Gloss, GlossFlags, GlossStatic},
    prelude::{
        token::{OwnedConsonantForm, Token, VowelForm},
        IntoVowelForm, IntoVxCs,
    },
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

impl IntoVxCs for CaseAccessorAffix {
    fn into_vx_cs(&self) -> (VowelForm, Token) {
        (
            {
                let mut case = self.case.into_vowel_form();
                case.has_glottal_stop = false;
                case
            },
            Token::C(OwnedConsonantForm(
                if self.case >= Case::PRN {
                    match self.mode {
                        CaseAccessorMode::Normal => match self.r#type {
                            AffixType::T1 => "sy",
                            AffixType::T2 => "zy",
                            AffixType::T3 => "čy",
                        },
                        CaseAccessorMode::Inverse => match self.r#type {
                            AffixType::T1 => "šy",
                            AffixType::T2 => "žy",
                            AffixType::T3 => "jy",
                        },
                    }
                } else {
                    match self.mode {
                        CaseAccessorMode::Normal => match self.r#type {
                            AffixType::T1 => "sw",
                            AffixType::T2 => "zw",
                            AffixType::T3 => "čw",
                        },
                        CaseAccessorMode::Inverse => match self.r#type {
                            AffixType::T1 => "šw",
                            AffixType::T2 => "žw",
                            AffixType::T3 => "jw",
                        },
                    }
                }
                .to_owned(),
            )),
        )
    }
}
