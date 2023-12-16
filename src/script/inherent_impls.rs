use super::{character::*, flags::IntoScriptFlags, traits::IntoSecondary};
use crate::{
    ca_pat,
    category::{
        Affiliation, AffixDegree, AffixSlot, AffixType, Case, CaseAccessorMode, CaseScope, Context,
        DatalessRelation, Essence, Extension, Function, Illocution, Level, Mood, Perspective,
        Plexity, Separability, Similarity, SimilarityAndSeparability, Specification, Stem,
        Validation, VcOrVk, Version,
    },
    prelude::token::NumeralForm,
};
use vec1::Vec1;

impl Core {
    /// Gets the core of a primary character.
    pub const fn primary_core(specification: Specification) -> Self {
        match specification {
            Specification::BSC => Self::BSC,
            Specification::CTE => Self::CTE,
            Specification::CSV => Self::CSV,
            Specification::OBJ => Self::OBJ,
        }
    }

    /// Gets the core associated with a character.
    pub const fn from_char(char: char) -> Option<Self> {
        match char {
            'b' => Some(Self::B),
            'c' => Some(Self::C),
            'ç' => Some(Self::Ç),
            'č' => Some(Self::Č),
            'd' => Some(Self::D),
            'ḑ' => Some(Self::Ḑ),
            'f' => Some(Self::F),
            'g' => Some(Self::G),
            'h' => Some(Self::H),
            'j' => Some(Self::J),
            'k' => Some(Self::K),
            'l' => Some(Self::L),
            'ļ' => Some(Self::Ļ),
            'm' => Some(Self::M),
            'n' => Some(Self::N),
            'ň' => Some(Self::Ň),
            'p' => Some(Self::P),
            'r' => Some(Self::R),
            'ř' => Some(Self::Ř),
            's' => Some(Self::S),
            'š' => Some(Self::Š),
            't' => Some(Self::T),
            'ţ' => Some(Self::Ţ),
            'v' => Some(Self::V),
            'x' => Some(Self::X),
            'z' => Some(Self::Z),
            'ż' => Some(Self::Ẓ),
            'ž' => Some(Self::Ž),
            _ => None,
        }
    }
}

impl Ext {
    /// Gets the extension for the top left of a primary character.
    pub const fn primary_top(extension: Extension, perspective: Perspective) -> Option<Self> {
        match perspective {
            Perspective::M => match extension {
                Extension::DEL => None,
                Extension::PRX => Some(Ext::S),
                Extension::ICP => Some(Ext::T),
                Extension::ATV => Some(Ext::D),
                Extension::GRA => Some(Ext::M),
                Extension::DPL => Some(Ext::N),
            },
            Perspective::G => match extension {
                Extension::DEL => Some(Ext::P),
                Extension::PRX => Some(Ext::G),
                Extension::ICP => Some(Ext::Ž),
                Extension::ATV => Some(Ext::Ḑ),
                Extension::GRA => Some(Ext::V),
                Extension::DPL => Some(Ext::X),
            },
            Perspective::N => match extension {
                Extension::DEL => Some(Ext::Š),
                Extension::PRX => Some(Ext::GeminateExt),
                Extension::ICP => Some(Ext::W),
                Extension::ATV => Some(Ext::H),
                Extension::GRA => Some(Ext::F),
                Extension::DPL => Some(Ext::Ř),
            },
            Perspective::A => match extension {
                Extension::DEL => Some(Ext::B),
                Extension::PRX => Some(Ext::K),
                Extension::ICP => Some(Ext::C),
                Extension::ATV => Some(Ext::Č),
                Extension::GRA => Some(Ext::Ẓ),
                Extension::DPL => Some(Ext::J),
            },
        }
    }

    /// Gets the extension for the bottom right of a primary character.
    pub const fn primary_bottom(
        function: Function,
        version: Version,
        plexity: Plexity,
        stem: Stem,
    ) -> Option<Self> {
        match function {
            Function::STA => match version {
                Version::PRC => match plexity {
                    Plexity::M => match stem {
                        Stem::S0 => Some(Self::B),
                        Stem::S1 => None,
                        Stem::S2 => Some(Self::P),
                        Stem::S3 => Some(Self::Š),
                    },
                    Plexity::D => match stem {
                        Stem::S0 => Some(Self::C),
                        Stem::S1 => Some(Self::Z),
                        Stem::S2 => Some(Self::PWithLine),
                        Stem::S3 => Some(Self::W),
                    },
                },
                Version::CPT => match plexity {
                    Plexity::M => match stem {
                        Stem::S0 => Some(Self::K),
                        Stem::S1 => Some(Self::L),
                        Stem::S2 => Some(Self::G),
                        Stem::S3 => Some(Self::GeminateExt),
                    },
                    Plexity::D => match stem {
                        Stem::S0 => Some(Self::Č),
                        Stem::S1 => Some(Self::RFlipped),
                        Stem::S2 => Some(Self::GWithLine),
                        Stem::S3 => Some(Self::H),
                    },
                },
            },
            Function::DYN => match version {
                Version::PRC => match plexity {
                    Plexity::M => match stem {
                        Stem::S0 => Some(Self::D),
                        Stem::S1 => Some(Self::M),
                        Stem::S2 => Some(Self::GeminateCore),
                        Stem::S3 => Some(Self::T),
                    },
                    Plexity::D => match stem {
                        Stem::S0 => Some(Self::DWithLine),
                        Stem::S1 => Some(Self::N),
                        Stem::S2 => Some(Self::Ň),
                        Stem::S3 => Some(Self::Ž),
                    },
                },
                Version::CPT => match plexity {
                    Plexity::M => match stem {
                        Stem::S0 => Some(Self::Ţ),
                        Stem::S1 => Some(Self::S),
                        Stem::S2 => Some(Self::X),
                        Stem::S3 => Some(Self::F),
                    },
                    Plexity::D => match stem {
                        Stem::S0 => Some(Self::Ḑ),
                        Stem::S1 => Some(Self::R),
                        Stem::S2 => Some(Self::Ř),
                        Stem::S3 => Some(Self::V),
                    },
                },
            },
        }
    }

    /// Gets the top and bottom extensions for a standard quaternary character.
    pub const fn standard_quaternary_exts(vc_or_vk: VcOrVk) -> (Option<Self>, Option<Self>) {
        match vc_or_vk {
            VcOrVk::Case(case) => {
                const CASE_EXTENSIONS: [Option<Ext>; 9] = [
                    None,
                    Some(Ext::D),
                    Some(Ext::Š),
                    Some(Ext::G),
                    Some(Ext::P),
                    Some(Ext::T),
                    Some(Ext::K),
                    Some(Ext::B),
                    Some(Ext::M),
                ];

                (
                    CASE_EXTENSIONS[case as usize / 9],
                    CASE_EXTENSIONS[case as usize % 9],
                )
            }
            VcOrVk::Illocution(illocution) => (
                match illocution {
                    Illocution::ASR => Some(Ext::S),
                    Illocution::DIR => Some(Ext::Ḑ),
                    Illocution::DEC => Some(Ext::F),
                    Illocution::IRG => Some(Ext::Ř),
                    Illocution::VER => Some(Ext::V),
                    Illocution::ADM => Some(Ext::Ţ),
                    Illocution::POT => Some(Ext::X),
                    Illocution::HOR => Some(Ext::N),
                    Illocution::CNJ => Some(Ext::Z),
                },
                None,
            ),
            VcOrVk::Validation(validation) => (
                Some(Ext::S),
                match validation {
                    Validation::OBS => Some(Ext::S),
                    Validation::REC => Some(Ext::Ḑ),
                    Validation::PUP => Some(Ext::F),
                    Validation::RPR => Some(Ext::Ř),
                    Validation::USP => Some(Ext::V),
                    Validation::IMA => Some(Ext::Ţ),
                    Validation::CVN => Some(Ext::X),
                    Validation::ITU => Some(Ext::N),
                    Validation::INF => Some(Ext::Z),
                },
            ),
        }
    }

    /// Gets the top and bottom extensions for a case-accessor quaternary character.
    pub const fn accessor_quaternary_exts(case: Case) -> (Option<Self>, Option<Self>) {
        const CASE_EXTENSIONS: [Option<Ext>; 9] = [
            None,
            Some(Ext::D),
            Some(Ext::Š),
            Some(Ext::G),
            Some(Ext::P),
            Some(Ext::T),
            Some(Ext::K),
            Some(Ext::B),
            Some(Ext::M),
        ];

        (
            CASE_EXTENSIONS[case as usize / 9],
            CASE_EXTENSIONS[case as usize % 9],
        )
    }

    /// Gets the extension associated with a character.
    pub const fn from_char(char: char) -> Option<Self> {
        match char {
            'b' => Some(Self::B),
            'c' => Some(Self::C),
            'ç' => Some(Self::Ç),
            'č' => Some(Self::Č),
            'd' => Some(Self::D),
            'ḑ' => Some(Self::Ḑ),
            'f' => Some(Self::F),
            'g' => Some(Self::G),
            'h' => Some(Self::H),
            'j' => Some(Self::J),
            'k' => Some(Self::K),
            'l' => Some(Self::L),
            'ļ' => Some(Self::Ļ),
            'm' => Some(Self::M),
            'n' => Some(Self::N),
            'ň' => Some(Self::Ň),
            'p' => Some(Self::P),
            'r' => Some(Self::R),
            'ř' => Some(Self::Ř),
            's' => Some(Self::S),
            'š' => Some(Self::Š),
            't' => Some(Self::T),
            'ţ' => Some(Self::Ţ),
            'v' => Some(Self::V),
            'w' => Some(Self::W),
            'x' => Some(Self::X),
            'y' => Some(Self::Y),
            'z' => Some(Self::Z),
            'ż' => Some(Self::Ẓ),
            'ž' => Some(Self::Ž),
            _ => None,
        }
    }
}

impl Diacritic {
    /// Gets the diacritic representing an elided primary.
    ///
    /// Returns [`None`] if the primary cannot be elided, [`Some(None)`] if the primary elides to
    /// nothing, and [`Some(Some(...))`] if the primary elides to a diacritic.
    pub const fn primary_elided(primary: Primary) -> Option<Option<Self>> {
        match primary {
            Primary {
                specification: Specification::BSC,
                ca: ca_pat!(),
                function: Function::STA,
                version: Version::PRC,
                stem: Stem::S1,
                context: Context::EXS,
                relation,
            } => match relation {
                DatalessRelation::NOM => Some(None),
                DatalessRelation::VRB => Some(Some(Self::Dot)),
                DatalessRelation::FRM => Some(Some(Self::HorizBar)),
                _ => None,
            },
            _ => None,
        }
    }

    /// Gets the superposed diacritic for a primary character.
    pub const fn primary_superposed(context: Context) -> Option<Self> {
        match context {
            Context::EXS => None,
            Context::FNC => Some(Self::Dot),
            Context::RPS => Some(Self::HorizBar),
            Context::AMG => Some(Self::DiagBar),
        }
    }

    /// Gets the underposed diacritic for a primary character.
    pub const fn primary_underposed(relation: DatalessRelation) -> Option<Self> {
        match relation {
            DatalessRelation::NOM => None,
            DatalessRelation::VRB => Some(Self::Dot),
            DatalessRelation::FRM => Some(Self::HorizBar),
            DatalessRelation::T1 => Some(Self::VertBar),
            DatalessRelation::T2 => Some(Self::HorizBarWithBottomLine),
        }
    }

    /// Gets the non-optional leftposed diacritic for a primary character.
    const fn primary_leftposed_req(similarity_separability: SimilarityAndSeparability) -> Self {
        match similarity_separability {
            SimilarityAndSeparability {
                similarity: Similarity::S,
                separability: Separability::S,
            } => Diacritic::DiagBar,

            SimilarityAndSeparability {
                similarity: Similarity::S,
                separability: Separability::C,
            } => Diacritic::VertBar,

            SimilarityAndSeparability {
                similarity: Similarity::S,
                separability: Separability::F,
            } => Diacritic::HorizBar,

            SimilarityAndSeparability {
                similarity: Similarity::D,
                separability: Separability::S,
            } => Diacritic::HorizBarWithTopLine,

            SimilarityAndSeparability {
                similarity: Similarity::D,
                separability: Separability::C,
            } => Diacritic::VertBarWithRightLine,

            SimilarityAndSeparability {
                similarity: Similarity::D,
                separability: Separability::F,
            } => Diacritic::CurveTowardsBottom,

            SimilarityAndSeparability {
                similarity: Similarity::F,
                separability: Separability::S,
            } => Diacritic::CurveTowardsTop,

            SimilarityAndSeparability {
                similarity: Similarity::F,
                separability: Separability::C,
            } => Diacritic::TwoPartHorizAndDiagBar,

            SimilarityAndSeparability {
                similarity: Similarity::F,
                separability: Separability::F,
            } => Diacritic::CurveTowardsBottomWithLine,
        }
    }

    /// Gets the leftposed diacritic for a primary character.
    pub const fn primary_leftposed(
        similarity_separability: Option<SimilarityAndSeparability>,
    ) -> Option<Self> {
        match similarity_separability {
            None => None,
            Some(value) => Some(Self::primary_leftposed_req(value)),
        }
    }

    /// Gets the rightposed diacritic for a primary character.
    pub const fn primary_rightposed(affiliation: Affiliation, essence: Essence) -> Option<Self> {
        match essence {
            Essence::NRM => match affiliation {
                Affiliation::CSL => None,
                Affiliation::ASO => Some(Self::DiagBar),
                Affiliation::VAR => Some(Self::VertBar),
                Affiliation::COA => Some(Self::HorizBar),
            },
            Essence::RPV => match affiliation {
                Affiliation::CSL => Some(Self::HorizBarWithBottomLine),
                Affiliation::ASO => Some(Self::VertBarWithLeftLine),
                Affiliation::VAR => Some(Self::CurveTowardsTop),
                Affiliation::COA => Some(Self::CurveTowardsBottom),
            },
        }
    }

    /// Gets the diacritic pair for when a quaternary is elided onto a secondary character.
    pub const fn elided_quaternary_pair(vc_or_vk: VcOrVk) -> (Option<Self>, Option<Self>) {
        match vc_or_vk {
            VcOrVk::Case(case) => (
                match (case as u8) / 9 {
                    0 => None,
                    1 => Some(Diacritic::Dot),
                    2 => Some(Diacritic::HorizBar),
                    3 => Some(Diacritic::CurveTowardsLeft),
                    4 => Some(Diacritic::CurveTowardsRight),
                    5 => Some(Diacritic::HorizBarWithBottomLine),
                    6 => Some(Diacritic::HorizBarWithTopLine),
                    7 => Some(Diacritic::CurveTowardsTop),
                    _ => unreachable!(),
                },
                match (case as u8) % 9 {
                    0 => None,
                    1 => Some(Diacritic::Dot),
                    2 => Some(Diacritic::HorizBar),
                    3 => Some(Diacritic::CurveTowardsLeft),
                    4 => Some(Diacritic::CurveTowardsRight),
                    5 => Some(Diacritic::HorizBarWithBottomLine),
                    6 => Some(Diacritic::HorizBarWithTopLine),
                    7 => Some(Diacritic::CurveTowardsTop),
                    8 => Some(Diacritic::CurveTowardsBottom),
                    _ => unreachable!(),
                },
            ),
            VcOrVk::Illocution(illocution) => (
                match illocution {
                    Illocution::ASR => None,
                    Illocution::DIR => Some(Diacritic::Dot),
                    Illocution::DEC => Some(Diacritic::HorizBar),
                    Illocution::IRG => Some(Diacritic::CurveTowardsLeft),
                    Illocution::VER => Some(Diacritic::CurveTowardsRight),
                    Illocution::ADM => Some(Diacritic::HorizBarWithBottomLine),
                    Illocution::POT => Some(Diacritic::HorizBarWithTopLine),
                    Illocution::HOR => Some(Diacritic::CurveTowardsTop),
                    Illocution::CNJ => Some(Diacritic::CurveTowardsBottom),
                },
                None,
            ),
            VcOrVk::Validation(validation) => (
                None,
                match validation {
                    Validation::OBS => None,
                    Validation::REC => Some(Diacritic::Dot),
                    Validation::PUP => Some(Diacritic::HorizBar),
                    Validation::RPR => Some(Diacritic::CurveTowardsLeft),
                    Validation::USP => Some(Diacritic::CurveTowardsRight),
                    Validation::IMA => Some(Diacritic::HorizBarWithBottomLine),
                    Validation::CVN => Some(Diacritic::HorizBarWithTopLine),
                    Validation::ITU => Some(Diacritic::CurveTowardsTop),
                    Validation::INF => Some(Diacritic::CurveTowardsBottom),
                },
            ),
        }
    }

    /// Gets the superposed diacritic for a standard quaternary character.
    pub const fn standard_quaternary_superposed(mood: Mood) -> Option<Self> {
        match mood {
            Mood::FAC => None,
            Mood::SUB => Some(Self::Dot),
            Mood::ASM => Some(Self::DiagBar),
            Mood::SPC => Some(Self::CurveTowardsTop),
            Mood::COU => Some(Self::CurveTowardsBottom),
            Mood::HYP => Some(Self::HorizBar),
        }
    }

    /// Gets the underposed diacritic for a standard quaternary character.
    pub const fn standard_quaternary_underposed(case_scope: CaseScope) -> Option<Self> {
        match case_scope {
            CaseScope::CCN => None,
            CaseScope::CCA => Some(Self::Dot),
            CaseScope::CCS => Some(Self::DiagBar),
            CaseScope::CCQ => Some(Self::CurveTowardsTop),
            CaseScope::CCP => Some(Self::CurveTowardsBottom),
            CaseScope::CCV => Some(Self::HorizBar),
        }
    }

    /// Gets the superposed diacritic representing an affix type.
    pub const fn affix_type(r#type: AffixType) -> Option<Self> {
        match r#type {
            AffixType::T1 => None,
            AffixType::T2 => Some(Diacritic::Dot),
            AffixType::T3 => Some(Diacritic::HorizBar),
        }
    }

    /// Gets the rightposed diacritic representing an affix slot.
    ///
    /// This returns [`Some(Dot)`] for `AffixSlot::XI` and [`None`] for everything else.
    pub const fn affix_slot(slot: AffixSlot) -> Option<Self> {
        match slot {
            AffixSlot::V | AffixSlot::VII => None,
            AffixSlot::XI => Some(Diacritic::Dot),
        }
    }

    /// Gets the _nonstandard_ superposed diacritic representing a numeric affix type.
    ///
    /// This is used because numerals can't be rotated, and thus need special handling to
    /// distinguish when they're in slot V vs VII.
    pub const fn numeric_affix_type(r#type: AffixType, slot: AffixSlot) -> Option<Self> {
        match (slot, r#type) {
            (AffixSlot::V, AffixType::T1) => None,
            (AffixSlot::V, AffixType::T2) => Some(Diacritic::Dot),
            (AffixSlot::V, AffixType::T3) => Some(Diacritic::HorizBar),

            (AffixSlot::VII, AffixType::T1) => Some(Diacritic::DiagBar),
            (AffixSlot::VII, AffixType::T2) => Some(Diacritic::CurveTowardsLeft),
            (AffixSlot::VII, AffixType::T3) => Some(Diacritic::CurveTowardsRight),

            (AffixSlot::XI, AffixType::T1) => Some(Diacritic::VertBar),
            (AffixSlot::XI, AffixType::T2) => Some(Diacritic::CurveTowardsLeftWithDot),
            (AffixSlot::XI, AffixType::T3) => Some(Diacritic::CurveTowardsRightWithDot),
        }
    }

    /// Gets the underposed diacritic for a case-accessor quaternary.
    pub const fn accessor_quaternary_underposed(slot: AffixSlot, mode: CaseAccessorMode) -> Self {
        match slot {
            AffixSlot::V => match mode {
                CaseAccessorMode::Normal => Self::CurveTowardsLeft,
                CaseAccessorMode::Inverse => Self::CurveTowardsRight,
            },
            AffixSlot::VII => match mode {
                CaseAccessorMode::Normal => Self::CurveTowardsLeftWithDot,
                CaseAccessorMode::Inverse => Self::CurveTowardsRightWithDot,
            },
            AffixSlot::XI => match mode {
                CaseAccessorMode::Normal => Self::CurveTowardsTop,
                CaseAccessorMode::Inverse => Self::CurveTowardsBottom,
            },
        }
    }

    /// Gets the diacritic associated with an affix degree.
    pub const fn affix_degree(degree: AffixDegree) -> Self {
        match degree {
            AffixDegree::D0 => Self::CurveTowardsRight,
            AffixDegree::D1 => Self::Dot,
            AffixDegree::D2 => Self::HorizBarWithBottomLine,
            AffixDegree::D3 => Self::VertBarWithRightLine,
            AffixDegree::D4 => Self::CurveTowardsTop,
            AffixDegree::D5 => Self::DiagBar,
            AffixDegree::D6 => Self::CurveTowardsBottom,
            AffixDegree::D7 => Self::VertBarWithLeftLine,
            AffixDegree::D8 => Self::HorizBarWithTopLine,
            AffixDegree::D9 => Self::HorizBar,
        }
    }

    /// The diacritic representing a Ca-stacking affix.
    pub const CA_STACKING_UNDERPOSED: Self = Self::CurveTowardsLeft;

    /// Gets the diacritic representing a level.
    pub const fn level(level: Level) -> Self {
        match level {
            Level::MIN => Self::Dot,
            Level::SBE => Self::HorizBarWithTopLine,
            Level::IFR => Self::VertBarWithLeftLine,
            Level::DFC => Self::CurveTowardsTop,
            Level::EQU => Self::DiagBar,
            Level::SUR => Self::CurveTowardsBottom,
            Level::SPL => Self::VertBarWithRightLine,
            Level::SPQ => Self::HorizBarWithBottomLine,
            Level::MAX => Self::HorizBar,
        }
    }
}

impl Secondary {
    /// Gets a sequence of secondaries representing a Cr root or Cs form.
    ///
    /// This function is specalized to use standard placeholders in all characters except the first,
    /// and to not allow placeholders in the first character. It also handles underscore
    /// placeholders properly.
    ///
    /// Returns [`None`] if the characters are invalid.
    pub fn cr_or_cs(s: &str, is_rotated: bool, flags: IntoScriptFlags) -> Option<Vec1<Self>> {
        // A Cr or Cs root must follow this pattern: (Ext? Core Ext?) (Ext Ext?)+

        let mut chars = s.chars();

        let (top, core, bottom) = match (chars.next(), chars.next()) {
            // If there are no characters, exit early.
            (None, _) => return None,

            // If there's one character, it must be a core.
            (Some(char), None) => (None, char, None),

            // If there are two characters and `c2` is a core shape, we can take another character
            // as an extension. Otherwise, exit early.
            (Some(c1), Some(c2)) => match Core::from_char(c2) {
                // If `c2` is a core shape...
                Some(_) => match chars.next() {
                    // Use the next character as a bottom extension.
                    Some(c3) => (Some(c1), c2, Some(c3)),

                    // If we only have two characters,
                    None => {
                        // If the user requests bottom extensions and we can make a bottom
                        // extension, do that.
                        if !flags.matches(IntoScriptFlags::PREFER_TOP_EXTENSIONS)
                            && Core::from_char(c1).is_some()
                        {
                            (None, c1, Some(c2))
                        }
                        // Otherwise, return a top extension.
                        else {
                            (Some(c1), c2, None)
                        }
                    }
                },

                // If `c2` isn't a core shape, make it a bottom extension to `c1`.
                None => (None, c1, Some(c2)),
            },
        };

        let mut output = Vec1::new(Secondary {
            is_rotated,
            core: Core::from_char(core)?,
            top: match top {
                Some('_') | None => None,
                // We use Some(..?) here because we want to return if we get an invalid extension.
                Some(top) => Some(Ext::from_char(top)?),
            },
            bottom: match bottom {
                Some('_') | None => None,
                // We use Some(..?) here because we want to return if we get an invalid extension.
                Some(bottom) => Some(Ext::from_char(bottom)?),
            },
            superposed: None,
            underposed: None,
            leftposed: None,
            rightposed: None,
        });

        loop {
            match (chars.next(), chars.next()) {
                (None, _) => break,

                (Some(char), None) => {
                    if flags.matches(IntoScriptFlags::PREFER_TOP_EXTENSIONS) {
                        output.push(Secondary {
                            is_rotated,
                            core: Core::StandardPlaceholder,
                            // We explicitly don't handle underscores because otherwise it'd be an
                            // empty character.
                            top: Some(Ext::from_char(char)?),
                            bottom: None,
                            superposed: None,
                            underposed: None,
                            leftposed: None,
                            rightposed: None,
                        });
                    } else {
                        output.push(Secondary {
                            is_rotated,
                            core: Core::StandardPlaceholder,
                            top: None,
                            // We explicitly don't handle underscores because otherwise it'd be an
                            // empty character.
                            bottom: Some(Ext::from_char(char)?),
                            superposed: None,
                            underposed: None,
                            leftposed: None,
                            rightposed: None,
                        });
                    }
                }

                (Some(c1), Some(c2)) => match (c1, c2) {
                    ('_', bottom) => output.push(Secondary {
                        is_rotated,
                        core: Core::StandardPlaceholder,
                        top: None,
                        // We explicitly don't handle underscores because otherwise it'd be an
                        // empty character.
                        bottom: Some(Ext::from_char(bottom)?),
                        superposed: None,
                        underposed: None,
                        leftposed: None,
                        rightposed: None,
                    }),

                    (top, '_') => output.push(Secondary {
                        is_rotated,
                        core: Core::StandardPlaceholder,
                        // We explicitly don't handle underscores because otherwise it'd be an
                        // empty character.
                        top: Some(Ext::from_char(top)?),
                        bottom: None,
                        superposed: None,
                        underposed: None,
                        leftposed: None,
                        rightposed: None,
                    }),

                    (top, bottom) => output.push(Secondary {
                        is_rotated,
                        core: Core::StandardPlaceholder,
                        top: Some(Ext::from_char(top)?),
                        bottom: Some(Ext::from_char(bottom)?),
                        superposed: None,
                        underposed: None,
                        leftposed: None,
                        rightposed: None,
                    }),
                },
            }
        }

        Some(output)
    }

    /// Gets a sequence of secondaries representing a numeric form.
    pub fn numeral(form: NumeralForm) -> Vec1<Self> {
        Numeral::numeral(form).mapped(IntoSecondary::into_secondary)
    }
}

impl Numeral {
    /// Gets a sequence of numerals representing a numeric form.
    pub fn numeral(form: NumeralForm) -> Vec1<Numeral> {
        let mut value = form.integer_part;

        let mut data = Vec1::new({
            let my_value = value % 10_000;
            value /= 10_000;
            Numeral {
                value: my_value as u16,
            }
        });

        loop {
            if value == 0 {
                break;
            }

            let my_value = value % 10_000;
            value /= 10_000;

            data.push(Numeral {
                value: my_value as u16,
            });
        }

        data.reverse();

        data
    }
}
