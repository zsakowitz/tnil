//! Contains functions for basic parsing of Ithkuil text.
//!
//! There are four functions defined in this module, and they should be called in the order
//! specified below to ensure correct results.
//!
//! 1. [`normalize`]: Normalizes the input string, turning alternate letters such as ṭ into ţ and
//!    non-canonical Unicode representations such as the dual-character e + ´ into é.
//!
//! 2. [`detect_stress`]: Detects the stress marked in the word, returning a [`Result`] wrapping the
//!    detected stress or returning an [`Err`] if the stress is incorrectly marked.
//!
//! 3. [`unstress_vowels`]: Replaces stressed vowel such as á or û into their non-stressed forms,
//!    such as a or ü.
//!
//! 4. [`tokenize`]: Takes a word containing no stress markings and turns it into a list of tokens,
//!    that is, a list of consonant, vowel, h-, and numeric forms.
//!
//! None of these functions are hyper-optimized. The replacement ones in particular ([`normalize`]
//! and [`unstress_vowels`]) could definitely be implemented more efficiently. But it's good enough,
//! and replacing text isn't the slow part of the program anyway.

use crate::{
    category::Stress,
    romanize::token::{NumeralForm, OwnedConsonantForm, Schwa, Token, ÜA},
};
use std::{
    error::Error,
    fmt::{self, Display},
};

/// Normalizes a string into proper New Ithkuil format. This means consolidating extending
/// diacritics into single letters and turning allomorphs such as ṭ into their actual letters.
pub fn normalize(word: &str) -> String {
    let word = word
        .replace('​', "")
        .replace("á", "á")
        .replace("ä", "ä")
        .replace("â", "â")
        .replace("é", "é")
        .replace("ë", "ë")
        .replace("ê", "ê")
        .replace("ì", "i")
        .replace("í", "í")
        .replace("ó", "ó")
        .replace("ö", "ö")
        .replace("ô", "ô")
        .replace("ù", "u")
        .replace("ú", "ú")
        .replace("ü", "ü")
        .replace("û", "û")
        .replace("č", "č")
        .replace("ç", "ç")
        .replace("ţ", "ţ")
        .replace("ṭ", "ţ")
        .replace("ḍ", "ḑ")
        .replace("ḑ", "ḑ")
        .replace("ḷ", "ļ")
        .replace("ļ", "ļ")
        .replace("š", "š")
        .replace("ž", "ž")
        .replace("ż", "ẓ")
        .replace("ẓ", "ẓ")
        .replace("ň", "ň")
        .replace("ņ", "ň")
        .replace("ṇ", "ň")
        .replace("ř", "ř")
        .replace("ŗ", "ř")
        .replace("r͕", "ř")
        .replace("ṛ", "ř");

    let mut output = String::with_capacity(word.capacity());

    for char in word.chars() {
        output.push(match char {
            '’' => '\'',
            'ʼ' => '\'',
            '‘' => '\'',
            'ì' => 'i',
            'ı' => 'i',
            'ù' => 'u',
            'ṭ' => 'ţ',
            'ŧ' => 'ţ',
            'ț' => 'ţ',
            'ḍ' => 'ḑ',
            'đ' => 'ḑ',
            'ł' => 'ļ',
            'ḷ' => 'ļ',
            'ż' => 'ẓ',
            'ṇ' => 'ň',
            'ṛ' => 'ř',
            'ŗ' => 'ř',
            value => value,
        });
    }

    output
}

/// An error returned when stress is invalid.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum StressError {
    /// An error returned when stress is marked twice.
    DoubleMarked,

    /// An error returned when a stress is marked other than ((ante)pen)ultimate.
    TooEarly,
}

impl Display for StressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DoubleMarked => f.write_str("stress should only be marked once"),
            Self::TooEarly => f.write_str("only ((ante)pen)ultimate stress is valid"),
        }
    }
}

impl Error for StressError {}

/// Detects the stress in a word, returning it. If no vowel form is accented, [`None`] is returned
/// instead of a definite stress marker. If two vowel forms are stressed, the [`Err`] variant is
/// returned.
///
/// The input is assumed to be [`normalize`]d.
pub fn detect_stress(word: &str) -> Result<Option<Stress>, StressError> {
    enum LastVowel {
        None,
        I,
        U,
    }

    let mut vowel_forms_detected = 0;
    let mut last_vowel = LastVowel::None;
    let mut stress = None;

    enum VowelStatus {
        NonVowel,
        Unstressed,
        Stressed,
        UnstressedAfterDipthong,
        StressedAfterDipthong,
    }

    for char in word.chars().rev() {
        let status = match (char, last_vowel) {
            ('a' | 'e' | 'o' | 'u', LastVowel::I) => {
                last_vowel = LastVowel::None;
                VowelStatus::UnstressedAfterDipthong
            }
            ('á' | 'é' | 'ó' | 'ú', LastVowel::I) => {
                last_vowel = LastVowel::None;
                VowelStatus::StressedAfterDipthong
            }
            ('a' | 'e' | 'o' | 'i', LastVowel::U) => {
                last_vowel = LastVowel::None;
                VowelStatus::UnstressedAfterDipthong
            }
            ('á' | 'é' | 'ó' | 'í', LastVowel::U) => {
                last_vowel = LastVowel::None;
                VowelStatus::StressedAfterDipthong
            }
            ('i', _) => {
                last_vowel = LastVowel::I;
                VowelStatus::Unstressed
            }
            ('í', _) => {
                last_vowel = LastVowel::I;
                VowelStatus::Stressed
            }
            ('u', _) => {
                last_vowel = LastVowel::U;
                VowelStatus::Unstressed
            }
            ('ú', _) => {
                last_vowel = LastVowel::U;
                VowelStatus::Stressed
            }
            ('a' | 'ä' | 'e' | 'ë' | 'o' | 'ö' | 'ü', _) => {
                last_vowel = LastVowel::None;
                VowelStatus::Unstressed
            }
            ('á' | 'â' | 'é' | 'ê' | 'ó' | 'ô' | 'û', _) => {
                last_vowel = LastVowel::None;
                VowelStatus::Stressed
            }
            (_, _) => {
                last_vowel = LastVowel::None;
                VowelStatus::NonVowel
            }
        };

        match status {
            VowelStatus::NonVowel => {}

            VowelStatus::Unstressed => {
                vowel_forms_detected += 1;
            }

            VowelStatus::Stressed => {
                vowel_forms_detected += 1;
                if stress.is_none() {
                    return Err(StressError::DoubleMarked);
                }
                stress = match vowel_forms_detected {
                    1 => Some(Stress::Ultimate),
                    2 => Some(Stress::Penultimate),
                    3 => Some(Stress::Antepenultimate),
                    _ => return Err(StressError::TooEarly),
                }
            }

            VowelStatus::UnstressedAfterDipthong => {}

            VowelStatus::StressedAfterDipthong => {
                if stress.is_none() {
                    return Err(StressError::DoubleMarked);
                }
                stress = match vowel_forms_detected {
                    1 => Some(Stress::Ultimate),
                    2 => Some(Stress::Penultimate),
                    3 => Some(Stress::Antepenultimate),
                    _ => return Err(StressError::TooEarly),
                }
            }
        };
    }

    Ok(stress)
}

/// Replaces stressed vowels with their unstressed counterparts.
pub fn unstress_vowels(word: &str) -> String {
    word.replace("á", "a")
        .replace("â", "ä")
        .replace("é", "e")
        .replace("ê", "ë")
        .replace("í", "i")
        .replace("ó", "o")
        .replace("ô", "ö")
        .replace("ú", "u")
        .replace("û", "ü")
}

/// An error returned when a word cannot be tokenized.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenizeWordError {
    /// An error returned when a character is neither a consonant, vowel, numeral, `_`, or `.`.
    InvalidChar(char),

    /// An error returned when a vowel form is invalid.
    InvalidVowelForm,

    /// An error returned when an h-form is invalid.
    InvalidHForm,

    /// An error returned when a numeral sequence is invalid.
    InvalidNumber,
}

impl Display for TokenizeWordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidChar(char) => {
                write!(f, "the character {char:?} is not valid in New Ithkuil")
            }
            Self::InvalidVowelForm => write!(f, "invalid vowel form"),
            Self::InvalidHForm => write!(f, "invalid h-form"),
            Self::InvalidNumber => write!(f, "invalid numeric literal"),
        }
    }
}

impl Error for TokenizeWordError {}

/// Turns a word into a sequence of tokens. Underscores are assumed to be consonants, and may be
/// used to force the positioning of certain cores and extensions in the script. For example,
/// "malëuţřait" will by default place "ţ" as the core of an affix with a "ř" bottom extension, but
/// "malëuţř_ait" will place "ţ" as the top extension and "ř" as the core.
///
/// If the input word has a final glottal stop, it will be turned in a [`Token::GlottalStop`] at the
/// end of the [`Vec<Token>`].
///
/// The input is assumed to be [`normalize`]d and have no stress markings (e.g. "walá" is invalid
/// input and will likely throw an error).
pub fn tokenize(word: &str) -> Result<Vec<Token>, TokenizeWordError> {
    let (word, has_word_final_glottal_stop) = match word.strip_suffix('\'') {
        Some(value) => (value, true),
        None => (word, false),
    };

    let word = word.to_owned();

    #[derive(Clone, Copy)]
    enum CurrentToken {
        None,
        C,
        V,
        N,
    }

    let mut tokens = Vec::new();
    let mut current_token: CurrentToken = CurrentToken::None;
    let mut current = String::new();

    macro_rules! push_current_token {
        () => {
            if !current.is_empty() {
                match current_token {
                    CurrentToken::None => {
                        unreachable!("tokens were parsed without setting a corresponding type");
                    }

                    CurrentToken::C => {
                        if current.starts_with(['h', 'w', 'y']) {
                            tokens.push(Token::H(match current.parse() {
                                Ok(h_form) => h_form,
                                Err(_) => return Err(TokenizeWordError::InvalidHForm),
                            }));
                        } else {
                            tokens.push(Token::Consonant(OwnedConsonantForm(current)));
                        }
                    }

                    CurrentToken::V => tokens.push(match &current[..] {
                        "'" => Token::GlottalStop,
                        "ë" => Token::Schwa(Schwa),
                        "üa" => Token::ÜA(ÜA),
                        vowel_form => Token::Vowel(match vowel_form.parse() {
                            Ok(vowel_form) => vowel_form,
                            Err(_) => return Err(TokenizeWordError::InvalidVowelForm),
                        }),
                    }),

                    // TODO: add decimal support
                    CurrentToken::N => match current.parse() {
                        Ok(value) => tokens.push(Token::Numeral(NumeralForm {
                            integer_part: value,
                        })),

                        Err(_) => return Err(TokenizeWordError::InvalidNumber),
                    },
                }
            }
        };
    }

    for char in word.chars() {
        match char {
            'b' | 'c' | 'ç' | 'č' | 'd' | 'ḑ' | 'f' | 'g' | 'h' | 'j' | 'k' | 'l' | 'ļ' | 'm'
            | 'n' | 'ň' | 'p' | 'r' | 'ř' | 's' | 'š' | 't' | 'ţ' | 'v' | 'w' | 'x' | 'y' | 'z'
            | 'ẓ' | 'ž' | '_' => {
                if matches!(current_token, CurrentToken::C) {
                    current.push(char);
                } else {
                    push_current_token!();
                    current_token = CurrentToken::C;
                    current = char.to_string();
                }
            }

            'a' | 'ä' | 'e' | 'ë' | 'i' | 'o' | 'ö' | 'u' | 'ü' | '\'' => {
                if matches!(current_token, CurrentToken::V) {
                    current.push(char);
                } else {
                    push_current_token!();
                    current_token = CurrentToken::V;
                    current = char.to_string();
                }
            }

            '0'..='9' | '.' => {
                if matches!(current_token, CurrentToken::N) {
                    current.push(char);
                } else {
                    push_current_token!();
                    current_token = CurrentToken::N;
                    current = char.to_string();
                }
            }

            _ => return Err(TokenizeWordError::InvalidChar(char)),
        }
    }

    push_current_token!();

    if has_word_final_glottal_stop {
        tokens.push(Token::GlottalStop);
    }

    Ok(tokens)
}
