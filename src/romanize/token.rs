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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwnedConsonantForm(pub String);

/// A normal vowel form.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VowelForm {
    /// Whether this form has a glottal stop.
    pub has_glottal_stop: bool,

    /// The sequence of this form.
    pub sequence: VowelFormSequence,

    /// The degree of this form.
    pub degree: VowelFormDegree,
}

/// A vowel form consisting of -üa-.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ÜA;

/// A vowel form consisting of -ë-.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Schwa;

/// A consonant form starting with h-, w-, or y-.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HForm {
    /// The sequence of this form.
    pub sequence: HFormSequence,

    /// The degree of this form.
    pub degree: HFormDegree,
}

/// A consonant form consisting of a single "h".
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Hh;

/// A consonant form consisting of "hr".
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Hr;

/// A consonant form that is either "w" or "y".
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WYForm {
    /// A consonant form consisting of a lone W.
    W,

    /// A consonant form consisting of a lone Y.
    Y,
}

/// A numeral form.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive] // TODO: Remove non_exhaustive once we decide how to work with decimals.
pub struct NumeralForm {
    /// The integral part of this numeral.
    pub integer_part: u64,
}

/// A glottal stop without any corresponding vowels, or a word-final glottal stop.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GlottalStop;

/// A generic token.
#[derive(Clone, Debug, PartialEq, Eq)]
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
            Self::H(_) => false,
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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

    /// Gets the string associated with a vowel form.
    pub fn as_str_after(&self, string_before: &str, is_word_final: bool) -> &'static str {
        let data = if string_before.ends_with("w") {
            if self.has_glottal_stop {
                if is_word_final {
                    vowel_sequence::AFTER_W_WORD_FINAL_GLOTTAL_STOP
                } else {
                    vowel_sequence::AFTER_W_GLOTTAL_STOP
                }
            } else {
                vowel_sequence::AFTER_W
            }
        } else if string_before.ends_with("y") {
            if self.has_glottal_stop {
                if is_word_final {
                    vowel_sequence::AFTER_Y_WORD_FINAL_GLOTTAL_STOP
                } else {
                    vowel_sequence::AFTER_Y_GLOTTAL_STOP
                }
            } else {
                vowel_sequence::AFTER_Y
            }
        } else {
            if self.has_glottal_stop {
                if is_word_final {
                    vowel_sequence::STANDARD_WORD_FINAL_GLOTTAL_STOP
                } else {
                    vowel_sequence::STANDARD_GLOTTAL_STOP
                }
            } else {
                vowel_sequence::STANDARD
            }
        };

        data[self.sequence as usize][self.degree as usize]
    }
}

mod vowel_sequence {
    pub const STANDARD: [[&str; 10]; 4] = [
        ["ae", "a", "ä", "e", "i", "ëi", "ö", "o", "ü", "u"],
        ["ea", "ai", "au", "ei", "eu", "ëu", "ou", "oi", "iu", "ui"],
        ["üo", "ia", "ie", "io", "iö", "eë", "uö", "uo", "ue", "ua"],
        ["üö", "ao", "aö", "eo", "ëo", "oë", "öe", "oe", "öa", "oa"],
    ];

    pub const AFTER_W: [[&str; 10]; 4] = [
        ["ae", "a", "ä", "e", "i", "ëi", "ö", "o", "ü", "u"],
        ["ea", "ai", "au", "ei", "eu", "ëu", "ou", "oi", "iu", "ui"],
        ["üo", "ia", "ie", "io", "iö", "eë", "öë", "öä", "ië", "iä"],
        ["üö", "ao", "aö", "eo", "ëo", "oë", "öe", "oe", "öa", "oa"],
    ];

    pub const AFTER_Y: [[&str; 10]; 4] = [
        ["ae", "a", "ä", "e", "i", "ëi", "ö", "o", "ü", "u"],
        ["ea", "ai", "au", "ei", "eu", "ëu", "ou", "oi", "iu", "ui"],
        ["üo", "uä", "uë", "üä", "üë", "eë", "uö", "uo", "ue", "ua"],
        ["üö", "ao", "aö", "eo", "ëo", "oë", "öe", "oe", "öa", "oa"],
    ];

    pub const STANDARD_GLOTTAL_STOP: [[&str; 10]; 4] = [
        ["a'e", "a'", "ä'", "e'", "i'", "ëi'", "ö'", "o'", "ü'", "u'"],
        [
            "e'a", "ai'", "au'", "ei'", "eu'", "ëu'", "ou'", "oi'", "iu'", "ui'",
        ],
        [
            "ü'o", "i'a", "i'e", "i'o", "i'ö", "e'ë", "u'ö", "u'o", "u'e", "u'a",
        ],
        [
            "ü'ö", "a'o", "a'ö", "e'o", "ë'o", "o'ë", "ö'e", "o'e", "ö'a", "o'a",
        ],
    ];

    pub const AFTER_W_GLOTTAL_STOP: [[&str; 10]; 4] = [
        ["a'e", "a'", "ä'", "e'", "i'", "ëi'", "ö'", "o'", "ü'", "u'"],
        [
            "e'a", "ai'", "au'", "ei'", "eu'", "ëu'", "ou'", "oi'", "iu'", "ui'",
        ],
        [
            "ü'o", "i'a", "i'e", "i'o", "i'ö", "e'ë", "ö'ë", "ö'ä", "i'ë", "i'ä",
        ],
        [
            "ü'ö", "a'o", "a'ö", "e'o", "ë'o", "o'ë", "ö'e", "o'e", "ö'a", "o'a",
        ],
    ];

    pub const AFTER_Y_GLOTTAL_STOP: [[&str; 10]; 4] = [
        ["a'e", "a'", "ä'", "e'", "i'", "ëi'", "ö'", "o'", "ü'", "u'"],
        [
            "e'a", "ai'", "au'", "ei'", "eu'", "ëu'", "ou'", "oi'", "iu'", "ui'",
        ],
        [
            "ü'o", "u'ä", "u'ë", "ü'ä", "ü'ë", "e'ë", "u'ö", "u'o", "u'e", "u'a",
        ],
        [
            "ü'ö", "a'o", "a'ö", "e'o", "ë'o", "o'ë", "ö'e", "o'e", "ö'a", "o'a",
        ],
    ];

    pub const STANDARD_WORD_FINAL_GLOTTAL_STOP: [[&str; 10]; 4] = [
        [
            "a'e", "a'a", "ä'ä", "e'e", "i'i", "ë'i", "ö'ö", "o'o", "ü'ü", "u'u",
        ],
        [
            "e'a", "a'i", "a'u", "e'i", "e'u", "ë'u", "o'u", "o'i", "i'u", "u'i",
        ],
        [
            "ü'o", "i'a", "i'e", "i'o", "i'ö", "e'ë", "u'ö", "u'o", "u'e", "u'a",
        ],
        [
            "ü'ö", "a'o", "a'ö", "e'o", "ë'o", "o'ë", "ö'e", "o'e", "ö'a", "o'a",
        ],
    ];

    pub const AFTER_W_WORD_FINAL_GLOTTAL_STOP: [[&str; 10]; 4] = [
        [
            "a'e", "a'a", "ä'ä", "e'e", "i'i", "ë'i", "ö'ö", "o'o", "ü'ü", "u'u",
        ],
        [
            "e'a", "a'i", "a'u", "e'i", "e'u", "ë'u", "o'u", "o'i", "i'u", "u'i",
        ],
        [
            "ü'o", "i'a", "i'e", "i'o", "i'ö", "e'ë", "ö'ë", "ö'ä", "i'ë", "i'ä",
        ],
        [
            "ü'ö", "a'o", "a'ö", "e'o", "ë'o", "o'ë", "ö'e", "o'e", "ö'a", "o'a",
        ],
    ];

    pub const AFTER_Y_WORD_FINAL_GLOTTAL_STOP: [[&str; 10]; 4] = [
        [
            "a'e", "a'a", "ä'ä", "e'e", "i'i", "ë'i", "ö'ö", "o'o", "ü'ü", "u'u",
        ],
        [
            "e'a", "a'i", "a'u", "e'i", "e'u", "ë'u", "o'u", "o'i", "i'u", "u'i",
        ],
        [
            "ü'o", "u'ä", "u'ë", "ü'ä", "ü'ë", "e'ë", "u'ö", "u'o", "u'e", "u'a",
        ],
        [
            "ü'ö", "a'o", "a'ö", "e'o", "ë'o", "o'ë", "ö'e", "o'e", "ö'a", "o'a",
        ],
    ];
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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

    /// The h-form "y".
    pub const Y: Self = Self {
        sequence: HFormSequence::SY,
        degree: HFormDegree::D1,
    };

    /// An alternate form of the h-form "hw".
    pub const HW_ALT: Self = Self {
        sequence: HFormSequence::SY,
        degree: HFormDegree::D2,
    };

    /// An alternate form of the h-form "hrw".
    pub const HRW_ALT: Self = Self {
        sequence: HFormSequence::SY,
        degree: HFormDegree::D3,
    };

    /// An alternate form of the h-form "hmw".
    pub const HMW_ALT: Self = Self {
        sequence: HFormSequence::SY,
        degree: HFormDegree::D4,
    };

    /// An alternate form of the h-form "hnw".
    pub const HNW_ALT: Self = Self {
        sequence: HFormSequence::SY,
        degree: HFormDegree::D5,
    };

    /// An alternate form of the h-form "hňw".
    pub const HŇW_ALT: Self = Self {
        sequence: HFormSequence::SY,
        degree: HFormDegree::D6,
    };

    /// Gets the string form associated with this h-form.
    pub const fn as_str(self) -> &'static str {
        match self {
            HForm::H => "h",
            HForm::HL => "hl",
            HForm::HR => "hr",
            HForm::HM => "hm",
            HForm::HN => "hn",
            HForm::HŇ => "hň",
            HForm::W => "w",
            HForm::Y => "y",
            HForm::HW | HForm::HW_ALT => "hw",
            HForm::HRW | HForm::HRW_ALT => "hrw",
            HForm::HMW | HForm::HMW_ALT => "hmw",
            HForm::HNW | HForm::HNW_ALT => "hnw",
            HForm::HŇW | HForm::HŇW_ALT => "hňw",
        }
    }
}
