use super::{
    buf::CharacterBuf,
    character::{
        AccessorQuaternary, Character, Core, Diacritic, Ext, Primary, Register, Secondary,
        StandardQuaternary, Tertiary,
    },
    flags::IntoScriptFlags,
    traits::{IntoCharacter, IntoScript, IntoSecondary},
};
use crate::{
    category::{ArbitraryMoodOrCaseScope, Bias, DestructuredConfiguration},
    word::{
        formative::root::{NormalFormativeRoot, ShortcutCheckedFormativeRoot},
        UncheckedFormative,
    },
};

impl IntoCharacter for Character {
    fn into_character(self) -> Character {
        self
    }
}

impl IntoSecondary for Secondary {
    fn into_secondary(self) -> Secondary {
        self
    }
}

impl IntoCharacter for Tertiary {
    fn into_character(self) -> Character {
        Character::Tertiary(self)
    }
}

impl IntoCharacter for Register {
    fn into_character(self) -> Character {
        Character::Register(self)
    }
}

impl IntoCharacter for Diacritic {
    fn into_character(self) -> Character {
        Character::Diacritic(self)
    }
}

impl IntoSecondary for Primary {
    fn into_secondary(self) -> Secondary {
        let DestructuredConfiguration {
            plexity,
            similarity_and_separability,
        } = self.ca.configuration.destructure();

        Secondary {
            is_rotated: false,
            core: Core::primary_core(self.specification),
            top: Ext::primary_top(self.ca.extension, self.ca.perspective),
            bottom: Ext::primary_bottom(self.function, self.version, plexity, self.stem),
            superposed: Diacritic::primary_superposed(self.context),
            underposed: Diacritic::primary_underposed(self.relation),
            leftposed: Diacritic::primary_leftposed(similarity_and_separability),
            rightposed: Diacritic::primary_rightposed(self.ca.affiliation, self.ca.essence),
        }
    }
}

impl IntoSecondary for StandardQuaternary {
    fn into_secondary(self) -> Secondary {
        let (top, bottom) = Ext::standard_quaternary_exts(self.vc_or_vk);

        Secondary {
            is_rotated: false,
            core: Core::VertBar,
            top,
            bottom,
            superposed: Diacritic::standard_quaternary_superposed(self.mood),
            underposed: Diacritic::standard_quaternary_underposed(self.case_scope),
            leftposed: None,
            rightposed: None,
        }
    }
}

impl IntoSecondary for AccessorQuaternary {
    fn into_secondary(self) -> Secondary {
        let (top, bottom) = Ext::accessor_quaternary_exts(self.case);

        Secondary {
            is_rotated: false,
            core: Core::VertBar,
            top,
            bottom,
            superposed: Diacritic::affix_type_superposed(self.r#type),
            underposed: Diacritic::accessor_quaternary_underposed(self.slot, self.mode).into(),
            leftposed: None,
            rightposed: None,
        }
    }
}

impl IntoSecondary for Bias {
    fn into_secondary(self) -> Secondary {
        let (is_rotated, top, bottom) = match self {
            Bias::ACC => (true, None, None),
            Bias::ACH => (true, None, Some(Ext::L)),
            Bias::ADS => (true, None, Some(Ext::Z)),
            Bias::ANN => (true, None, Some(Ext::S)),
            Bias::ANP => (true, None, Some(Ext::K)),
            Bias::APB => (true, None, Some(Ext::G)),
            Bias::APH => (true, None, Some(Ext::GeminateExt)),
            Bias::ARB => (true, None, Some(Ext::GeminateCore)),
            Bias::ATE => (true, None, Some(Ext::D)),
            Bias::CMD => (true, None, Some(Ext::T)),
            Bias::CNV => (true, None, Some(Ext::Š)),
            Bias::COI => (true, None, Some(Ext::P)),
            Bias::CRP => (true, None, Some(Ext::X)),
            Bias::CRR => (true, None, Some(Ext::Ř)),
            Bias::CTP => (true, None, Some(Ext::Ţ)),
            Bias::CTV => (true, None, Some(Ext::F)),

            Bias::DCC => {
                return Secondary {
                    is_rotated: true,
                    core: Core::Sigma,
                    top: None,
                    bottom: None,
                    superposed: None,
                    underposed: None,
                    leftposed: Some(Diacritic::Dot),
                    rightposed: None,
                }
            }
            Bias::DEJ => (true, Some(Ext::L), None),
            Bias::DES => (true, Some(Ext::Z), None),
            Bias::DFD => (true, Some(Ext::S), None),
            Bias::DIS => (true, Some(Ext::K), None),
            Bias::DLC => (true, Some(Ext::G), None),
            Bias::DOL => (true, Some(Ext::GeminateExt), None),
            Bias::DPB => (true, Some(Ext::GeminateCore), None),
            Bias::DRS => (true, Some(Ext::D), None),
            Bias::DUB => (true, Some(Ext::T), None),
            Bias::EUH => (true, Some(Ext::Š), None),
            Bias::EUP => (true, Some(Ext::P), None),
            Bias::EXA => (true, Some(Ext::X), None),
            Bias::EXG => (true, Some(Ext::Ř), None),
            Bias::MNF => (true, Some(Ext::Ţ), None),
            Bias::FOR => (true, Some(Ext::F), None),

            Bias::FSC => (false, None, None),
            Bias::GRT => (false, None, Some(Ext::L)),
            Bias::IDG => (false, None, Some(Ext::Z)),
            Bias::IFT => (false, None, Some(Ext::S)),
            Bias::IPL => (false, None, Some(Ext::K)),
            Bias::IPT => (false, None, Some(Ext::G)),
            Bias::IRO => (false, None, Some(Ext::GeminateExt)),
            Bias::ISP => (false, None, Some(Ext::GeminateCore)),
            Bias::IVD => (false, None, Some(Ext::D)),
            Bias::MAN => (false, None, Some(Ext::T)),
            Bias::OPT => (false, None, Some(Ext::Š)),
            Bias::PES => (false, None, Some(Ext::P)),
            Bias::PPT => (false, None, Some(Ext::X)),
            Bias::PPX => (false, None, Some(Ext::Ř)),
            Bias::PPV => (false, None, Some(Ext::Ţ)),
            Bias::PSC => (false, None, Some(Ext::F)),

            Bias::PSM => {
                return Secondary {
                    is_rotated: false,
                    core: Core::Sigma,
                    top: None,
                    bottom: None,
                    superposed: None,
                    underposed: None,
                    leftposed: None,
                    rightposed: Some(Diacritic::Dot),
                }
            }
            Bias::RAC => (false, Some(Ext::L), None),
            Bias::RFL => (false, Some(Ext::Z), None),
            Bias::RSG => (false, Some(Ext::S), None),
            Bias::RPU => (false, Some(Ext::K), None),
            Bias::RVL => (false, Some(Ext::G), None),
            Bias::SAT => (false, Some(Ext::GeminateExt), None),
            Bias::SGS => (false, Some(Ext::GeminateCore), None),
            Bias::SKP => (false, Some(Ext::D), None),
            Bias::SOL => (false, Some(Ext::T), None),
            Bias::STU => (false, Some(Ext::Š), None),
            Bias::TRP => (false, Some(Ext::P), None),
            Bias::VEX => (false, Some(Ext::X), None),
        };

        Secondary {
            is_rotated,
            core: Core::Sigma,
            top,
            bottom,
            superposed: None,
            underposed: None,
            leftposed: None,
            rightposed: None,
        }
    }
}

impl IntoScript for UncheckedFormative {
    fn append_script_to(&self, list: &mut CharacterBuf, flags: IntoScriptFlags) {
        let primary = Primary {
            specification: self.specification,
            ca: self.ca,
            function: self.function,
            version: self.version,
            stem: self.stem,
            context: self.context,
            relation: self.relation,
        };

        if flags.matches(IntoScriptFlags::ELIDE_PRIMARIES) {
            match Diacritic::primary_elided(primary) {
                None => list.push(primary),
                Some(None) => {}
                Some(Some(diacritic)) => list.push(diacritic),
            }
        } else {
            list.push(primary);
        }

        todo!()

        // match &self.root {
        //     ShortcutCheckedFormativeRoot::Normal(NormalFormativeRoot { cr }) => {}
        //     ShortcutCheckedFormativeRoot::Numeric()
        // }

        // if self.cn == ArbitraryMoodOrCaseScope::FAC_CCN
        //     && flags.matches(IntoScriptFlags::ELIDE_QUATERNARIES)
        // {}
    }
}
