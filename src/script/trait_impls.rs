use std::mem::replace;

use super::{
    buf::CharacterBuf,
    character::{
        AccessorQuaternary, Character, Core, Diacritic, Ext, Numeral, Primary, Register, Secondary,
        StandardQuaternary, Tertiary, TertiarySegment,
    },
    flags::IntoScriptFlags,
    traits::{IntoCharacter, IntoScript, IntoSecondary},
};
use crate::{
    affix::{
        AffixList, AppositiveReferentialAffix, CaStackingAffix, CaseAccessorAffix, NumericAffix,
        PlainAffix, RegularAffix, ThematicReferentialAffix,
    },
    category::{
        AffixDegree, AffixSlot, AffixType, ArbitraryMoodOrCaseScope, Aspect, Bias, CaseScope,
        DatalessRelation, DestructuredConfiguration, Effect, Illocution, Level, Mood, Phase,
        Valence, VcOrVk, Vn,
    },
    prelude::{token::NumeralForm, AsGeneral, AsSpecific},
    word::{
        formative::root::{
            AffixualFormativeRoot, NormalFormativeRoot, NumericFormativeRoot,
            ReferentialFormativeRoot, ShortcutCheckedFormativeRoot,
        },
        UncheckedFormative,
    },
};
use vec1::Vec1;

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
            superposed: Diacritic::affix_type(self.r#type),
            underposed: Diacritic::accessor_quaternary_underposed(self.slot, self.mode).into(),
            leftposed: None,
            rightposed: None,
        }
    }
}

impl IntoSecondary for Numeral {
    fn into_secondary(self) -> Secondary {
        let mut value = self.value;
        let ones = value % 10;
        value /= 10;
        let tens = value % 10;
        value /= 10;
        let hundreds = value % 10;
        value /= 10;
        let thousands = value % 10;

        Secondary {
            is_rotated: false,
            core: match ones {
                0 => Core::N0,
                1 => Core::N1,
                2 => Core::N2,
                3 => Core::N3,
                4 => Core::N4,
                5 => Core::N5,
                6 => Core::N6,
                7 => Core::N7,
                8 => Core::N8,
                9 => Core::N9,
                _ => unreachable!(),
            },
            top: match hundreds {
                0 => None,
                1 => Some(Ext::N1),
                2 => Some(Ext::N2),
                3 => Some(Ext::N3),
                4 => Some(Ext::N4),
                5 => Some(Ext::N5),
                6 => Some(Ext::N6),
                7 => Some(Ext::N7),
                8 => Some(Ext::N8),
                9 => Some(Ext::N9),
                _ => unreachable!(),
            },
            bottom: match tens {
                0 => None,
                1 => Some(Ext::N1),
                2 => Some(Ext::N2),
                3 => Some(Ext::N3),
                4 => Some(Ext::N4),
                5 => Some(Ext::N5),
                6 => Some(Ext::N6),
                7 => Some(Ext::N7),
                8 => Some(Ext::N8),
                9 => Some(Ext::N9),
                _ => unreachable!(),
            },
            superposed: None,
            underposed: None,
            leftposed: match thousands {
                0 => None,
                1 => Some(Diacritic::N1),
                2 => Some(Diacritic::N2),
                3 => Some(Diacritic::N3),
                4 => Some(Diacritic::N4),
                5 => Some(Diacritic::N5),
                6 => Some(Diacritic::N6),
                7 => Some(Diacritic::N7),
                8 => Some(Diacritic::N8),
                9 => Some(Diacritic::N9),
                _ => unreachable!(),
            },
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

impl IntoScript for NumeralForm {
    fn append_script_to(&self, list: &mut CharacterBuf, _flags: IntoScriptFlags) {
        for char in Numeral::numeral(*self) {
            list.push(char);
        }
    }
}

fn vec1_h() -> Vec1<Secondary> {
    return Vec1::new(Secondary {
        is_rotated: false,
        core: Core::H,
        top: None,
        bottom: None,
        superposed: None,
        underposed: None,
        leftposed: None,
        rightposed: None,
    });
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

        let mut elided_quaternary = false;

        match &self.root {
            ShortcutCheckedFormativeRoot::Normal(NormalFormativeRoot { cr }) => {
                let mut data = Secondary::cr_or_cs(&cr, false, flags).unwrap_or_else(vec1_h);

                if self.cn == ArbitraryMoodOrCaseScope::FAC_CCN
                    && !flags.matches(IntoScriptFlags::KEEP_QUATERNARIES)
                {
                    elided_quaternary = true;

                    let (superposed, underposed) =
                        Diacritic::elided_quaternary_pair(match self.relation {
                            DatalessRelation::VRB => match self.vc.as_vk() {
                                Some(ivl) => ivl.as_general(),
                                None => VcOrVk::Illocution(Illocution::ASR),
                            },
                            _ => VcOrVk::Case(self.vc),
                        });

                    let first = data.first_mut();
                    first.superposed = superposed;
                    first.underposed = underposed;
                }

                for char in data {
                    list.push(char);
                }
            }
            ShortcutCheckedFormativeRoot::Numeric(NumericFormativeRoot { form }) => {
                list.append(*form, flags);
            }
            ShortcutCheckedFormativeRoot::Referential(ReferentialFormativeRoot { referents }) => {
                let mut data = Secondary::cr_or_cs(&referents.to_string(), true, flags)
                    .expect("referent lists should become valid Cr roots");

                data.first_mut().superposed = Some(Diacritic::Dot);

                for el in data {
                    list.push(el);
                }
            }
            ShortcutCheckedFormativeRoot::Affixual(AffixualFormativeRoot { cs, degree }) => {
                let mut data = Secondary::cr_or_cs(&cs, true, flags)
                    .expect("referent lists should become valid Cr roots");

                data.first_mut().underposed = Some(Diacritic::affix_degree(*degree));

                for el in data {
                    list.push(el);
                }
            }
        }

        let mut slot_vii_affixes = self.slot_vii_affixes.clone();

        let mut valences = Vec::new();
        let mut tertiary_segments = Vec::new();
        let mut absolute_levels = Vec::new();
        let mut relative_levels = Vec::new();

        'extract_ternaries: {
            if !flags.matches(IntoScriptFlags::TERTIARIES_AS_AFFIXES) {
                let AffixList::Normal(ref mut new_affixes) = slot_vii_affixes else {
                    break 'extract_ternaries;
                };

                let old_affixes = replace(new_affixes, Vec::new());

                for affix in old_affixes {
                    let RegularAffix::Plain(affix) = affix else {
                        new_affixes.push(affix);
                        continue;
                    };

                    match (&affix.cs[..], affix.r#type, affix.degree) {
                        (Valence::CS_FORM, AffixType::T1, degree) => valences.push(match degree {
                            AffixDegree::D0 => {
                                new_affixes.push(RegularAffix::Plain(affix));
                                continue;
                            }
                            AffixDegree::D1 => Valence::MNO,
                            AffixDegree::D2 => Valence::PRL,
                            AffixDegree::D3 => Valence::CRO,
                            AffixDegree::D4 => Valence::RCP,
                            AffixDegree::D5 => Valence::CPL,
                            AffixDegree::D6 => Valence::DUP,
                            AffixDegree::D7 => Valence::DEM,
                            AffixDegree::D8 => Valence::CNG,
                            AffixDegree::D9 => Valence::PTI,
                        }),

                        (Phase::CS_FORM, AffixType::T1, degree) => {
                            tertiary_segments.push(TertiarySegment::Phase(match degree {
                                AffixDegree::D0 => {
                                    new_affixes.push(RegularAffix::Plain(affix));
                                    continue;
                                }
                                AffixDegree::D1 => Phase::PUN,
                                AffixDegree::D2 => Phase::ITR,
                                AffixDegree::D3 => Phase::REP,
                                AffixDegree::D4 => Phase::ITM,
                                AffixDegree::D5 => Phase::RCT,
                                AffixDegree::D6 => Phase::FRE,
                                AffixDegree::D7 => Phase::FRG,
                                AffixDegree::D8 => Phase::VAC,
                                AffixDegree::D9 => Phase::FLC,
                            }))
                        }

                        (Effect::CS_FORM, AffixType::T1, degree) => {
                            tertiary_segments.push(TertiarySegment::Effect(match degree {
                                AffixDegree::D0 => {
                                    new_affixes.push(RegularAffix::Plain(affix));
                                    continue;
                                }
                                AffixDegree::D1 => Effect::BEN1,
                                AffixDegree::D2 => Effect::BEN2,
                                AffixDegree::D3 => Effect::BEN3,
                                AffixDegree::D4 => Effect::BENSELF,
                                AffixDegree::D5 => Effect::UNK,
                                AffixDegree::D6 => Effect::DETSELF,
                                AffixDegree::D7 => Effect::DET3,
                                AffixDegree::D8 => Effect::DET2,
                                AffixDegree::D9 => Effect::DET1,
                            }))
                        }

                        (Aspect::CS_FORM_1, AffixType::T1, degree) => {
                            tertiary_segments.push(TertiarySegment::Aspect(match degree {
                                AffixDegree::D0 => {
                                    new_affixes.push(RegularAffix::Plain(affix));
                                    continue;
                                }
                                AffixDegree::D1 => Aspect::RTR,
                                AffixDegree::D2 => Aspect::PRS,
                                AffixDegree::D3 => Aspect::HAB,
                                AffixDegree::D4 => Aspect::PRG,
                                AffixDegree::D5 => Aspect::IMM,
                                AffixDegree::D6 => Aspect::PCS,
                                AffixDegree::D7 => Aspect::REG,
                                AffixDegree::D8 => Aspect::SMM,
                                AffixDegree::D9 => Aspect::ATP,
                            }))
                        }

                        (Aspect::CS_FORM_2, AffixType::T1, degree) => {
                            tertiary_segments.push(TertiarySegment::Aspect(match degree {
                                AffixDegree::D0 => {
                                    new_affixes.push(RegularAffix::Plain(affix));
                                    continue;
                                }
                                AffixDegree::D1 => Aspect::RSM,
                                AffixDegree::D2 => Aspect::CSS,
                                AffixDegree::D3 => Aspect::PAU,
                                AffixDegree::D4 => Aspect::RGR,
                                AffixDegree::D5 => Aspect::PCL,
                                AffixDegree::D6 => Aspect::CNT,
                                AffixDegree::D7 => Aspect::ICS,
                                AffixDegree::D8 => Aspect::EXP,
                                AffixDegree::D9 => Aspect::IRP,
                            }))
                        }

                        (Aspect::CS_FORM_3, AffixType::T1, degree) => {
                            tertiary_segments.push(TertiarySegment::Aspect(match degree {
                                AffixDegree::D0 => {
                                    new_affixes.push(RegularAffix::Plain(affix));
                                    continue;
                                }
                                AffixDegree::D1 => Aspect::PMP,
                                AffixDegree::D2 => Aspect::CLM,
                                AffixDegree::D3 => Aspect::DLT,
                                AffixDegree::D4 => Aspect::TMP,
                                AffixDegree::D5 => Aspect::XPD,
                                AffixDegree::D6 => Aspect::LIM,
                                AffixDegree::D7 => Aspect::EPD,
                                AffixDegree::D8 => Aspect::PTC,
                                AffixDegree::D9 => Aspect::PPR,
                            }))
                        }

                        (Aspect::CS_FORM_4, AffixType::T1, degree) => {
                            tertiary_segments.push(TertiarySegment::Aspect(match degree {
                                AffixDegree::D0 => {
                                    new_affixes.push(RegularAffix::Plain(affix));
                                    continue;
                                }
                                AffixDegree::D1 => Aspect::DCL,
                                AffixDegree::D2 => Aspect::CCL,
                                AffixDegree::D3 => Aspect::CUL,
                                AffixDegree::D4 => Aspect::IMD,
                                AffixDegree::D5 => Aspect::TRD,
                                AffixDegree::D6 => Aspect::TNS,
                                AffixDegree::D7 => Aspect::ITC,
                                AffixDegree::D8 => Aspect::MTV,
                                AffixDegree::D9 => Aspect::SQN,
                            }))
                        }

                        (Level::CS_FORM, AffixType::T1, degree) => {
                            relative_levels.push(match degree {
                                AffixDegree::D0 => {
                                    new_affixes.push(RegularAffix::Plain(affix));
                                    continue;
                                }
                                AffixDegree::D1 => Level::MIN,
                                AffixDegree::D2 => Level::SBE,
                                AffixDegree::D3 => Level::IFR,
                                AffixDegree::D4 => Level::DFC,
                                AffixDegree::D5 => Level::EQU,
                                AffixDegree::D6 => Level::SUR,
                                AffixDegree::D7 => Level::SPL,
                                AffixDegree::D8 => Level::SPQ,
                                AffixDegree::D9 => Level::MAX,
                            })
                        }

                        (Level::CS_FORM, AffixType::T2, degree) => {
                            absolute_levels.push(match degree {
                                AffixDegree::D0 => {
                                    new_affixes.push(RegularAffix::Plain(affix));
                                    continue;
                                }
                                AffixDegree::D1 => Level::MIN,
                                AffixDegree::D2 => Level::SBE,
                                AffixDegree::D3 => Level::IFR,
                                AffixDegree::D4 => Level::DFC,
                                AffixDegree::D5 => Level::EQU,
                                AffixDegree::D6 => Level::SUR,
                                AffixDegree::D7 => Level::SPL,
                                AffixDegree::D8 => Level::SPQ,
                                AffixDegree::D9 => Level::MAX,
                            })
                        }

                        _ => new_affixes.push(RegularAffix::Plain(affix)),
                    }
                }
            }
        }

        match self.vn {
            Vn::Valence(Valence::MNO) => {}
            Vn::Valence(value) => valences.push(value),
            Vn::Phase(value) => tertiary_segments.push(TertiarySegment::Phase(value)),
            Vn::Effect(value) => tertiary_segments.push(TertiarySegment::Effect(value)),
            Vn::Aspect(value) => tertiary_segments.push(TertiarySegment::Aspect(value)),
            Vn::Level(value) => relative_levels.push(value),
        }

        let mut case_stacking_affixes = Vec::new();
        let mut referentials = CharacterBuf::new();

        for (affix_list, slot, is_rotated) in [
            (&self.slot_v_affixes, AffixSlot::V, false),
            (&slot_vii_affixes, AffixSlot::VII, true),
        ] {
            match affix_list {
                AffixList::AppositiveReferential(AppositiveReferentialAffix {
                    case,
                    referents,
                }) => {
                    referentials.push(StandardQuaternary {
                        case_scope: CaseScope::CCN,
                        mood: Mood::FAC,
                        vc_or_vk: VcOrVk::Case(case.as_general()),
                    });

                    let mut data = Secondary::cr_or_cs(&referents.to_string(), false, flags)
                        .expect("referent lists should become valid Cr roots");

                    data.first_mut().superposed = Some(Diacritic::HorizBar);

                    for el in data {
                        referentials.push(el);
                    }
                }

                AffixList::Normal(affixes) => {
                    for affix in affixes {
                        match affix {
                            RegularAffix::Plain(PlainAffix { cs, degree, r#type }) => {
                                let mut data = Secondary::cr_or_cs(&cs, is_rotated, flags)
                                    .unwrap_or_else(vec1_h);

                                let first = data.first_mut();
                                first.superposed = Diacritic::affix_type(*r#type);
                                first.underposed = Some(Diacritic::affix_degree(*degree));

                                for char in data {
                                    list.push(char);
                                }
                            }

                            RegularAffix::Numeric(NumericAffix {
                                degree,
                                form,
                                r#type,
                            }) => {
                                let mut data = Secondary::numeral(*form);

                                let first = data.first_mut();
                                first.superposed = Diacritic::numeric_affix_type(*r#type, slot);
                                first.underposed = Some(Diacritic::affix_degree(*degree));

                                for char in data {
                                    list.push(char);
                                }
                            }

                            RegularAffix::Ca(CaStackingAffix { ca }) => {
                                let mut data = Secondary::cr_or_cs(
                                    &ca.to_ungeminated_string(),
                                    is_rotated,
                                    flags,
                                )
                                .expect("Ca forms should be valid Cs forms");

                                let first = data.first_mut();
                                first.underposed = Some(Diacritic::CA_STACKING_UNDERPOSED);

                                for char in data {
                                    list.push(char);
                                }
                            }

                            RegularAffix::CaseStacking(case) => {
                                // `slot` is ignored because JQ never told us how to write it.
                                case_stacking_affixes.push(case.case);
                            }

                            RegularAffix::CaseAccessor(CaseAccessorAffix {
                                case,
                                mode,
                                r#type,
                            }) => {
                                list.push(AccessorQuaternary {
                                    case: *case,
                                    mode: *mode,
                                    slot,
                                    r#type: *r#type,
                                });
                            }

                            RegularAffix::Referential(ThematicReferentialAffix {
                                case,
                                referents,
                            }) => {
                                referentials.push(StandardQuaternary {
                                    case_scope: CaseScope::CCN,
                                    mood: Mood::FAC,
                                    vc_or_vk: VcOrVk::Case(case.as_general()),
                                });

                                let mut data =
                                    Secondary::cr_or_cs(&referents.to_string(), false, flags)
                                        .expect("referent lists should become valid Cr roots");

                                data.first_mut().superposed = Some(Diacritic::HorizBar);

                                for el in data {
                                    referentials.push(el);
                                }
                            }
                        }
                    }
                }
            }
        }

        while !(valences.is_empty()
            && tertiary_segments.is_empty()
            && absolute_levels.is_empty()
            && relative_levels.is_empty())
        {
            let (top, bottom) = if flags.matches(IntoScriptFlags::PREFER_BOTTOM_TERTIARIES) {
                let bottom = tertiary_segments.pop();
                let top = tertiary_segments.pop();
                (top, bottom)
            } else {
                let top = tertiary_segments.pop();
                let bottom = tertiary_segments.pop();
                (top, bottom)
            };

            list.push(Tertiary {
                valence: valences.pop().unwrap_or_default(),
                top,
                bottom,
                superposed: absolute_levels.pop(),
                underposed: relative_levels.pop(),
            });
        }

        for case in case_stacking_affixes {
            list.push(StandardQuaternary {
                case_scope: CaseScope::CCN,
                mood: Mood::FAC,
                vc_or_vk: VcOrVk::Case(case),
            });
        }

        if !elided_quaternary {
            let (case_scope, mood, vc_or_vk) = match self.relation {
                DatalessRelation::VRB => (
                    CaseScope::CCN,
                    self.cn.as_specific(),
                    match self.vc.as_vk() {
                        Some(ivl) => ivl.as_general(),
                        None => VcOrVk::Illocution(Illocution::ASR),
                    },
                ),
                _ => (self.cn.as_specific(), Mood::FAC, VcOrVk::Case(self.vc)),
            };

            list.push(StandardQuaternary {
                case_scope,
                mood,
                vc_or_vk,
            });
        }

        list.vec.extend_from_slice(&referentials.vec);
    }
}

#[cfg(test)]
mod tests {
    use crate::word::UncheckedFormative;

    #[test]
    fn irburučpaizya() {
        use crate::{prelude::*, script::repr::IthkuilBasicEncoding};

        let word = UncheckedFormative::parse_str("irburučpaizya", FromTokenFlags::NONE).unwrap();
        let script = word.into_script(IntoScriptFlags::NONE);
        let encoded = IthkuilBasicEncoding::from_chars(&script.vec[..]);
        assert_eq!(encoded.0, r#"\^p_xr_bč'_p_ä|^t^a_aò"#);
    }
}
