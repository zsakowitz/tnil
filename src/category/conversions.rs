//! This module provides conversions between different categories and complexes defined in
//! crate::category. All conversions are provided both as inherent `const fn`s, and lossless
//! conversions are also implemented as `From` and `Into` implementations.

use super::{
    Affiliation, AffixDegree, AffixType, AppositiveCase, ArbitraryMoodOrCaseScope, Aspect, Bias,
    Ca, Case, CaseScope, Configuration, DatalessRelation, DestructuredConfiguration, Effect,
    Essence, Extension, Illocution, IllocutionOrValidation, Level, Mood, MoodOrCaseScope,
    NominalMode, NonAspectualVn, NonDefaultCaseScope, NonDefaultMood, NormalCaShortcut,
    Perspective, Phase, Plexity, ReferentialAffixPerspective, ReferentialCaShortcut, Sanction,
    Separability, Similarity, SimilarityAndSeparability, ThematicCase, Valence, Validation, Vn,
    VowelFormDegree, VowelFormSequence,
};
use crate::{
    ca,
    romanize::{stream::ParseError, token::VowelForm},
    specificity::{AsGeneral, AsSpecific, TryAsGeneral, TryAsSpecific},
};
use std::{
    error::Error,
    fmt::{self, Display},
    str::FromStr,
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

impl AsGeneral<Perspective> for ReferentialAffixPerspective {
    fn as_general(self) -> Perspective {
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

impl AsGeneral<Vn> for Valence {
    fn as_general(self) -> Vn {
        Vn::Valence(self)
    }
}

impl AsGeneral<NonAspectualVn> for Valence {
    fn as_general(self) -> NonAspectualVn {
        NonAspectualVn::Valence(self)
    }
}

impl From<Valence> for Vn {
    fn from(value: Valence) -> Self {
        Vn::Valence(value)
    }
}

impl From<Valence> for NonAspectualVn {
    fn from(value: Valence) -> Self {
        NonAspectualVn::Valence(value)
    }
}

impl AsGeneral<Vn> for Phase {
    fn as_general(self) -> Vn {
        Vn::Phase(self)
    }
}

impl AsGeneral<NonAspectualVn> for Phase {
    fn as_general(self) -> NonAspectualVn {
        NonAspectualVn::Phase(self)
    }
}

impl From<Phase> for Vn {
    fn from(value: Phase) -> Self {
        Vn::Phase(value)
    }
}

impl From<Phase> for NonAspectualVn {
    fn from(value: Phase) -> Self {
        NonAspectualVn::Phase(value)
    }
}

impl AsGeneral<Vn> for Effect {
    fn as_general(self) -> Vn {
        Vn::Effect(self)
    }
}

impl AsGeneral<NonAspectualVn> for Effect {
    fn as_general(self) -> NonAspectualVn {
        NonAspectualVn::Effect(self)
    }
}

impl From<Effect> for Vn {
    fn from(value: Effect) -> Self {
        Vn::Effect(value)
    }
}

impl From<Effect> for NonAspectualVn {
    fn from(value: Effect) -> Self {
        NonAspectualVn::Effect(value)
    }
}

impl AsGeneral<Vn> for Level {
    fn as_general(self) -> Vn {
        Vn::Level(self)
    }
}

impl AsGeneral<NonAspectualVn> for Level {
    fn as_general(self) -> NonAspectualVn {
        NonAspectualVn::Level(self)
    }
}

impl From<Level> for Vn {
    fn from(value: Level) -> Self {
        Vn::Level(value)
    }
}

impl From<Level> for NonAspectualVn {
    fn from(value: Level) -> Self {
        NonAspectualVn::Level(value)
    }
}

impl AsGeneral<Vn> for Aspect {
    fn as_general(self) -> Vn {
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

impl NonAspectualVn {
    /// Returns `true` if this [`NonAspectualVn`] contains a [`Valence`], otherwise returns `false`.
    pub const fn is_valence(self) -> bool {
        matches!(self, Self::Valence(_))
    }

    /// Returns `true` if this [`NonAspectualVn`] contains a [`Phase`], otherwise returns `false`.
    pub const fn is_phase(self) -> bool {
        matches!(self, Self::Phase(_))
    }

    /// Returns `true` if this [`NonAspectualVn`] contains a [`Effect`], otherwise returns `false`.
    pub const fn is_effect(self) -> bool {
        matches!(self, Self::Effect(_))
    }

    /// Returns `true` if this [`NonAspectualVn`] contains a [`Level`], otherwise returns `false`.
    pub const fn is_level(self) -> bool {
        matches!(self, Self::Level(_))
    }

    /// Returns [`Some`] if this [`NonAspectualVn`] contains a [`Valence`], otherwise returns
    /// [`None`].
    pub const fn as_valence(self) -> Option<Valence> {
        match self {
            Self::Valence(value) => Some(value),
            _ => None,
        }
    }

    /// Returns [`Some`] if this [`NonAspectualVn`] contains a [`Phase`], otherwise returns
    /// [`None`].
    pub const fn as_phase(self) -> Option<Phase> {
        match self {
            Self::Phase(value) => Some(value),
            _ => None,
        }
    }

    /// Returns [`Some`] if this [`NonAspectualVn`] contains an [`Effect`], otherwise returns
    /// [`None`].
    pub const fn as_effect(self) -> Option<Effect> {
        match self {
            Self::Effect(value) => Some(value),
            _ => None,
        }
    }

    /// Returns [`Some`] if this [`NonAspectualVn`] contains a [`Level`], otherwise returns
    /// [`None`].
    pub const fn as_level(self) -> Option<Level> {
        match self {
            Self::Level(value) => Some(value),
            _ => None,
        }
    }
}

impl TryAsSpecific<Valence> for NonAspectualVn {
    fn try_as_specific(self) -> Option<Valence> {
        match self {
            Self::Valence(value) => Some(value),
            _ => None,
        }
    }
}

impl TryAsSpecific<Phase> for NonAspectualVn {
    fn try_as_specific(self) -> Option<Phase> {
        match self {
            Self::Phase(value) => Some(value),
            _ => None,
        }
    }
}

impl TryAsSpecific<Effect> for NonAspectualVn {
    fn try_as_specific(self) -> Option<Effect> {
        match self {
            Self::Effect(value) => Some(value),
            _ => None,
        }
    }
}

impl TryAsSpecific<Level> for NonAspectualVn {
    fn try_as_specific(self) -> Option<Level> {
        match self {
            Self::Level(value) => Some(value),
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

impl TryAsGeneral<IllocutionOrValidation> for Illocution {
    fn try_as_general(self) -> Option<IllocutionOrValidation> {
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

impl AsGeneral<IllocutionOrValidation> for Validation {
    fn as_general(self) -> IllocutionOrValidation {
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

impl AsGeneral<Case> for ThematicCase {
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

impl AsGeneral<Case> for AppositiveCase {
    fn as_general(self) -> Case {
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

impl AsGeneral<Mood> for NonDefaultMood {
    fn as_general(self) -> Mood {
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

impl AsGeneral<MoodOrCaseScope> for NonDefaultMood {
    fn as_general(self) -> MoodOrCaseScope {
        let this: Mood = self.as_general();
        this.as_general()
    }
}

impl TryAsSpecific<NonDefaultMood> for MoodOrCaseScope {
    fn try_as_specific(self) -> Option<NonDefaultMood> {
        let this: Mood = self.try_as_specific()?;
        this.try_as_specific()
    }
}

impl From<NonDefaultMood> for MoodOrCaseScope {
    fn from(value: NonDefaultMood) -> Self {
        value.as_general()
    }
}

impl AsGeneral<MoodOrCaseScope> for NonDefaultCaseScope {
    fn as_general(self) -> MoodOrCaseScope {
        let this: CaseScope = self.as_general();
        this.as_general()
    }
}

impl TryAsSpecific<NonDefaultCaseScope> for MoodOrCaseScope {
    fn try_as_specific(self) -> Option<NonDefaultCaseScope> {
        let this: CaseScope = self.try_as_specific()?;
        this.try_as_specific()
    }
}

impl From<NonDefaultCaseScope> for MoodOrCaseScope {
    fn from(value: NonDefaultCaseScope) -> Self {
        value.as_general()
    }
}

impl AsGeneral<CaseScope> for NonDefaultCaseScope {
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

impl AsGeneral<Ca> for NormalCaShortcut {
    fn as_general(self) -> Ca {
        match self {
            Self::Default => ca!(),
            Self::PRX => ca!(PRX),
            Self::G => ca!(G),
            Self::RPV => ca!(RPV),
            Self::N => ca!(N),
            Self::A => ca!(A),
            Self::G_RPV => ca!(G, RPV),
            Self::PRX_RPV => ca!(PRX, RPV),
        }
    }
}

impl From<NormalCaShortcut> for Ca {
    fn from(value: NormalCaShortcut) -> Self {
        value.as_general()
    }
}

impl TryAsSpecific<NormalCaShortcut> for Ca {
    fn try_as_specific(self) -> Option<NormalCaShortcut> {
        match self {
            Ca {
                affiliation: Affiliation::CSL,
                configuration: Configuration::UPX,
                extension: Extension::DEL,
                perspective: Perspective::M,
                essence: Essence::NRM,
            } => Some(NormalCaShortcut::Default),

            Ca {
                affiliation: Affiliation::CSL,
                configuration: Configuration::UPX,
                extension: Extension::PRX,
                perspective: Perspective::M,
                essence: Essence::NRM,
            } => Some(NormalCaShortcut::PRX),

            Ca {
                affiliation: Affiliation::CSL,
                configuration: Configuration::UPX,
                extension: Extension::DEL,
                perspective: Perspective::G,
                essence: Essence::NRM,
            } => Some(NormalCaShortcut::G),

            Ca {
                affiliation: Affiliation::CSL,
                configuration: Configuration::UPX,
                extension: Extension::DEL,
                perspective: Perspective::M,
                essence: Essence::RPV,
            } => Some(NormalCaShortcut::RPV),

            Ca {
                affiliation: Affiliation::CSL,
                configuration: Configuration::UPX,
                extension: Extension::DEL,
                perspective: Perspective::N,
                essence: Essence::NRM,
            } => Some(NormalCaShortcut::N),

            Ca {
                affiliation: Affiliation::CSL,
                configuration: Configuration::UPX,
                extension: Extension::DEL,
                perspective: Perspective::A,
                essence: Essence::NRM,
            } => Some(NormalCaShortcut::A),

            Ca {
                affiliation: Affiliation::CSL,
                configuration: Configuration::UPX,
                extension: Extension::DEL,
                perspective: Perspective::G,
                essence: Essence::RPV,
            } => Some(NormalCaShortcut::G_RPV),

            Ca {
                affiliation: Affiliation::CSL,
                configuration: Configuration::UPX,
                extension: Extension::PRX,
                perspective: Perspective::M,
                essence: Essence::RPV,
            } => Some(NormalCaShortcut::PRX_RPV),

            _ => None,
        }
    }
}

impl AsGeneral<Ca> for ReferentialCaShortcut {
    fn as_general(self) -> Ca {
        match self {
            Self::Default => ca!(),
            Self::PRX => ca!(PRX),
        }
    }
}

impl From<ReferentialCaShortcut> for Ca {
    fn from(value: ReferentialCaShortcut) -> Self {
        value.as_general()
    }
}

impl TryAsSpecific<ReferentialCaShortcut> for Ca {
    fn try_as_specific(self) -> Option<ReferentialCaShortcut> {
        match self {
            Ca {
                affiliation: Affiliation::CSL,
                configuration: Configuration::UPX,
                extension: Extension::DEL,
                perspective: Perspective::M,
                essence: Essence::NRM,
            } => Some(ReferentialCaShortcut::Default),

            Ca {
                affiliation: Affiliation::CSL,
                configuration: Configuration::UPX,
                extension: Extension::PRX,
                perspective: Perspective::M,
                essence: Essence::NRM,
            } => Some(ReferentialCaShortcut::PRX),

            _ => None,
        }
    }
}

impl AsGeneral<MoodOrCaseScope> for Mood {
    fn as_general(self) -> MoodOrCaseScope {
        MoodOrCaseScope::Mood(self)
    }
}

impl From<Mood> for MoodOrCaseScope {
    fn from(value: Mood) -> Self {
        Self::Mood(value)
    }
}

impl TryAsSpecific<Mood> for MoodOrCaseScope {
    fn try_as_specific(self) -> Option<Mood> {
        match self {
            Self::Mood(value) => Some(value),
            _ => None,
        }
    }
}

impl AsGeneral<MoodOrCaseScope> for CaseScope {
    fn as_general(self) -> MoodOrCaseScope {
        MoodOrCaseScope::CaseScope(self)
    }
}

impl From<CaseScope> for MoodOrCaseScope {
    fn from(value: CaseScope) -> Self {
        Self::CaseScope(value)
    }
}

impl TryAsSpecific<CaseScope> for MoodOrCaseScope {
    fn try_as_specific(self) -> Option<CaseScope> {
        match self {
            Self::CaseScope(value) => Some(value),
            _ => None,
        }
    }
}

impl AsGeneral<ArbitraryMoodOrCaseScope> for Mood {
    fn as_general(self) -> ArbitraryMoodOrCaseScope {
        match self {
            Self::FAC => ArbitraryMoodOrCaseScope::FAC_CCN,
            Self::SUB => ArbitraryMoodOrCaseScope::SUB_CCA,
            Self::ASM => ArbitraryMoodOrCaseScope::ASM_CCS,
            Self::SPC => ArbitraryMoodOrCaseScope::SPC_CCQ,
            Self::COU => ArbitraryMoodOrCaseScope::COU_CCP,
            Self::HYP => ArbitraryMoodOrCaseScope::HYP_CCV,
        }
    }
}

impl From<Mood> for ArbitraryMoodOrCaseScope {
    fn from(value: Mood) -> Self {
        value.as_general()
    }
}

impl AsSpecific<Mood> for ArbitraryMoodOrCaseScope {
    fn as_specific(self) -> Mood {
        match self {
            Self::FAC_CCN => Mood::FAC,
            Self::SUB_CCA => Mood::SUB,
            Self::ASM_CCS => Mood::ASM,
            Self::SPC_CCQ => Mood::SPC,
            Self::COU_CCP => Mood::COU,
            Self::HYP_CCV => Mood::HYP,
        }
    }
}

impl AsGeneral<ArbitraryMoodOrCaseScope> for CaseScope {
    fn as_general(self) -> ArbitraryMoodOrCaseScope {
        match self {
            Self::CCN => ArbitraryMoodOrCaseScope::FAC_CCN,
            Self::CCA => ArbitraryMoodOrCaseScope::SUB_CCA,
            Self::CCS => ArbitraryMoodOrCaseScope::ASM_CCS,
            Self::CCQ => ArbitraryMoodOrCaseScope::SPC_CCQ,
            Self::CCP => ArbitraryMoodOrCaseScope::COU_CCP,
            Self::CCV => ArbitraryMoodOrCaseScope::HYP_CCV,
        }
    }
}

impl From<CaseScope> for ArbitraryMoodOrCaseScope {
    fn from(value: CaseScope) -> Self {
        value.as_general()
    }
}

impl AsSpecific<CaseScope> for ArbitraryMoodOrCaseScope {
    fn as_specific(self) -> CaseScope {
        match self {
            Self::FAC_CCN => CaseScope::CCN,
            Self::SUB_CCA => CaseScope::CCA,
            Self::ASM_CCS => CaseScope::CCS,
            Self::SPC_CCQ => CaseScope::CCQ,
            Self::COU_CCP => CaseScope::CCP,
            Self::HYP_CCV => CaseScope::CCV,
        }
    }
}

impl AsGeneral<ArbitraryMoodOrCaseScope> for NonDefaultMood {
    fn as_general(self) -> ArbitraryMoodOrCaseScope {
        match self {
            Self::SUB => ArbitraryMoodOrCaseScope::SUB_CCA,
            Self::ASM => ArbitraryMoodOrCaseScope::ASM_CCS,
            Self::SPC => ArbitraryMoodOrCaseScope::SPC_CCQ,
            Self::COU => ArbitraryMoodOrCaseScope::COU_CCP,
            Self::HYP => ArbitraryMoodOrCaseScope::HYP_CCV,
        }
    }
}

impl From<NonDefaultMood> for ArbitraryMoodOrCaseScope {
    fn from(value: NonDefaultMood) -> Self {
        value.as_general()
    }
}

impl TryAsSpecific<NonDefaultMood> for ArbitraryMoodOrCaseScope {
    fn try_as_specific(self) -> Option<NonDefaultMood> {
        match self {
            Self::FAC_CCN => None,
            Self::SUB_CCA => Some(NonDefaultMood::SUB),
            Self::ASM_CCS => Some(NonDefaultMood::ASM),
            Self::SPC_CCQ => Some(NonDefaultMood::SPC),
            Self::COU_CCP => Some(NonDefaultMood::COU),
            Self::HYP_CCV => Some(NonDefaultMood::HYP),
        }
    }
}

impl AsGeneral<ArbitraryMoodOrCaseScope> for NonDefaultCaseScope {
    fn as_general(self) -> ArbitraryMoodOrCaseScope {
        match self {
            Self::CCA => ArbitraryMoodOrCaseScope::SUB_CCA,
            Self::CCS => ArbitraryMoodOrCaseScope::ASM_CCS,
            Self::CCQ => ArbitraryMoodOrCaseScope::SPC_CCQ,
            Self::CCP => ArbitraryMoodOrCaseScope::COU_CCP,
            Self::CCV => ArbitraryMoodOrCaseScope::HYP_CCV,
        }
    }
}

impl From<NonDefaultCaseScope> for ArbitraryMoodOrCaseScope {
    fn from(value: NonDefaultCaseScope) -> Self {
        value.as_general()
    }
}

impl TryAsSpecific<NonDefaultCaseScope> for ArbitraryMoodOrCaseScope {
    fn try_as_specific(self) -> Option<NonDefaultCaseScope> {
        match self {
            Self::FAC_CCN => None,
            Self::SUB_CCA => Some(NonDefaultCaseScope::CCA),
            Self::ASM_CCS => Some(NonDefaultCaseScope::CCS),
            Self::SPC_CCQ => Some(NonDefaultCaseScope::CCQ),
            Self::COU_CCP => Some(NonDefaultCaseScope::CCP),
            Self::HYP_CCV => Some(NonDefaultCaseScope::CCV),
        }
    }
}

impl From<AffixDegree> for VowelFormDegree {
    fn from(value: AffixDegree) -> Self {
        match value {
            AffixDegree::D0 => VowelFormDegree::D0,
            AffixDegree::D1 => VowelFormDegree::D1,
            AffixDegree::D2 => VowelFormDegree::D2,
            AffixDegree::D3 => VowelFormDegree::D3,
            AffixDegree::D4 => VowelFormDegree::D4,
            AffixDegree::D5 => VowelFormDegree::D5,
            AffixDegree::D6 => VowelFormDegree::D6,
            AffixDegree::D7 => VowelFormDegree::D7,
            AffixDegree::D8 => VowelFormDegree::D8,
            AffixDegree::D9 => VowelFormDegree::D9,
        }
    }
}

impl From<VowelFormDegree> for AffixDegree {
    fn from(value: VowelFormDegree) -> Self {
        match value {
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
        }
    }
}

impl From<AffixType> for VowelFormSequence {
    fn from(value: AffixType) -> Self {
        match value {
            AffixType::T1 => VowelFormSequence::S1,
            AffixType::T2 => VowelFormSequence::S2,
            AffixType::T3 => VowelFormSequence::S3,
        }
    }
}

/// An error returned when a bias cannot be parsed because its consonant form is invalid.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParseBiasError;

impl Display for ParseBiasError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("expected a valid bias consonant form")
    }
}

impl Error for ParseBiasError {}

impl FromStr for Bias {
    type Err = ParseBiasError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lf" => Ok(Bias::ACC),
            "mçt" => Ok(Bias::ACH),
            "lļ" => Ok(Bias::ADS),
            "drr" => Ok(Bias::ANN),
            "lst" => Ok(Bias::ANP),
            "řs" => Ok(Bias::APB),
            "vvz" => Ok(Bias::APH),
            "xtļ" => Ok(Bias::ARB),
            "ňj" => Ok(Bias::ATE),
            "pļļ" => Ok(Bias::CMD),
            "rrj" => Ok(Bias::CNV),
            "ššč" => Ok(Bias::COI),
            "gžž" => Ok(Bias::CRP),
            "ňţ" => Ok(Bias::CRR),
            "kšš" => Ok(Bias::CTP),
            "gvv" => Ok(Bias::CTV),
            "gzj" => Ok(Bias::DCC),
            "žžg" => Ok(Bias::DEJ),
            "mřř" => Ok(Bias::DES),
            "cč" => Ok(Bias::DFD),
            "kff" => Ok(Bias::DIS),
            "ẓmm" => Ok(Bias::DLC),
            "řřx" => Ok(Bias::DOL),
            "ffx" => Ok(Bias::DPB),
            "pfc" => Ok(Bias::DRS),
            "mmf" => Ok(Bias::DUB),
            "gzz" => Ok(Bias::EUH),
            "vvt" => Ok(Bias::EUP),
            "kçç" => Ok(Bias::EXA),
            "rrs" => Ok(Bias::EXG),
            "lzp" => Ok(Bias::FOR),
            "žžj" => Ok(Bias::FSC),
            "mmh" => Ok(Bias::GRT),
            "pšš" => Ok(Bias::IDG),
            "vvr" => Ok(Bias::IFT),
            "vll" => Ok(Bias::IPL),
            "žžv" => Ok(Bias::IPT),
            "mmž" => Ok(Bias::IRO),
            "lçp" => Ok(Bias::ISP),
            "řřn" => Ok(Bias::IVD),
            "msk" => Ok(Bias::MAN),
            "pss" => Ok(Bias::MNF),
            "ççk" => Ok(Bias::OPT),
            "ksp" => Ok(Bias::PES),
            "mll" => Ok(Bias::PPT),
            "llh" => Ok(Bias::PPX),
            "sl" => Ok(Bias::PPV),
            "žžt" => Ok(Bias::PSC),
            "nnţ" => Ok(Bias::PSM),
            "kll" => Ok(Bias::RAC),
            "llm" => Ok(Bias::RFL),
            "msf" => Ok(Bias::RSG),
            "šštļ" => Ok(Bias::RPU),
            "mmļ" => Ok(Bias::RVL),
            "ļţ" => Ok(Bias::SAT),
            "ltç" => Ok(Bias::SGS),
            "rnž" => Ok(Bias::SKP),
            "ňňs" => Ok(Bias::SOL),
            "ļļč" => Ok(Bias::STU),
            "llč" => Ok(Bias::TRP),
            "ksk" => Ok(Bias::VEX),
            _ => Err(ParseBiasError),
        }
    }
}

impl Bias {
    /// Gets the Cb form of this [`Bias`].
    pub const fn as_cb(self) -> &'static str {
        match self {
            Bias::ACC => "lf",
            Bias::ACH => "mçt",
            Bias::ADS => "lļ",
            Bias::ANN => "drr",
            Bias::ANP => "lst",
            Bias::APB => "řs",
            Bias::APH => "vvz",
            Bias::ARB => "xtļ",
            Bias::ATE => "ňj",
            Bias::CMD => "pļļ",
            Bias::CNV => "rrj",
            Bias::COI => "ššč",
            Bias::CRP => "gžž",
            Bias::CRR => "ňţ",
            Bias::CTP => "kšš",
            Bias::CTV => "gvv",
            Bias::DCC => "gzj",
            Bias::DEJ => "žžg",
            Bias::DES => "mřř",
            Bias::DFD => "cč",
            Bias::DIS => "kff",
            Bias::DLC => "ẓmm",
            Bias::DOL => "řřx",
            Bias::DPB => "ffx",
            Bias::DRS => "pfc",
            Bias::DUB => "mmf",
            Bias::EUH => "gzz",
            Bias::EUP => "vvt",
            Bias::EXA => "kçç",
            Bias::EXG => "rrs",
            Bias::FOR => "lzp",
            Bias::FSC => "žžj",
            Bias::GRT => "mmh",
            Bias::IDG => "pšš",
            Bias::IFT => "vvr",
            Bias::IPL => "vll",
            Bias::IPT => "žžv",
            Bias::IRO => "mmž",
            Bias::ISP => "lçp",
            Bias::IVD => "řřn",
            Bias::MAN => "msk",
            Bias::MNF => "pss",
            Bias::OPT => "ççk",
            Bias::PES => "ksp",
            Bias::PPT => "mll",
            Bias::PPX => "llh",
            Bias::PPV => "sl",
            Bias::PSC => "žžt",
            Bias::PSM => "nnţ",
            Bias::RAC => "kll",
            Bias::RFL => "llm",
            Bias::RSG => "msf",
            Bias::RPU => "šštļ",
            Bias::RVL => "mmļ",
            Bias::SAT => "ļţ",
            Bias::SGS => "ltç",
            Bias::SKP => "rnž",
            Bias::SOL => "ňňs",
            Bias::STU => "ļļč",
            Bias::TRP => "llč",
            Bias::VEX => "ksk",
        }
    }
}

impl Case {
    /// Constructs a case from its associated variant, returning [`None`] upon failure.
    pub const fn from_variant(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::THM),
            1 => Some(Self::INS),
            2 => Some(Self::ABS),
            3 => Some(Self::AFF),
            4 => Some(Self::STM),
            5 => Some(Self::EFF),
            6 => Some(Self::ERG),
            7 => Some(Self::DAT),
            8 => Some(Self::IND),
            9 => Some(Self::POS),
            10 => Some(Self::PRP),
            11 => Some(Self::GEN),
            12 => Some(Self::ATT),
            13 => Some(Self::PDC),
            14 => Some(Self::ITP),
            15 => Some(Self::OGN),
            16 => Some(Self::IDP),
            17 => Some(Self::PAR),
            18 => Some(Self::APL),
            19 => Some(Self::PUR),
            20 => Some(Self::TRA),
            21 => Some(Self::DFR),
            22 => Some(Self::CRS),
            23 => Some(Self::TSP),
            24 => Some(Self::CMM),
            25 => Some(Self::CMP),
            26 => Some(Self::CSD),
            27 => Some(Self::FUN),
            28 => Some(Self::TFM),
            29 => Some(Self::CLA),
            30 => Some(Self::RSL),
            31 => Some(Self::CSM),
            32 => Some(Self::CON),
            33 => Some(Self::AVR),
            34 => Some(Self::CVS),
            35 => Some(Self::SIT),
            36 => Some(Self::PRN),
            37 => Some(Self::DSP),
            38 => Some(Self::COR),
            39 => Some(Self::CPS),
            40 => Some(Self::COM),
            41 => Some(Self::UTL),
            42 => Some(Self::PRD),
            44 => Some(Self::RLT),
            45 => Some(Self::ACT),
            46 => Some(Self::ASI),
            47 => Some(Self::ESS),
            48 => Some(Self::TRM),
            49 => Some(Self::SEL),
            50 => Some(Self::CFM),
            51 => Some(Self::DEP),
            53 => Some(Self::VOC),
            54 => Some(Self::LOC),
            55 => Some(Self::ATD),
            56 => Some(Self::ALL),
            57 => Some(Self::ABL),
            58 => Some(Self::ORI),
            59 => Some(Self::IRL),
            60 => Some(Self::INV),
            62 => Some(Self::NAV),
            63 => Some(Self::CNR),
            64 => Some(Self::ASS),
            65 => Some(Self::PER),
            66 => Some(Self::PRO),
            67 => Some(Self::PCV),
            68 => Some(Self::PCR),
            69 => Some(Self::ELP),
            71 => Some(Self::PLM),
            _ => None,
        }
    }

    /// Constructs a case from a [`VowelForm`].
    pub fn from_vc(vc: VowelForm) -> Result<Self, ParseError> {
        if vc.degree == VowelFormDegree::D0 {
            return Err(ParseError::ExpectedVc);
        }

        let shift = 36 * (vc.has_glottal_stop as u8);
        let sequence = 9 * (vc.sequence as u8);
        let degree = (vc.degree as u8) - 1;
        let value = shift + sequence + degree;
        Case::from_variant(value).ok_or_else(|| {
            eprintln!("hello");
            ParseError::ExpectedVc
        })
    }

    /// Converts into an [`IllocutionOrValidation`] with the same vowel form.
    pub const fn as_vk(self) -> Option<IllocutionOrValidation> {
        match self {
            Self::THM => Some(IllocutionOrValidation::OBS),
            Self::INS => Some(IllocutionOrValidation::REC),
            Self::ABS => Some(IllocutionOrValidation::PUP),
            Self::AFF => Some(IllocutionOrValidation::RPR),
            Self::STM => Some(IllocutionOrValidation::USP),
            Self::EFF => Some(IllocutionOrValidation::IMA),
            Self::ERG => Some(IllocutionOrValidation::CVN),
            Self::DAT => Some(IllocutionOrValidation::ITU),
            Self::IND => Some(IllocutionOrValidation::INF),
            Self::POS => Some(IllocutionOrValidation::DIR),
            Self::PRP => Some(IllocutionOrValidation::DEC),
            Self::GEN => Some(IllocutionOrValidation::IRG),
            Self::ATT => Some(IllocutionOrValidation::VER),
            Self::PDC => None,
            Self::ITP => Some(IllocutionOrValidation::ADM),
            Self::OGN => Some(IllocutionOrValidation::POT),
            Self::IDP => Some(IllocutionOrValidation::HOR),
            Self::PAR => Some(IllocutionOrValidation::CNJ),
            _ => None,
        }
    }
}

impl IllocutionOrValidation {
    /// Converts into a [`Case`] with the same vowel form.
    pub const fn as_vc(self) -> Case {
        match self {
            Self::OBS => Case::THM,
            Self::REC => Case::INS,
            Self::PUP => Case::ABS,
            Self::RPR => Case::AFF,
            Self::USP => Case::STM,
            Self::IMA => Case::EFF,
            Self::CVN => Case::ERG,
            Self::ITU => Case::DAT,
            Self::INF => Case::IND,
            Self::DIR => Case::POS,
            Self::DEC => Case::PRP,
            Self::IRG => Case::GEN,
            Self::VER => Case::ATT,
            Self::ADM => Case::ITP,
            Self::POT => Case::OGN,
            Self::HOR => Case::IDP,
            Self::CNJ => Case::PAR,
        }
    }
}

impl Aspect {
    /// Creates an [`Aspect`] from a vowel form.
    pub const fn from_vowel_form(vn: VowelForm) -> Option<Self> {
        use VowelFormDegree as D;
        use VowelFormSequence as S;

        match (vn.sequence, vn.degree) {
            (S::S1, D::D1) => Some(Aspect::RTR),
            (S::S1, D::D2) => Some(Aspect::PRS),
            (S::S1, D::D3) => Some(Aspect::HAB),
            (S::S1, D::D4) => Some(Aspect::PRG),
            (S::S1, D::D5) => Some(Aspect::IMM),
            (S::S1, D::D6) => Some(Aspect::PCS),
            (S::S1, D::D7) => Some(Aspect::REG),
            (S::S1, D::D8) => Some(Aspect::SMM),
            (S::S1, D::D9) => Some(Aspect::ATP),

            (S::S2, D::D1) => Some(Aspect::RSM),
            (S::S2, D::D2) => Some(Aspect::CSS),
            (S::S2, D::D3) => Some(Aspect::PAU),
            (S::S2, D::D4) => Some(Aspect::RGR),
            (S::S2, D::D5) => Some(Aspect::PCL),
            (S::S2, D::D6) => Some(Aspect::CNT),
            (S::S2, D::D7) => Some(Aspect::ICS),
            (S::S2, D::D8) => Some(Aspect::EXP),
            (S::S2, D::D9) => Some(Aspect::IRP),

            (S::S3, D::D1) => Some(Aspect::PMP),
            (S::S3, D::D2) => Some(Aspect::CLM),
            (S::S3, D::D3) => Some(Aspect::DLT),
            (S::S3, D::D4) => Some(Aspect::TMP),
            (S::S3, D::D5) => Some(Aspect::XPD),
            (S::S3, D::D6) => Some(Aspect::LIM),
            (S::S3, D::D7) => Some(Aspect::EPD),
            (S::S3, D::D8) => Some(Aspect::PTC),
            (S::S3, D::D9) => Some(Aspect::PPR),

            (S::S4, D::D1) => Some(Aspect::DCL),
            (S::S4, D::D2) => Some(Aspect::CCL),
            (S::S4, D::D3) => Some(Aspect::CUL),
            (S::S4, D::D4) => Some(Aspect::IMD),
            (S::S4, D::D5) => Some(Aspect::TRD),
            (S::S4, D::D6) => Some(Aspect::TNS),
            (S::S4, D::D7) => Some(Aspect::ITC),
            (S::S4, D::D8) => Some(Aspect::MTV),
            (S::S4, D::D9) => Some(Aspect::SQN),

            _ => None,
        }
    }
}

impl NonAspectualVn {
    /// Creates a [`NonAspectualVn`] from a vowel form.
    pub const fn from_vowel_form(vn: VowelForm) -> Option<Self> {
        use VowelFormDegree as D;
        use VowelFormSequence as S;

        match (vn.sequence, vn.degree) {
            (S::S1, D::D1) => Some(Self::Valence(Valence::MNO)),
            (S::S1, D::D2) => Some(Self::Valence(Valence::PRL)),
            (S::S1, D::D3) => Some(Self::Valence(Valence::CRO)),
            (S::S1, D::D4) => Some(Self::Valence(Valence::RCP)),
            (S::S1, D::D5) => Some(Self::Valence(Valence::CPL)),
            (S::S1, D::D6) => Some(Self::Valence(Valence::DUP)),
            (S::S1, D::D7) => Some(Self::Valence(Valence::DEM)),
            (S::S1, D::D8) => Some(Self::Valence(Valence::CNG)),
            (S::S1, D::D9) => Some(Self::Valence(Valence::PTI)),

            (S::S2, D::D1) => Some(Self::Phase(Phase::PUN)),
            (S::S2, D::D2) => Some(Self::Phase(Phase::ITR)),
            (S::S2, D::D3) => Some(Self::Phase(Phase::REP)),
            (S::S2, D::D4) => Some(Self::Phase(Phase::ITM)),
            (S::S2, D::D5) => Some(Self::Phase(Phase::RCT)),
            (S::S2, D::D6) => Some(Self::Phase(Phase::FRE)),
            (S::S2, D::D7) => Some(Self::Phase(Phase::FRG)),
            (S::S2, D::D8) => Some(Self::Phase(Phase::VAC)),
            (S::S2, D::D9) => Some(Self::Phase(Phase::FLC)),

            (S::S3, D::D1) => Some(Self::Effect(Effect::BEN1)),
            (S::S3, D::D2) => Some(Self::Effect(Effect::BEN2)),
            (S::S3, D::D3) => Some(Self::Effect(Effect::BEN3)),
            (S::S3, D::D4) => Some(Self::Effect(Effect::BENSELF)),
            (S::S3, D::D5) => Some(Self::Effect(Effect::UNK)),
            (S::S3, D::D6) => Some(Self::Effect(Effect::DETSELF)),
            (S::S3, D::D7) => Some(Self::Effect(Effect::DET3)),
            (S::S3, D::D8) => Some(Self::Effect(Effect::DET2)),
            (S::S3, D::D9) => Some(Self::Effect(Effect::DET1)),

            (S::S4, D::D1) => Some(Self::Level(Level::MIN)),
            (S::S4, D::D2) => Some(Self::Level(Level::SBE)),
            (S::S4, D::D3) => Some(Self::Level(Level::IFR)),
            (S::S4, D::D4) => Some(Self::Level(Level::DFC)),
            (S::S4, D::D5) => Some(Self::Level(Level::EQU)),
            (S::S4, D::D6) => Some(Self::Level(Level::SUR)),
            (S::S4, D::D7) => Some(Self::Level(Level::SPL)),
            (S::S4, D::D8) => Some(Self::Level(Level::SPQ)),
            (S::S4, D::D9) => Some(Self::Level(Level::MAX)),

            _ => None,
        }
    }
}

impl Vn {
    /// Creates a [`Vn`] from a vowel form.
    pub const fn from_vowel_form(vn: VowelForm, is_aspect: bool) -> Option<Self> {
        use VowelFormDegree as D;
        use VowelFormSequence as S;

        if is_aspect {
            match (vn.sequence, vn.degree) {
                (S::S1, D::D1) => Some(Self::Aspect(Aspect::RTR)),
                (S::S1, D::D2) => Some(Self::Aspect(Aspect::PRS)),
                (S::S1, D::D3) => Some(Self::Aspect(Aspect::HAB)),
                (S::S1, D::D4) => Some(Self::Aspect(Aspect::PRG)),
                (S::S1, D::D5) => Some(Self::Aspect(Aspect::IMM)),
                (S::S1, D::D6) => Some(Self::Aspect(Aspect::PCS)),
                (S::S1, D::D7) => Some(Self::Aspect(Aspect::REG)),
                (S::S1, D::D8) => Some(Self::Aspect(Aspect::SMM)),
                (S::S1, D::D9) => Some(Self::Aspect(Aspect::ATP)),

                (S::S2, D::D1) => Some(Self::Aspect(Aspect::RSM)),
                (S::S2, D::D2) => Some(Self::Aspect(Aspect::CSS)),
                (S::S2, D::D3) => Some(Self::Aspect(Aspect::PAU)),
                (S::S2, D::D4) => Some(Self::Aspect(Aspect::RGR)),
                (S::S2, D::D5) => Some(Self::Aspect(Aspect::PCL)),
                (S::S2, D::D6) => Some(Self::Aspect(Aspect::CNT)),
                (S::S2, D::D7) => Some(Self::Aspect(Aspect::ICS)),
                (S::S2, D::D8) => Some(Self::Aspect(Aspect::EXP)),
                (S::S2, D::D9) => Some(Self::Aspect(Aspect::IRP)),

                (S::S3, D::D1) => Some(Self::Aspect(Aspect::PMP)),
                (S::S3, D::D2) => Some(Self::Aspect(Aspect::CLM)),
                (S::S3, D::D3) => Some(Self::Aspect(Aspect::DLT)),
                (S::S3, D::D4) => Some(Self::Aspect(Aspect::TMP)),
                (S::S3, D::D5) => Some(Self::Aspect(Aspect::XPD)),
                (S::S3, D::D6) => Some(Self::Aspect(Aspect::LIM)),
                (S::S3, D::D7) => Some(Self::Aspect(Aspect::EPD)),
                (S::S3, D::D8) => Some(Self::Aspect(Aspect::PTC)),
                (S::S3, D::D9) => Some(Self::Aspect(Aspect::PPR)),

                (S::S4, D::D1) => Some(Self::Aspect(Aspect::DCL)),
                (S::S4, D::D2) => Some(Self::Aspect(Aspect::CCL)),
                (S::S4, D::D3) => Some(Self::Aspect(Aspect::CUL)),
                (S::S4, D::D4) => Some(Self::Aspect(Aspect::IMD)),
                (S::S4, D::D5) => Some(Self::Aspect(Aspect::TRD)),
                (S::S4, D::D6) => Some(Self::Aspect(Aspect::TNS)),
                (S::S4, D::D7) => Some(Self::Aspect(Aspect::ITC)),
                (S::S4, D::D8) => Some(Self::Aspect(Aspect::MTV)),
                (S::S4, D::D9) => Some(Self::Aspect(Aspect::SQN)),

                _ => None,
            }
        } else {
            match (vn.sequence, vn.degree) {
                (S::S1, D::D1) => Some(Self::Valence(Valence::MNO)),
                (S::S1, D::D2) => Some(Self::Valence(Valence::PRL)),
                (S::S1, D::D3) => Some(Self::Valence(Valence::CRO)),
                (S::S1, D::D4) => Some(Self::Valence(Valence::RCP)),
                (S::S1, D::D5) => Some(Self::Valence(Valence::CPL)),
                (S::S1, D::D6) => Some(Self::Valence(Valence::DUP)),
                (S::S1, D::D7) => Some(Self::Valence(Valence::DEM)),
                (S::S1, D::D8) => Some(Self::Valence(Valence::CNG)),
                (S::S1, D::D9) => Some(Self::Valence(Valence::PTI)),

                (S::S2, D::D1) => Some(Self::Phase(Phase::PUN)),
                (S::S2, D::D2) => Some(Self::Phase(Phase::ITR)),
                (S::S2, D::D3) => Some(Self::Phase(Phase::REP)),
                (S::S2, D::D4) => Some(Self::Phase(Phase::ITM)),
                (S::S2, D::D5) => Some(Self::Phase(Phase::RCT)),
                (S::S2, D::D6) => Some(Self::Phase(Phase::FRE)),
                (S::S2, D::D7) => Some(Self::Phase(Phase::FRG)),
                (S::S2, D::D8) => Some(Self::Phase(Phase::VAC)),
                (S::S2, D::D9) => Some(Self::Phase(Phase::FLC)),

                (S::S3, D::D1) => Some(Self::Effect(Effect::BEN1)),
                (S::S3, D::D2) => Some(Self::Effect(Effect::BEN2)),
                (S::S3, D::D3) => Some(Self::Effect(Effect::BEN3)),
                (S::S3, D::D4) => Some(Self::Effect(Effect::BENSELF)),
                (S::S3, D::D5) => Some(Self::Effect(Effect::UNK)),
                (S::S3, D::D6) => Some(Self::Effect(Effect::DETSELF)),
                (S::S3, D::D7) => Some(Self::Effect(Effect::DET3)),
                (S::S3, D::D8) => Some(Self::Effect(Effect::DET2)),
                (S::S3, D::D9) => Some(Self::Effect(Effect::DET1)),

                (S::S4, D::D1) => Some(Self::Level(Level::MIN)),
                (S::S4, D::D2) => Some(Self::Level(Level::SBE)),
                (S::S4, D::D3) => Some(Self::Level(Level::IFR)),
                (S::S4, D::D4) => Some(Self::Level(Level::DFC)),
                (S::S4, D::D5) => Some(Self::Level(Level::EQU)),
                (S::S4, D::D6) => Some(Self::Level(Level::SUR)),
                (S::S4, D::D7) => Some(Self::Level(Level::SPL)),
                (S::S4, D::D8) => Some(Self::Level(Level::SPQ)),
                (S::S4, D::D9) => Some(Self::Level(Level::MAX)),

                _ => None,
            }
        }
    }
}

impl AsGeneral<NormalCaShortcut> for ReferentialCaShortcut {
    fn as_general(self) -> NormalCaShortcut {
        match self {
            Self::Default => NormalCaShortcut::Default,
            Self::PRX => NormalCaShortcut::PRX,
        }
    }
}

impl From<ReferentialCaShortcut> for NormalCaShortcut {
    fn from(value: ReferentialCaShortcut) -> Self {
        match value {
            ReferentialCaShortcut::Default => Self::Default,
            ReferentialCaShortcut::PRX => Self::PRX,
        }
    }
}

impl TryAsSpecific<ReferentialCaShortcut> for NormalCaShortcut {
    fn try_as_specific(self) -> Option<ReferentialCaShortcut> {
        match self {
            Self::Default => Some(ReferentialCaShortcut::Default),
            Self::PRX => Some(ReferentialCaShortcut::PRX),
            _ => None,
        }
    }
}

impl IllocutionOrValidation {
    /// Attempts to construct an [`IllocutionOrValidation`] from a Vk form.
    pub fn from_vk(vk: VowelForm) -> Result<Self, ParseError> {
        if vk.has_glottal_stop {
            return Err(ParseError::GlottalizedVk);
        }

        match (vk.sequence, vk.degree) {
            (VowelFormSequence::S1, VowelFormDegree::D1) => Ok(Self::OBS),
            (VowelFormSequence::S1, VowelFormDegree::D2) => Ok(Self::REC),
            (VowelFormSequence::S1, VowelFormDegree::D3) => Ok(Self::PUP),
            (VowelFormSequence::S1, VowelFormDegree::D4) => Ok(Self::RPR),
            (VowelFormSequence::S1, VowelFormDegree::D5) => Ok(Self::USP),
            (VowelFormSequence::S1, VowelFormDegree::D6) => Ok(Self::IMA),
            (VowelFormSequence::S1, VowelFormDegree::D7) => Ok(Self::CVN),
            (VowelFormSequence::S1, VowelFormDegree::D8) => Ok(Self::ITU),
            (VowelFormSequence::S1, VowelFormDegree::D9) => Ok(Self::INF),

            (VowelFormSequence::S2, VowelFormDegree::D1) => Ok(Self::DIR),
            (VowelFormSequence::S2, VowelFormDegree::D2) => Ok(Self::DEC),
            (VowelFormSequence::S2, VowelFormDegree::D3) => Ok(Self::IRG),
            (VowelFormSequence::S2, VowelFormDegree::D4) => Ok(Self::VER),
            (VowelFormSequence::S2, VowelFormDegree::D6) => Ok(Self::ADM),
            (VowelFormSequence::S2, VowelFormDegree::D7) => Ok(Self::POT),
            (VowelFormSequence::S2, VowelFormDegree::D8) => Ok(Self::HOR),
            (VowelFormSequence::S2, VowelFormDegree::D9) => Ok(Self::CNJ),

            _ => Err(ParseError::ExpectedVk),
        }
    }
}

impl AsGeneral<DatalessRelation> for NominalMode {
    fn as_general(self) -> DatalessRelation {
        match self {
            Self::NOM => DatalessRelation::NOM,
            Self::T1 => DatalessRelation::T1,
            Self::T2 => DatalessRelation::T2,
            Self::FRM => DatalessRelation::FRM,
        }
    }
}

impl From<NominalMode> for DatalessRelation {
    fn from(value: NominalMode) -> Self {
        value.as_general()
    }
}

impl TryAsSpecific<NominalMode> for DatalessRelation {
    fn try_as_specific(self) -> Option<NominalMode> {
        match self {
            Self::NOM => Some(NominalMode::NOM),
            Self::T1 => Some(NominalMode::T1),
            Self::T2 => Some(NominalMode::T2),
            Self::VRB => None,
            Self::FRM => Some(NominalMode::FRM),
        }
    }
}

macro_rules! cs_forms {
    ($($ty:ident :: $name:ident = $value:literal,)+) => {
        $(impl $ty {
            /// The Cs form used in the affix representing this type.
            pub const $name: &'static str = $value;
        })+
    };
}

cs_forms!(
    Valence::CS_FORM = "ẓk",
    Phase::CS_FORM = "bž",
    Effect::CS_FORM = "m",
    Level::CS_FORM = "řž",
    Sanction::CS_FORM = "gj",
    IllocutionOrValidation::CS_FORM = "nļ",
    MoodOrCaseScope::CS_FORM = "bẓ",
);

impl Aspect {
    /// The first Cs form used in the affix representing this type.
    pub const CS_FORM_1: &'static str = "mc";

    /// The second Cs form used in the affix representing this type.
    pub const CS_FORM_2: &'static str = "mč";

    /// The third Cs form used in the affix representing this type.
    pub const CS_FORM_3: &'static str = "mẓ";

    /// The fourth Cs form used in the affix representing this type.
    pub const CS_FORM_4: &'static str = "mj";
}
