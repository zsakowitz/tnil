//! Contains types for various tokens.

pub use super::consonant::*;
use super::{
    stream::ParseError,
    traits::{FromToken, IntoToken},
};
use crate::category::{HFormDegree, HFormSequence, VowelFormDegree, VowelFormSequence};
use std::{
    error::Error,
    fmt,
    ops::{Deref, DerefMut},
    str::FromStr,
};

/// An owned consonant form.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct OwnedConsonantForm(pub String);

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
    C(OwnedConsonantForm),

    /// A normal vowel form.
    V(VowelForm),

    /// A consonant form starting with h-, w-, or y-.
    H(HForm),

    /// A numeral form.
    N(NumeralForm),

    /// The vowel form -üa-.
    ÜA,

    /// The vowel form -ë-.
    Schwa,

    /// A glottal stop without any corresponding vowels, or a word-final glottal stop.
    GlottalStop,
}

impl Token {
    /// Checks if this token is phonotactically valid in word-initial position.
    pub fn is_valid_word_initial(&self) -> bool {
        match self {
            Self::C(cs) => cs.is_valid_word_initial(),
            Self::N(NumeralForm { integer_part }) => *integer_part < 16,
            _ => true,
        }
    }

    /// Checks if this token is phonotactically valid in word-final position.
    pub fn is_valid_word_final(&self) -> bool {
        match self {
            Self::C(cs) => cs.is_valid_word_final(),
            Self::N(NumeralForm { integer_part }) => *integer_part < 16,
            _ => true,
        }
    }
}

impl IntoToken for Token {
    fn into_token(self) -> Token {
        self
    }
}

/// An error returned when an invalid vowel form is parsed.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParseVowelFormError;

impl fmt::Display for ParseVowelFormError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid vowel form")
    }
}

impl Error for ParseVowelFormError {}

impl VowelForm {
    /// Merges a glottal stop into this [`VowelForm`], returning the
    /// [`ParseError::DoublyGlottalizedFormative`] error if two glottal stops are indicated.
    pub fn merge_vcvk_glottal_stop(&mut self, has_glottal_stop: bool) -> Result<(), ParseError> {
        if has_glottal_stop {
            if self.has_glottal_stop {
                Err(ParseError::DoublyGlottalizedFormative)
            } else {
                self.has_glottal_stop = true;
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}

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
            "aa" => (VowelFormSequence::S1, VowelFormDegree::D1),
            "ää" => (VowelFormSequence::S1, VowelFormDegree::D2),
            "ee" => (VowelFormSequence::S1, VowelFormDegree::D3),
            "ii" => (VowelFormSequence::S1, VowelFormDegree::D4),
            "öö" => (VowelFormSequence::S1, VowelFormDegree::D6),
            "oo" => (VowelFormSequence::S1, VowelFormDegree::D7),
            "üü" => (VowelFormSequence::S1, VowelFormDegree::D8),
            "uu" => (VowelFormSequence::S1, VowelFormDegree::D9),

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

            value => {
                eprintln!("{value:?}");
                return Err(ParseVowelFormError);
            }
        };

        Ok(Self {
            has_glottal_stop,
            sequence,
            degree,
        })
    }
}

impl FromToken for VowelForm {
    fn from_token(token: &Token) -> Option<Self> {
        match token {
            Token::V(vx) => Some(*vx),
            _ => None,
        }
    }
}

/// An error returned when an invalid h-form is parsed.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParseHFormError;

impl fmt::Display for ParseHFormError {
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

impl Deref for OwnedConsonantForm {
    type Target = ConsonantForm;

    fn deref(&self) -> &Self::Target {
        ConsonantForm::new(&self.0)
    }
}

impl DerefMut for OwnedConsonantForm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        (&mut self.0[..]).into()
    }
}

impl<T: Into<String>> From<T> for OwnedConsonantForm {
    fn from(value: T) -> Self {
        OwnedConsonantForm(value.into())
    }
}

impl HForm {
    /// The h-form "h".
    pub const H: Self = Self {
        sequence: HFormSequence::S0,
        degree: HFormDegree::D1,
    };

    /// The h-form "hl".
    pub const HL: Self = Self {
        sequence: HFormSequence::S0,
        degree: HFormDegree::D2,
    };

    /// The h-form "hr".
    pub const HR: Self = Self {
        sequence: HFormSequence::S0,
        degree: HFormDegree::D3,
    };

    /// The h-form "hm".
    pub const HM: Self = Self {
        sequence: HFormSequence::S0,
        degree: HFormDegree::D4,
    };

    /// The h-form "hn".
    pub const HN: Self = Self {
        sequence: HFormSequence::S0,
        degree: HFormDegree::D5,
    };

    /// The h-form "hň".
    pub const HŇ: Self = Self {
        sequence: HFormSequence::S0,
        degree: HFormDegree::D6,
    };

    /// The h-form "w".
    pub const W: Self = Self {
        sequence: HFormSequence::SW,
        degree: HFormDegree::D1,
    };

    /// The h-form "y".
    pub const Y: Self = Self {
        sequence: HFormSequence::SY,
        degree: HFormDegree::D1,
    };

    /// The h-form "hw".
    pub const HW: Self = Self {
        sequence: HFormSequence::SW,
        degree: HFormDegree::D2,
    };

    /// The h-form "hrw".
    pub const HRW: Self = Self {
        sequence: HFormSequence::SW,
        degree: HFormDegree::D3,
    };

    /// The h-form "hmw".
    pub const HMW: Self = Self {
        sequence: HFormSequence::SW,
        degree: HFormDegree::D4,
    };

    /// The h-form "hnw".
    pub const HNW: Self = Self {
        sequence: HFormSequence::SW,
        degree: HFormDegree::D5,
    };

    /// The h-form "hňw".
    pub const HŇW: Self = Self {
        sequence: HFormSequence::SW,
        degree: HFormDegree::D6,
    };
}
