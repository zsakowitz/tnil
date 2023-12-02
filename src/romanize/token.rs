//! Contains types for various tokens.

use std::{
    error::Error,
    fmt::{self, Display},
    str::FromStr,
};

use crate::category::{
    Bias, Case, CaseScope, HFormDegree, HFormSequence, Mood, MoodOrCaseScope, Register, Stress,
    SuppletiveAdjunctMode, VowelFormDegree, VowelFormSequence,
};

/// A consonant form.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConsonantForm {
    /// The consonants in this form.
    pub source: String,
}

/// A normal vowel form.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VowelForm {
    /// Whether this form has a glottal stop.
    pub has_glottal_stop: bool,

    /// The sequence of this form.
    pub sequence: VowelFormSequence,

    /// The degree of this form.
    pub degree: VowelFormDegree,
}

/// A vowel form consisting of -üa-.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ÜA;

/// A vowel form consisting of -ë-.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Schwa;

/// A consonant form starting with h-, w-, or y-.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HForm {
    /// The sequence of this form.
    pub sequence: HFormSequence,

    /// The degree of this form.
    pub degree: HFormDegree,
}

/// A consonant form consisting of a single "h".
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hh;

/// A consonant form consisting of "hr".
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hr;

/// A consonant form that is either "w" or "y".
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum WYForm {
    /// A consonant form consisting of a lone W.
    W,

    /// A consonant form consisting of a lone Y.
    Y,
}

/// A numeral form.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive] // TODO: Remove non_exhaustive once we decide how to work with decimals.
pub struct NumeralForm {
    /// The integral part of this numeral.
    pub integer_part: u64,
}

/// A glottal stop without any corresponding vowels, or a word-final glottal stop.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GlottalStop;

/// A generic token.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    /// A consonant form.
    Consonant(ConsonantForm),

    /// A normal vowel form.
    Vowel(VowelForm),

    /// The vowel form -üa-.
    ÜA(ÜA),

    /// The vowel form -ë-.
    Schwa(Schwa),

    /// A consonant form starting with h-, w-, or y-.
    H(HForm),

    /// A numeral form.
    Numeral(NumeralForm),

    /// A glottal stop without any corresponding vowels, or a word-final glottal stop.
    GlottalStop,
}

/// An error returned when an invalid vowel form is parsed.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParseVowelFormError;

impl Display for ParseVowelFormError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid vowel form")
    }
}

impl Error for ParseVowelFormError {}

impl FromStr for VowelForm {
    type Err = ParseVowelFormError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let has_glottal_stop = s.contains('\'');

        let (sequence, degree) = match &s.replace('\'', "")[..] {
            "ae" => (VowelFormSequence::S1, VowelFormDegree::D0),
            "a" => (VowelFormSequence::S1, VowelFormDegree::D1),
            "ä" => (VowelFormSequence::S1, VowelFormDegree::D2),
            "e" => (VowelFormSequence::S1, VowelFormDegree::D3),
            "i" => (VowelFormSequence::S1, VowelFormDegree::D4),
            "ëi" => (VowelFormSequence::S1, VowelFormDegree::D5),
            "ö" => (VowelFormSequence::S1, VowelFormDegree::D6),
            "o" => (VowelFormSequence::S1, VowelFormDegree::D7),
            "ü" => (VowelFormSequence::S1, VowelFormDegree::D8),
            "u" => (VowelFormSequence::S1, VowelFormDegree::D9),

            "ea" => (VowelFormSequence::S2, VowelFormDegree::D0),
            "ai" => (VowelFormSequence::S2, VowelFormDegree::D1),
            "au" => (VowelFormSequence::S2, VowelFormDegree::D2),
            "ei" => (VowelFormSequence::S2, VowelFormDegree::D3),
            "eu" => (VowelFormSequence::S2, VowelFormDegree::D4),
            "ëu" => (VowelFormSequence::S2, VowelFormDegree::D5),
            "ou" => (VowelFormSequence::S2, VowelFormDegree::D6),
            "oi" => (VowelFormSequence::S2, VowelFormDegree::D7),
            "iu" => (VowelFormSequence::S2, VowelFormDegree::D8),
            "ui" => (VowelFormSequence::S2, VowelFormDegree::D9),

            "üo" => (VowelFormSequence::S3, VowelFormDegree::D0),
            "ia" => (VowelFormSequence::S3, VowelFormDegree::D1),
            "uä" => (VowelFormSequence::S3, VowelFormDegree::D1),
            "ie" => (VowelFormSequence::S3, VowelFormDegree::D2),
            "uë" => (VowelFormSequence::S3, VowelFormDegree::D2),
            "io" => (VowelFormSequence::S3, VowelFormDegree::D3),
            "üä" => (VowelFormSequence::S3, VowelFormDegree::D3),
            "iö" => (VowelFormSequence::S3, VowelFormDegree::D4),
            "üë" => (VowelFormSequence::S3, VowelFormDegree::D4),
            "eë" => (VowelFormSequence::S3, VowelFormDegree::D5),
            "uö" => (VowelFormSequence::S3, VowelFormDegree::D6),
            "öë" => (VowelFormSequence::S3, VowelFormDegree::D6),
            "uo" => (VowelFormSequence::S3, VowelFormDegree::D7),
            "öä" => (VowelFormSequence::S3, VowelFormDegree::D7),
            "ue" => (VowelFormSequence::S3, VowelFormDegree::D8),
            "ië" => (VowelFormSequence::S3, VowelFormDegree::D8),
            "ua" => (VowelFormSequence::S3, VowelFormDegree::D9),
            "iä" => (VowelFormSequence::S3, VowelFormDegree::D9),

            "üö" => (VowelFormSequence::S4, VowelFormDegree::D0),
            "ao" => (VowelFormSequence::S4, VowelFormDegree::D1),
            "aö" => (VowelFormSequence::S4, VowelFormDegree::D2),
            "eo" => (VowelFormSequence::S4, VowelFormDegree::D3),
            "eö" => (VowelFormSequence::S4, VowelFormDegree::D4),
            "oë" => (VowelFormSequence::S4, VowelFormDegree::D5),
            "öe" => (VowelFormSequence::S4, VowelFormDegree::D6),
            "oe" => (VowelFormSequence::S4, VowelFormDegree::D7),
            "öa" => (VowelFormSequence::S4, VowelFormDegree::D8),
            "oa" => (VowelFormSequence::S4, VowelFormDegree::D9),

            _ => return Err(ParseVowelFormError),
        };

        Ok(Self {
            has_glottal_stop,
            sequence,
            degree,
        })
    }
}

/// An error returned when an invalid h-form is parsed.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParseHFormError;

impl Display for ParseHFormError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid h-form")
    }
}

impl Error for ParseHFormError {}

impl FromStr for HForm {
    type Err = ParseHFormError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sequence, degree) = match s {
            "h" => (HFormSequence::S0, HFormDegree::D1),
            "hl" => (HFormSequence::S0, HFormDegree::D2),
            "hr" => (HFormSequence::S0, HFormDegree::D3),
            "hm" => (HFormSequence::S0, HFormDegree::D4),
            "hn" => (HFormSequence::S0, HFormDegree::D5),
            "hň" => (HFormSequence::S0, HFormDegree::D6),

            "w" => (HFormSequence::SW, HFormDegree::D1),
            "hw" => (HFormSequence::SW, HFormDegree::D2),
            "hrw" => (HFormSequence::SW, HFormDegree::D3),
            "hmw" => (HFormSequence::SW, HFormDegree::D4),
            "hnw" => (HFormSequence::SW, HFormDegree::D5),
            "hňw" => (HFormSequence::SW, HFormDegree::D6),

            "y" => (HFormSequence::SY, HFormDegree::D1),

            _ => return Err(ParseHFormError),
        };

        Ok(Self { sequence, degree })
    }
}

/// A trait implemented by tokens allowing them to be parsed by `Token::parse()`.
pub trait FromToken {
    /// Tries to parse a token as a `Self`, returning [`None`] in the event of failure.
    fn try_from_token(token: &Token) -> Option<Self>
    where
        Self: Sized;
}

impl FromToken for ConsonantForm {
    fn try_from_token(token: &Token) -> Option<Self> {
        match token {
            Token::Consonant(value) => Some(value.clone()),
            _ => None,
        }
    }
}

impl FromToken for VowelForm {
    fn try_from_token(token: &Token) -> Option<Self> {
        match token {
            Token::Vowel(value) => Some(*value),
            _ => None,
        }
    }
}

impl FromToken for ÜA {
    fn try_from_token(token: &Token) -> Option<Self> {
        match token {
            Token::ÜA(value) => Some(*value),
            _ => None,
        }
    }
}

impl FromToken for Schwa {
    fn try_from_token(token: &Token) -> Option<Self> {
        match token {
            Token::Schwa(value) => Some(*value),
            _ => None,
        }
    }
}

impl FromToken for HForm {
    fn try_from_token(token: &Token) -> Option<Self> {
        match token {
            Token::H(value) => Some(*value),
            _ => None,
        }
    }
}

impl FromToken for Hh {
    fn try_from_token(token: &Token) -> Option<Self> {
        match token {
            Token::H(HForm {
                sequence: HFormSequence::S0,
                degree: HFormDegree::D1,
            }) => Some(Self),
            _ => None,
        }
    }
}

impl FromToken for Hr {
    fn try_from_token(token: &Token) -> Option<Self> {
        match token {
            Token::H(HForm {
                sequence: HFormSequence::S0,
                degree: HFormDegree::D3,
            }) => Some(Self),
            _ => None,
        }
    }
}

impl FromToken for WYForm {
    fn try_from_token(token: &Token) -> Option<Self> {
        match token {
            Token::H(HForm {
                sequence: HFormSequence::SW,
                degree: HFormDegree::D1,
            }) => Some(WYForm::W),

            Token::H(HForm {
                sequence: HFormSequence::SY,
                degree: HFormDegree::D1,
            }) => Some(WYForm::Y),

            _ => None,
        }
    }
}

impl FromToken for NumeralForm {
    fn try_from_token(token: &Token) -> Option<Self> {
        match token {
            Token::Numeral(value) => Some(*value),
            _ => None,
        }
    }
}

impl FromToken for GlottalStop {
    fn try_from_token(token: &Token) -> Option<Self> {
        match token {
            Token::GlottalStop => Some(Self),
            _ => None,
        }
    }
}

impl FromToken for Bias {
    fn try_from_token(token: &Token) -> Option<Self> {
        match token {
            Token::Consonant(value) => match Bias::from_str(&value.source) {
                Ok(value) => Some(value),
                Err(_) => None,
            },
            _ => None,
        }
    }
}

impl FromToken for SuppletiveAdjunctMode {
    fn try_from_token(token: &Token) -> Option<Self> {
        match token {
            Token::H(HForm {
                sequence: HFormSequence::S0,
                degree,
            }) => match degree {
                HFormDegree::D2 => Some(SuppletiveAdjunctMode::CAR),
                HFormDegree::D4 => Some(SuppletiveAdjunctMode::QUO),
                HFormDegree::D5 => Some(SuppletiveAdjunctMode::NAM),
                HFormDegree::D6 => Some(SuppletiveAdjunctMode::PHR),
                _ => None,
            },

            _ => None,
        }
    }
}

impl FromToken for Case {
    fn try_from_token(token: &Token) -> Option<Self> {
        match token {
            Token::Vowel(VowelForm {
                has_glottal_stop,
                sequence,
                degree,
            }) if *degree != VowelFormDegree::D0 => {
                let degree = *degree as u8;
                let sequence = 9 * (*sequence as u8);
                let shift = 36 * (*has_glottal_stop as u8);
                Case::from_variant(shift + sequence + degree)
            }

            _ => None,
        }
    }
}

impl FromToken for Register {
    fn try_from_token(token: &Token) -> Option<Self> {
        match token {
            Token::Vowel(VowelForm {
                has_glottal_stop: false,
                sequence,
                degree,
            }) => match (sequence, degree) {
                (VowelFormSequence::S1, VowelFormDegree::D1) => Some(Register::DSV),
                (VowelFormSequence::S1, VowelFormDegree::D3) => Some(Register::PNT),
                (VowelFormSequence::S1, VowelFormDegree::D4) => Some(Register::SPF),
                (VowelFormSequence::S1, VowelFormDegree::D7) => Some(Register::EXM),
                (VowelFormSequence::S1, VowelFormDegree::D9) => Some(Register::CGT),
                (VowelFormSequence::S2, VowelFormDegree::D1) => Some(Register::DSV_END),
                (VowelFormSequence::S2, VowelFormDegree::D3) => Some(Register::PNT_END),
                (VowelFormSequence::S2, VowelFormDegree::D8) => Some(Register::SPF_END),
                (VowelFormSequence::S2, VowelFormDegree::D7) => Some(Register::EXM_END),
                (VowelFormSequence::S2, VowelFormDegree::D9) => Some(Register::CGT_END),
                (VowelFormSequence::S1, VowelFormDegree::D8) => Some(Register::END),
                _ => None,
            },
            _ => None,
        }
    }
}

impl FromToken for Stress {
    fn try_from_token(token: &Token) -> Option<Self> {
        match token {
            Token::Vowel(VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S1,
                degree,
            }) => match degree {
                VowelFormDegree::D1 => Some(Stress::Monosyllabic),
                VowelFormDegree::D3 => Some(Stress::Ultimate),
                VowelFormDegree::D7 => Some(Stress::Penultimate),
                VowelFormDegree::D9 => Some(Stress::Antepenultimate),
                _ => None,
            },
            _ => None,
        }
    }
}

impl FromToken for MoodOrCaseScope {
    fn try_from_token(token: &Token) -> Option<Self> {
        use VowelFormDegree as D;
        use VowelFormSequence as S;

        match token {
            Token::Vowel(VowelForm {
                has_glottal_stop: false,
                sequence,
                degree,
            }) => match (sequence, degree) {
                (S::S1, D::D1) => Some(MoodOrCaseScope::Mood(Mood::FAC)),
                (S::S1, D::D3) => Some(MoodOrCaseScope::Mood(Mood::SUB)),
                (S::S1, D::D4) => Some(MoodOrCaseScope::Mood(Mood::ASM)),
                (S::S1, D::D7) => Some(MoodOrCaseScope::Mood(Mood::SPC)),
                (S::S1, D::D6) => Some(MoodOrCaseScope::Mood(Mood::COU)),
                (S::S1, D::D9) => Some(MoodOrCaseScope::Mood(Mood::HYP)),

                (S::S2, D::D1) => Some(MoodOrCaseScope::CaseScope(CaseScope::CCN)),
                (S::S2, D::D3) => Some(MoodOrCaseScope::CaseScope(CaseScope::CCA)),
                (S::S2, D::D8) => Some(MoodOrCaseScope::CaseScope(CaseScope::CCS)),
                (S::S2, D::D7) => Some(MoodOrCaseScope::CaseScope(CaseScope::CCQ)),
                (S::S1, D::D8) => Some(MoodOrCaseScope::CaseScope(CaseScope::CCP)),
                (S::S2, D::D9) => Some(MoodOrCaseScope::CaseScope(CaseScope::CCV)),

                _ => None,
            },
            _ => None,
        }
    }
}

impl ConsonantForm {
    /// Creates a new `ConsonantForm`.
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
        }
    }

    /// Checks if this consonant form contains a geminate.
    ///
    /// # Examples
    ///
    /// ```
    /// use tnil::romanize::token::ConsonantForm;
    ///
    /// let consonant_form = ConsonantForm::new("rrw");
    /// assert!(consonant_form.is_geminate());
    ///
    /// let consonant_form = ConsonantForm::new("zv");
    /// assert!(!consonant_form.is_geminate());
    /// ```
    pub fn is_geminate(&self) -> bool {
        let mut chars = self.source.chars();

        let Some(mut last_char) = chars.next() else {
            return false;
        };

        for char in chars {
            if char == last_char {
                return true;
            }

            last_char = char;
        }

        false
    }
}
