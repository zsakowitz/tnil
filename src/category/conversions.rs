//! This module provides conversions between different categories and complexes defined in
//! crate::category. All conversions are provided both as inherent `const fn`s, and lossless
//! conversions are also implemented as `From` and `Into` implementations.

use crate::{AsGeneral, TryAsGeneral, TryAsSpecific};

use super::{
    Affiliation, AppositiveCase, Aspect, Ca, CaShortcut, Case, CaseScope, Configuration,
    DestructuredConfiguration, Effect, Essence, Extension, Illocution, IllocutionOrValidation,
    Level, Mood, NonDefaultCaseScope, NonDefaultMood, Perspective, Phase, Plexity,
    ReferentialAffixPerspective, Separability, Similarity, SimilarityAndSeparability, ThematicCase,
    Valence, Validation, Vn,
};

impl Configuration {
    /// Gets the [`Plexity`] of this configuration.
    pub const fn plexity(self) -> Plexity {
        match self {
            Configuration::UPX
            | Configuration::MSS
            | Configuration::MSC
            | Configuration::MSF
            | Configuration::MDS
            | Configuration::MDC
            | Configuration::MDF
            | Configuration::MFS
            | Configuration::MFC
            | Configuration::MFF => Plexity::M,

            Configuration::DPX
            | Configuration::DSS
            | Configuration::DSC
            | Configuration::DSF
            | Configuration::DDS
            | Configuration::DDC
            | Configuration::DDF
            | Configuration::DFS
            | Configuration::DFC
            | Configuration::DFF => Plexity::D,
        }
    }

    /// Gets the [`Similarity`] of this configuration.
    pub const fn similarity(self) -> Option<Similarity> {
        match self {
            Configuration::UPX | Configuration::DPX => None,

            Configuration::MSS
            | Configuration::MSC
            | Configuration::MSF
            | Configuration::DSS
            | Configuration::DSC
            | Configuration::DSF => Some(Similarity::S),

            Configuration::MDS
            | Configuration::MDC
            | Configuration::MDF
            | Configuration::DDS
            | Configuration::DDC
            | Configuration::DDF => Some(Similarity::D),

            Configuration::MFS
            | Configuration::MFC
            | Configuration::MFF
            | Configuration::DFS
            | Configuration::DFC
            | Configuration::DFF => Some(Similarity::F),
        }
    }

    /// Gets the [`Separability`] of this configuration.
    pub const fn separability(self) -> Option<Separability> {
        match self {
            Configuration::UPX | Configuration::DPX => None,

            Configuration::MSS
            | Configuration::MDS
            | Configuration::MFS
            | Configuration::DSS
            | Configuration::DDS
            | Configuration::DFS => Some(Separability::S),

            Configuration::MSC
            | Configuration::MDC
            | Configuration::MFC
            | Configuration::DSC
            | Configuration::DDC
            | Configuration::DFC => Some(Separability::C),

            Configuration::MSF
            | Configuration::MDF
            | Configuration::MFF
            | Configuration::DSF
            | Configuration::DDF
            | Configuration::DFF => Some(Separability::F),
        }
    }

    /// Gets the [`SimilarityAndSeparability`] of this configuration.
    pub const fn similarity_and_separability(self) -> Option<SimilarityAndSeparability> {
        match self {
            Configuration::UPX | Configuration::DPX => None,
            Configuration::MSS | Configuration::DSS => Some(SimilarityAndSeparability {
                similarity: Similarity::S,
                separability: Separability::S,
            }),
            Configuration::MSC | Configuration::DSC => Some(SimilarityAndSeparability {
                similarity: Similarity::S,
                separability: Separability::C,
            }),
            Configuration::MSF | Configuration::DSF => Some(SimilarityAndSeparability {
                similarity: Similarity::S,
                separability: Separability::F,
            }),
            Configuration::MDS | Configuration::DDS => Some(SimilarityAndSeparability {
                similarity: Similarity::D,
                separability: Separability::S,
            }),
            Configuration::MDC | Configuration::DDC => Some(SimilarityAndSeparability {
                similarity: Similarity::D,
                separability: Separability::C,
            }),
            Configuration::MDF | Configuration::DDF => Some(SimilarityAndSeparability {
                similarity: Similarity::D,
                separability: Separability::F,
            }),
            Configuration::MFS | Configuration::DFS => Some(SimilarityAndSeparability {
                similarity: Similarity::F,
                separability: Separability::S,
            }),
            Configuration::MFC | Configuration::DFC => Some(SimilarityAndSeparability {
                similarity: Similarity::F,
                separability: Separability::C,
            }),
            Configuration::MFF | Configuration::DFF => Some(SimilarityAndSeparability {
                similarity: Similarity::F,
                separability: Separability::F,
            }),
        }
    }

    /// Destructures into a [`DestructuredConfiguration`].
    pub const fn destructure(self) -> DestructuredConfiguration {
        match self {
            Configuration::UPX => DestructuredConfiguration {
                plexity: Plexity::M,
                similarity_and_separability: None,
            },
            Configuration::MSS => DestructuredConfiguration {
                plexity: Plexity::M,
                similarity_and_separability: Some(SimilarityAndSeparability {
                    similarity: Similarity::S,
                    separability: Separability::S,
                }),
            },
            Configuration::MSC => DestructuredConfiguration {
                plexity: Plexity::M,
                similarity_and_separability: Some(SimilarityAndSeparability {
                    similarity: Similarity::S,
                    separability: Separability::C,
                }),
            },
            Configuration::MSF => DestructuredConfiguration {
                plexity: Plexity::M,
                similarity_and_separability: Some(SimilarityAndSeparability {
                    similarity: Similarity::S,
                    separability: Separability::F,
                }),
            },
            Configuration::MDS => DestructuredConfiguration {
                plexity: Plexity::M,
                similarity_and_separability: Some(SimilarityAndSeparability {
                    similarity: Similarity::D,
                    separability: Separability::S,
                }),
            },
            Configuration::MDC => DestructuredConfiguration {
                plexity: Plexity::M,
                similarity_and_separability: Some(SimilarityAndSeparability {
                    similarity: Similarity::D,
                    separability: Separability::C,
                }),
            },
            Configuration::MDF => DestructuredConfiguration {
                plexity: Plexity::M,
                similarity_and_separability: Some(SimilarityAndSeparability {
                    similarity: Similarity::D,
                    separability: Separability::F,
                }),
            },
            Configuration::MFS => DestructuredConfiguration {
                plexity: Plexity::M,
                similarity_and_separability: Some(SimilarityAndSeparability {
                    similarity: Similarity::F,
                    separability: Separability::S,
                }),
            },
            Configuration::MFC => DestructuredConfiguration {
                plexity: Plexity::M,
                similarity_and_separability: Some(SimilarityAndSeparability {
                    similarity: Similarity::F,
                    separability: Separability::C,
                }),
            },
            Configuration::MFF => DestructuredConfiguration {
                plexity: Plexity::M,
                similarity_and_separability: Some(SimilarityAndSeparability {
                    similarity: Similarity::F,
                    separability: Separability::F,
                }),
            },
            Configuration::DPX => DestructuredConfiguration {
                plexity: Plexity::D,
                similarity_and_separability: None,
            },
            Configuration::DSS => DestructuredConfiguration {
                plexity: Plexity::D,
                similarity_and_separability: Some(SimilarityAndSeparability {
                    similarity: Similarity::S,
                    separability: Separability::S,
                }),
            },
            Configuration::DSC => DestructuredConfiguration {
                plexity: Plexity::D,
                similarity_and_separability: Some(SimilarityAndSeparability {
                    similarity: Similarity::S,
                    separability: Separability::C,
                }),
            },
            Configuration::DSF => DestructuredConfiguration {
                plexity: Plexity::D,
                similarity_and_separability: Some(SimilarityAndSeparability {
                    similarity: Similarity::S,
                    separability: Separability::F,
                }),
            },
            Configuration::DDS => DestructuredConfiguration {
                plexity: Plexity::D,
                similarity_and_separability: Some(SimilarityAndSeparability {
                    similarity: Similarity::D,
                    separability: Separability::S,
                }),
            },
            Configuration::DDC => DestructuredConfiguration {
                plexity: Plexity::D,
                similarity_and_separability: Some(SimilarityAndSeparability {
                    similarity: Similarity::D,
                    separability: Separability::C,
                }),
            },
            Configuration::DDF => DestructuredConfiguration {
                plexity: Plexity::D,
                similarity_and_separability: Some(SimilarityAndSeparability {
                    similarity: Similarity::D,
                    separability: Separability::F,
                }),
            },
            Configuration::DFS => DestructuredConfiguration {
                plexity: Plexity::D,
                similarity_and_separability: Some(SimilarityAndSeparability {
                    similarity: Similarity::F,
                    separability: Separability::S,
                }),
            },
            Configuration::DFC => DestructuredConfiguration {
                plexity: Plexity::D,
                similarity_and_separability: Some(SimilarityAndSeparability {
                    similarity: Similarity::F,
                    separability: Separability::C,
                }),
            },
            Configuration::DFF => DestructuredConfiguration {
                plexity: Plexity::D,
                similarity_and_separability: Some(SimilarityAndSeparability {
                    similarity: Similarity::F,
                    separability: Separability::F,
                }),
            },
        }
    }
}

impl From<Configuration> for DestructuredConfiguration {
    fn from(value: Configuration) -> Self {
        value.destructure()
    }
}

impl DestructuredConfiguration {
    /// Gets the [`Plexity`] of this configuration.
    pub const fn plexity(self) -> Plexity {
        self.plexity
    }

    /// Gets the [`Similarity`] of this configuration.
    pub const fn similarity(self) -> Option<Similarity> {
        match self.similarity_and_separability {
            Some(SimilarityAndSeparability { similarity, .. }) => Some(similarity),
            None => None,
        }
    }

    /// Gets the [`Separability`] of this configuration.
    pub const fn separability(self) -> Option<Separability> {
        match self.similarity_and_separability {
            Some(SimilarityAndSeparability { separability, .. }) => Some(separability),
            None => None,
        }
    }

    /// Gets the [`SimilarityAndSeparability`] of this configuration.
    pub const fn similarity_and_separability(self) -> Option<SimilarityAndSeparability> {
        self.similarity_and_separability
    }

    /// Restructres into a [`Configuration`].
    pub const fn restructure(self) -> Configuration {
        match self.plexity {
            Plexity::M => match self.similarity_and_separability {
                None => Configuration::UPX,
                Some(SimilarityAndSeparability {
                    similarity,
                    separability,
                }) => match similarity {
                    Similarity::S => match separability {
                        Separability::S => Configuration::MSS,
                        Separability::C => Configuration::MSC,
                        Separability::F => Configuration::MSF,
                    },
                    Similarity::D => match separability {
                        Separability::S => Configuration::MDS,
                        Separability::C => Configuration::MDC,
                        Separability::F => Configuration::MDF,
                    },
                    Similarity::F => match separability {
                        Separability::S => Configuration::MFS,
                        Separability::C => Configuration::MFC,
                        Separability::F => Configuration::MFF,
                    },
                },
            },

            Plexity::D => match self.similarity_and_separability {
                None => Configuration::DPX,
                Some(SimilarityAndSeparability {
                    similarity,
                    separability,
                }) => match similarity {
                    Similarity::S => match separability {
                        Separability::S => Configuration::DSS,
                        Separability::C => Configuration::DSC,
                        Separability::F => Configuration::DSF,
                    },
                    Similarity::D => match separability {
                        Separability::S => Configuration::DDS,
                        Separability::C => Configuration::DDC,
                        Separability::F => Configuration::DDF,
                    },
                    Similarity::F => match separability {
                        Separability::S => Configuration::DFS,
                        Separability::C => Configuration::DFC,
                        Separability::F => Configuration::DFF,
                    },
                },
            },
        }
    }
}

impl From<DestructuredConfiguration> for Configuration {
    fn from(value: DestructuredConfiguration) -> Self {
        value.restructure()
    }
}

impl TryAsSpecific<ReferentialAffixPerspective> for Perspective {
    fn try_as_specific(self) -> Option<ReferentialAffixPerspective> {
        match self {
            Perspective::M => Some(ReferentialAffixPerspective::M),
            Perspective::G => Some(ReferentialAffixPerspective::G),
            Perspective::N => Some(ReferentialAffixPerspective::N),
            Perspective::A => None,
        }
    }
}

impl AsGeneral for ReferentialAffixPerspective {
    type Output = Perspective;

    fn as_general(self) -> Self::Output {
        match self {
            ReferentialAffixPerspective::M => Perspective::M,
            ReferentialAffixPerspective::G => Perspective::G,
            ReferentialAffixPerspective::N => Perspective::N,
        }
    }
}

impl From<ReferentialAffixPerspective> for Perspective {
    fn from(value: ReferentialAffixPerspective) -> Self {
        value.as_general()
    }
}

impl AsGeneral for Valence {
    type Output = Vn;

    fn as_general(self) -> Self::Output {
        Vn::Valence(self)
    }
}

impl From<Valence> for Vn {
    fn from(value: Valence) -> Self {
        Vn::Valence(value)
    }
}

impl AsGeneral for Phase {
    type Output = Vn;

    fn as_general(self) -> Self::Output {
        Vn::Phase(self)
    }
}

impl From<Phase> for Vn {
    fn from(value: Phase) -> Self {
        Vn::Phase(value)
    }
}

impl AsGeneral for Effect {
    type Output = Vn;

    fn as_general(self) -> Self::Output {
        Vn::Effect(self)
    }
}

impl From<Effect> for Vn {
    fn from(value: Effect) -> Self {
        Vn::Effect(value)
    }
}

impl AsGeneral for Level {
    type Output = Vn;

    fn as_general(self) -> Self::Output {
        Vn::Level(self)
    }
}

impl From<Level> for Vn {
    fn from(value: Level) -> Self {
        Vn::Level(value)
    }
}

impl AsGeneral for Aspect {
    type Output = Vn;

    fn as_general(self) -> Self::Output {
        Vn::Aspect(self)
    }
}

impl From<Aspect> for Vn {
    fn from(value: Aspect) -> Self {
        Vn::Aspect(value)
    }
}

impl Vn {
    /// Returns `true` if this [`Vn`] contains a [`Valence`], otherwise returns `false`.
    pub const fn is_valence(self) -> bool {
        matches!(self, Self::Valence(_))
    }

    /// Returns `true` if this [`Vn`] contains a [`Phase`], otherwise returns `false`.
    pub const fn is_phase(self) -> bool {
        matches!(self, Self::Phase(_))
    }

    /// Returns `true` if this [`Vn`] contains a [`Effect`], otherwise returns `false`.
    pub const fn is_effect(self) -> bool {
        matches!(self, Self::Effect(_))
    }

    /// Returns `true` if this [`Vn`] contains a [`Level`], otherwise returns `false`.
    pub const fn is_level(self) -> bool {
        matches!(self, Self::Level(_))
    }

    /// Returns `true` if this [`Vn`] contains a [`Aspect`], otherwise returns `false`.
    pub const fn is_aspect(self) -> bool {
        matches!(self, Self::Aspect(_))
    }

    /// Returns [`Some`] if this [`Vn`] contains a [`Valence`], otherwise returns [`None`].
    pub const fn as_valence(self) -> Option<Valence> {
        match self {
            Self::Valence(value) => Some(value),
            _ => None,
        }
    }

    /// Returns [`Some`] if this [`Vn`] contains a [`Phase`], otherwise returns [`None`].
    pub const fn as_phase(self) -> Option<Phase> {
        match self {
            Self::Phase(value) => Some(value),
            _ => None,
        }
    }

    /// Returns [`Some`] if this [`Vn`] contains an [`Effect`], otherwise returns [`None`].
    pub const fn as_effect(self) -> Option<Effect> {
        match self {
            Self::Effect(value) => Some(value),
            _ => None,
        }
    }

    /// Returns [`Some`] if this [`Vn`] contains a [`Level`], otherwise returns [`None`].
    pub const fn as_level(self) -> Option<Level> {
        match self {
            Self::Level(value) => Some(value),
            _ => None,
        }
    }

    /// Returns [`Some`] if this [`Vn`] contains an [`Aspect`], otherwise returns [`None`].
    pub const fn as_aspect(self) -> Option<Aspect> {
        match self {
            Self::Aspect(value) => Some(value),
            _ => None,
        }
    }
}

impl TryAsSpecific<Valence> for Vn {
    fn try_as_specific(self) -> Option<Valence> {
        match self {
            Self::Valence(value) => Some(value),
            _ => None,
        }
    }
}

impl TryAsSpecific<Phase> for Vn {
    fn try_as_specific(self) -> Option<Phase> {
        match self {
            Self::Phase(value) => Some(value),
            _ => None,
        }
    }
}

impl TryAsSpecific<Effect> for Vn {
    fn try_as_specific(self) -> Option<Effect> {
        match self {
            Self::Effect(value) => Some(value),
            _ => None,
        }
    }
}

impl TryAsSpecific<Level> for Vn {
    fn try_as_specific(self) -> Option<Level> {
        match self {
            Self::Level(value) => Some(value),
            _ => None,
        }
    }
}

impl TryAsSpecific<Aspect> for Vn {
    fn try_as_specific(self) -> Option<Aspect> {
        match self {
            Self::Aspect(value) => Some(value),
            _ => None,
        }
    }
}

impl Illocution {
    /// Converts `self` into an [`IllocutionOrValidation`], or returns [`Err`] if it isn't possible.
    pub const fn as_ivl(self) -> Option<IllocutionOrValidation> {
        match self {
            Self::ASR => None,
            Self::DIR => Some(IllocutionOrValidation::DIR),
            Self::DEC => Some(IllocutionOrValidation::DEC),
            Self::IRG => Some(IllocutionOrValidation::IRG),
            Self::VER => Some(IllocutionOrValidation::VER),
            Self::ADM => Some(IllocutionOrValidation::ADM),
            Self::POT => Some(IllocutionOrValidation::POT),
            Self::HOR => Some(IllocutionOrValidation::HOR),
            Self::CNJ => Some(IllocutionOrValidation::CNJ),
        }
    }
}

impl TryAsGeneral for Illocution {
    type Output = IllocutionOrValidation;

    fn try_as_general(self) -> Option<Self::Output> {
        self.as_ivl()
    }
}

impl Validation {
    /// Converts `self` into an [`IllocutionOrValidation`].
    pub const fn as_ivl(self) -> IllocutionOrValidation {
        match self {
            Self::OBS => IllocutionOrValidation::OBS,
            Self::REC => IllocutionOrValidation::REC,
            Self::PUP => IllocutionOrValidation::PUP,
            Self::RPR => IllocutionOrValidation::RPR,
            Self::USP => IllocutionOrValidation::USP,
            Self::IMA => IllocutionOrValidation::IMA,
            Self::CVN => IllocutionOrValidation::CVN,
            Self::ITU => IllocutionOrValidation::ITU,
            Self::INF => IllocutionOrValidation::INF,
        }
    }
}

impl AsGeneral for Validation {
    type Output = IllocutionOrValidation;

    fn as_general(self) -> Self::Output {
        self.as_ivl()
    }
}

impl From<Validation> for IllocutionOrValidation {
    fn from(value: Validation) -> Self {
        value.as_ivl()
    }
}

impl IllocutionOrValidation {
    /// Converts `self` into an [`Illocution`], returning [`None`] if it contains a [`Validation`].
    pub const fn as_illocution(self) -> Option<Illocution> {
        match self {
            Self::DIR => Some(Illocution::DIR),
            Self::DEC => Some(Illocution::DEC),
            Self::IRG => Some(Illocution::IRG),
            Self::VER => Some(Illocution::VER),
            Self::ADM => Some(Illocution::ADM),
            Self::POT => Some(Illocution::POT),
            Self::HOR => Some(Illocution::HOR),
            Self::CNJ => Some(Illocution::CNJ),
            _ => None,
        }
    }

    /// Converts `self` into a [`Validation`], returning [`None`] if it contains an [`Illocution`].
    pub const fn as_validation(self) -> Option<Validation> {
        match self {
            Self::OBS => Some(Validation::OBS),
            Self::REC => Some(Validation::REC),
            Self::PUP => Some(Validation::PUP),
            Self::RPR => Some(Validation::RPR),
            Self::USP => Some(Validation::USP),
            Self::IMA => Some(Validation::IMA),
            Self::CVN => Some(Validation::CVN),
            Self::ITU => Some(Validation::ITU),
            Self::INF => Some(Validation::INF),
            _ => None,
        }
    }
}

impl TryAsSpecific<Illocution> for IllocutionOrValidation {
    fn try_as_specific(self) -> Option<Illocution> {
        self.as_illocution()
    }
}

impl TryAsSpecific<Validation> for IllocutionOrValidation {
    fn try_as_specific(self) -> Option<Validation> {
        self.as_validation()
    }
}

impl AsGeneral for ThematicCase {
    type Output = Case;

    fn as_general(self) -> Case {
        match self {
            Self::THM => Case::THM,
            Self::INS => Case::INS,
            Self::ABS => Case::ABS,
            Self::AFF => Case::AFF,
            Self::STM => Case::STM,
            Self::EFF => Case::EFF,
            Self::ERG => Case::ERG,
            Self::DAT => Case::DAT,
            Self::IND => Case::IND,
        }
    }
}

impl From<ThematicCase> for Case {
    fn from(value: ThematicCase) -> Self {
        value.as_general()
    }
}

impl AsGeneral for AppositiveCase {
    type Output = Case;

    fn as_general(self) -> Self::Output {
        match self {
            Self::POS => Case::POS,
            Self::PRP => Case::PRP,
            Self::GEN => Case::GEN,
            Self::ATT => Case::ATT,
            Self::PDC => Case::PDC,
            Self::ITP => Case::ITP,
            Self::OGN => Case::OGN,
            Self::IDP => Case::IDP,
            Self::PAR => Case::PAR,
        }
    }
}

impl From<AppositiveCase> for Case {
    fn from(value: AppositiveCase) -> Self {
        value.as_general()
    }
}

impl Case {
    /// Converts `self` into a [`ThematicCase`], returning [`None`] if it isn't possible
    pub const fn as_thematic(self) -> Option<ThematicCase> {
        match self {
            Self::THM => Some(ThematicCase::THM),
            Self::INS => Some(ThematicCase::INS),
            Self::ABS => Some(ThematicCase::ABS),
            Self::AFF => Some(ThematicCase::AFF),
            Self::STM => Some(ThematicCase::STM),
            Self::EFF => Some(ThematicCase::EFF),
            Self::ERG => Some(ThematicCase::ERG),
            Self::DAT => Some(ThematicCase::DAT),
            Self::IND => Some(ThematicCase::IND),
            _ => None,
        }
    }

    /// Converts `self` into an [`AppositiveCase`], returning [`None`] if it isn't possible
    pub const fn as_appositive(self) -> Option<AppositiveCase> {
        match self {
            Self::POS => Some(AppositiveCase::POS),
            Self::PRP => Some(AppositiveCase::PRP),
            Self::GEN => Some(AppositiveCase::GEN),
            Self::ATT => Some(AppositiveCase::ATT),
            Self::PDC => Some(AppositiveCase::PDC),
            Self::ITP => Some(AppositiveCase::ITP),
            Self::OGN => Some(AppositiveCase::OGN),
            Self::IDP => Some(AppositiveCase::IDP),
            Self::PAR => Some(AppositiveCase::PAR),
            _ => None,
        }
    }
}

impl TryAsSpecific<ThematicCase> for Case {
    fn try_as_specific(self) -> Option<ThematicCase> {
        self.as_thematic()
    }
}

impl TryAsSpecific<AppositiveCase> for Case {
    fn try_as_specific(self) -> Option<AppositiveCase> {
        self.as_appositive()
    }
}

impl AsGeneral for NonDefaultMood {
    type Output = Mood;

    fn as_general(self) -> Self::Output {
        match self {
            Self::SUB => Mood::SUB,
            Self::ASM => Mood::ASM,
            Self::SPC => Mood::SPC,
            Self::COU => Mood::COU,
            Self::HYP => Mood::HYP,
        }
    }
}

impl From<NonDefaultMood> for Mood {
    fn from(value: NonDefaultMood) -> Self {
        value.as_general()
    }
}

impl AsGeneral for NonDefaultCaseScope {
    type Output = CaseScope;

    fn as_general(self) -> CaseScope {
        match self {
            Self::CCA => CaseScope::CCA,
            Self::CCS => CaseScope::CCS,
            Self::CCQ => CaseScope::CCQ,
            Self::CCP => CaseScope::CCP,
            Self::CCV => CaseScope::CCV,
        }
    }
}

impl From<NonDefaultCaseScope> for CaseScope {
    fn from(value: NonDefaultCaseScope) -> Self {
        value.as_general()
    }
}

impl TryAsSpecific<NonDefaultMood> for Mood {
    fn try_as_specific(self) -> Option<NonDefaultMood> {
        match self {
            Self::FAC => None,
            Self::SUB => Some(NonDefaultMood::SUB),
            Self::ASM => Some(NonDefaultMood::ASM),
            Self::SPC => Some(NonDefaultMood::SPC),
            Self::COU => Some(NonDefaultMood::COU),
            Self::HYP => Some(NonDefaultMood::HYP),
        }
    }
}

impl TryAsSpecific<NonDefaultCaseScope> for CaseScope {
    fn try_as_specific(self) -> Option<NonDefaultCaseScope> {
        match self {
            Self::CCN => None,
            Self::CCA => Some(NonDefaultCaseScope::CCA),
            Self::CCS => Some(NonDefaultCaseScope::CCS),
            Self::CCQ => Some(NonDefaultCaseScope::CCQ),
            Self::CCP => Some(NonDefaultCaseScope::CCP),
            Self::CCV => Some(NonDefaultCaseScope::CCV),
        }
    }
}

impl AsGeneral for CaShortcut {
    type Output = Ca;

    fn as_general(self) -> Ca {
        match self {
            Self::Default => Ca {
                affiliation: Affiliation::CSL,
                configuration: Configuration::UPX,
                extension: Extension::DEL,
                perspective: Perspective::M,
                essence: Essence::NRM,
            },

            Self::PRX => Ca {
                affiliation: Affiliation::CSL,
                configuration: Configuration::UPX,
                extension: Extension::PRX,
                perspective: Perspective::M,
                essence: Essence::NRM,
            },

            Self::G => Ca {
                affiliation: Affiliation::CSL,
                configuration: Configuration::UPX,
                extension: Extension::DEL,
                perspective: Perspective::G,
                essence: Essence::NRM,
            },

            Self::RPV => Ca {
                affiliation: Affiliation::CSL,
                configuration: Configuration::UPX,
                extension: Extension::DEL,
                perspective: Perspective::M,
                essence: Essence::RPV,
            },

            Self::N => Ca {
                affiliation: Affiliation::CSL,
                configuration: Configuration::UPX,
                extension: Extension::DEL,
                perspective: Perspective::N,
                essence: Essence::NRM,
            },

            Self::A => Ca {
                affiliation: Affiliation::CSL,
                configuration: Configuration::UPX,
                extension: Extension::DEL,
                perspective: Perspective::A,
                essence: Essence::NRM,
            },

            Self::G_RPV => Ca {
                affiliation: Affiliation::CSL,
                configuration: Configuration::UPX,
                extension: Extension::DEL,
                perspective: Perspective::G,
                essence: Essence::RPV,
            },

            Self::PRX_RPV => Ca {
                affiliation: Affiliation::CSL,
                configuration: Configuration::UPX,
                extension: Extension::PRX,
                perspective: Perspective::M,
                essence: Essence::RPV,
            },
        }
    }
}

impl From<CaShortcut> for Ca {
    fn from(value: CaShortcut) -> Self {
        value.as_general()
    }
}

impl TryAsSpecific<CaShortcut> for Ca {
    fn try_as_specific(self) -> Option<CaShortcut> {
        match self {
            Ca {
                affiliation: Affiliation::CSL,
                configuration: Configuration::UPX,
                extension: Extension::DEL,
                perspective: Perspective::M,
                essence: Essence::NRM,
            } => Some(CaShortcut::Default),

            Ca {
                affiliation: Affiliation::CSL,
                configuration: Configuration::UPX,
                extension: Extension::PRX,
                perspective: Perspective::M,
                essence: Essence::NRM,
            } => Some(CaShortcut::PRX),

            Ca {
                affiliation: Affiliation::CSL,
                configuration: Configuration::UPX,
                extension: Extension::DEL,
                perspective: Perspective::G,
                essence: Essence::NRM,
            } => Some(CaShortcut::G),

            Ca {
                affiliation: Affiliation::CSL,
                configuration: Configuration::UPX,
                extension: Extension::DEL,
                perspective: Perspective::M,
                essence: Essence::RPV,
            } => Some(CaShortcut::RPV),

            Ca {
                affiliation: Affiliation::CSL,
                configuration: Configuration::UPX,
                extension: Extension::DEL,
                perspective: Perspective::N,
                essence: Essence::NRM,
            } => Some(CaShortcut::N),

            Ca {
                affiliation: Affiliation::CSL,
                configuration: Configuration::UPX,
                extension: Extension::DEL,
                perspective: Perspective::A,
                essence: Essence::NRM,
            } => Some(CaShortcut::A),

            Ca {
                affiliation: Affiliation::CSL,
                configuration: Configuration::UPX,
                extension: Extension::DEL,
                perspective: Perspective::G,
                essence: Essence::RPV,
            } => Some(CaShortcut::G_RPV),

            Ca {
                affiliation: Affiliation::CSL,
                configuration: Configuration::UPX,
                extension: Extension::PRX,
                perspective: Perspective::M,
                essence: Essence::RPV,
            } => Some(CaShortcut::PRX_RPV),

            _ => None,
        }
    }
}
