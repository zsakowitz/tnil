//! Contains types and traits used to parse Ithkuil from token streams.

use super::{
    flags::FromTokenFlags,
    token::{ConsonantForm, Token},
    traits::{FromToken, FromTokens},
};
use crate::category::Stress;
use std::{error::Error, fmt};

/// A stream of tokens used to facilitate constructing parsers.
#[derive(Clone, Debug)]
pub struct TokenStream<'a> {
    /// The referenced stream of tokens.
    pub(super) tokens: &'a [Token],

    /// The stress of the referenced list.
    pub(super) stress: Option<Stress>,

    /// The cursor index into the list from the front.
    pub(super) start: usize,

    /// The cursor index into the list from the back.
    pub(super) end: usize,
}

impl<'a> TokenStream<'a> {
    /// Checks if this [`TokenStream`] has no tokens left to parse.
    pub const fn is_done(&self) -> bool {
        self.start >= self.end
    }

    /// Gets the stress of this [`TokenStream`].
    #[must_use]
    pub const fn stress(&self) -> Option<Stress> {
        self.stress
    }

    /// Returns the next token as a specialized token type.
    #[must_use]
    pub fn next<T: FromToken>(&mut self) -> Option<T> {
        if self.is_done() {
            return None;
        }
        let token = self.tokens.get(self.start)?;
        let token = T::from_token(&token)?;
        self.start += 1;
        Some(token)
    }

    /// Returns the next token as a borrowed [`ConsonantForm`].
    ///
    /// This method cannot be integrated into `.next()` due to issues with lifetimes.
    #[must_use]
    pub fn next_cs(&mut self) -> Option<&ConsonantForm> {
        if self.is_done() {
            return None;
        }
        let token = self.tokens.get(self.start);
        let token = match token {
            Some(Token::C(cs)) => &*cs,
            _ => return None,
        };
        self.start += 1;
        Some(token)
    }

    /// Returns the next token from the end as a specialized token type.
    #[must_use]
    pub fn next_back<T: FromToken>(&mut self) -> Option<T> {
        if self.is_done() {
            return None;
        }
        let token = self.tokens.get(self.end - 1)?;
        let token = T::from_token(&token)?;
        self.end -= 1;
        Some(token)
    }

    /// Returns the next token from the end as a borrowed [`ConsonantForm`].
    ///
    /// This method cannot be integrated into `.next_back()` due to issues with lifetimes.
    #[must_use]
    pub fn next_cs_back(&mut self) -> Option<&ConsonantForm> {
        if self.is_done() {
            return None;
        }
        let token = self.tokens.get(self.end - 1);
        let token = match token {
            Some(Token::C(cs)) => &*cs,
            _ => return None,
        };
        self.end -= 1;
        Some(token)
    }

    /// Returns the next token.
    #[must_use]
    pub fn next_any(&mut self) -> Option<&Token> {
        if self.is_done() {
            return None;
        }
        let token = self.tokens.get(self.start)?;
        self.start += 1;
        Some(token)
    }

    /// Returns the next token from the end.
    #[must_use]
    pub fn next_back_any(&mut self) -> Option<&Token> {
        if self.is_done() {
            return None;
        }
        let token = self.tokens.get(self.end - 1)?;
        self.end -= 1;
        Some(token)
    }

    /// Returns the next token without advancing the stream.
    #[must_use]
    pub fn peek(&mut self) -> Option<&Token> {
        if self.is_done() {
            return None;
        }
        let token = self.tokens.get(self.start)?;
        Some(token)
    }

    /// Returns the next token from the back without advancing the stream.
    #[must_use]
    pub fn peek_back(&mut self) -> Option<&Token> {
        if self.is_done() {
            return None;
        }
        let token = self.tokens.get(self.end - 1)?;
        Some(token)
    }

    /// Parses an item.
    #[must_use]
    pub fn parse<T: FromTokens>(&mut self, flags: FromTokenFlags) -> Result<T, ParseError> {
        T::parse(self, flags)
    }

    /// Parses an item, returning [`ParseError::TooManyTokens`] if it does not consume the entire
    /// stream.
    #[must_use]
    pub fn parse_entire<T: FromTokens>(&mut self, flags: FromTokenFlags) -> Result<T, ParseError> {
        let start = self.start;
        let end = self.end;

        match T::parse_volatile(self, flags) {
            Ok(value) => {
                if self.is_done() {
                    Ok(value)
                } else {
                    self.start = start;
                    self.end = end;
                    Err(ParseError::TooManyTokens)
                }
            }
            Err(error) => {
                self.start = start;
                self.end = end;
                Err(error)
            }
        }
    }

    /// Returns a slice into the remaining tokens.
    #[must_use]
    pub fn remaining_tokens(&self) -> &'a [Token] {
        &self.tokens[self.start..self.end]
    }
}

macro_rules! parse_error_defn {
    (match self { $($variant:ident => $message:literal,)+ }) => {
        ::paste::paste! {
            /// The error type returned when an item cannot be parsed.
            #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
            #[non_exhaustive]
            pub enum ParseError {
                $(
                    #[doc = "The error \"" $message "\"."]
                    $variant
                ),+
            }
        }

        impl fmt::Display for ParseError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(match self {
                    $(Self::$variant => $message,)+
                })
            }
        }

        impl Error for ParseError {}
    };
}

parse_error_defn!(match self {
    StressInvalid => "stress is not ((ante)pen)ultimate",
    StressDoubled => "stress is marked twice",

    SourceCharInvalid => "only consonants, vowels, and numbers can be parsed",
    SourceVowelInvalid => "a vowel form could not be parsed",
    SourceHFormInvalid => "a consonant form starting with w, y, or h could not be parsed",
    SourceNumeralInvalid => "a numeric form could not be parsed",

    ExpectedCa => "expected an ungeminated Ca form (e.g. r, lňn, řţgw)",
    ExpectedCaGeminated => "expected a geminated Ca form (e.g. rr, lňňn, řţţgw)",
    ExpectedCb => "expected Cb bias (e.g. pļļ, lçp, kšš)",
    ExpectedCc => "expected Cc form (e.g. w, h, hn)",
    ExpectedCn => "expected Cn mood/case-scope (e.g. h, hm, hňw)",
    ExpectedCm => "expected Cm form (n/ň)",
    ExpectedCp => "expected Cp suppletive adjunct mode (hl/hm/hn/hň)",
    ExpectedCs => "expected Cs affix form (e.g. t, kb, ltř)",
    ExpectedCy => "expected Cy mood/case-scope adjunct vowel (e.g. a, oi, iu)",
    ExpectedCz => "expected Cz multiple-affix adjunct scope (h/’h/’hl/’hr/’hw/’hw)",
    ExpectedHh => "expected single ‘h’ at the beginning of a register",
    ExpectedHr => "expected ‘hr’ at the beginning of a mood/case-scope adjunct",
    ExpectedNn => "expected Nn numeric form (e.g. 4, 23, 7832)",
    ExpectedVc => "expected Vc case form (e.g. ü, ai, io)",
    ExpectedVc2 => "expected Vc2 combination referential second case (e.g. o, ei, üa)",
    ExpectedVh => "expected Vh modular adjunct scope (a/e/i/o/u)",
    ExpectedVk => "expected Vk illocution/validation (e.g. á, éi, óu)",
    ExpectedVm => "expected Vm register type (e.g. a, o, ei)",
    ExpectedVn => "expected Vn form (e.g. a, ou, ie)",
    ExpectedVp => "expected Vp parsing adjunct type (a/e/o/u)",
    ExpectedVr => "expected Vr form (e.g. a, ou, ie)",
    ExpectedVs => "expected Vs single-affix adjunct scope (a/u/e/i/o/ö)",
    ExpectedVv => "expected Vv form (e.g. i, ui, ae, oë)",
    ExpectedVz => "expected Vz multiple-affix adjunct scope (a/u/e/i/o/ö)",
    ExpectedVx => "expected Vx form (e.g. a, ou, ie)",

    ExpectedCsOrVx => "expected Cs or Vx form (e.g. a, kb, ie)",
    ExpectedGlottalStop => "expected word-final glottal stop",
    ExpectedNonDefaultCn => "expected non-default Cn mood/case-scope (e.g. hm, hň)",
    ExpectedNonNumericRoot => "expected non-numeric formative root",
    ExpectedReferentSpecification => "expected combination referential specification (x/xt/xp/xx)",
    ExpectedReferentialRoot => "expected referential formative root (e.g. l, sm, ňzl)",
    ExpectedRoot => "expected formative root",
    ExpectedSuppletiveReferential => "expected suppletive referential head (a/üo)",
    ExpectedWYSpecification => "expected w/y/x/xt/xp/xx to follow referent",

    GlottalizedVc => "Vc forms cannot have glottal stops in concatenated formatives",
    GlottalizedVh => "Vh modular adjunct scopes cannot have glottal stops",
    GlottalizedVk => "Vk illocution/validation forms cannot have glottal stops",
    GlottalizedVn => "Vn forms cannot have glottal stops except in formatives",
    GlottalizedVs => "Vs single-affix adjunct scopes cannot have glottal stops",
    GlottalizedVz => "Vz multiple-affix adjunct scopes cannot have glottal stops",
    GlottalizedVx => "Vx forms cannot have glottal stops except in formatives",

    AffixualFormativeWithCaShortcut => "affixual formatives cannot have Ca shortcuts",
    AspectualCnShortcut => "Cn shortcuts cannot indicate an aspectual Vn",
    AntepenultimateStress => "only unconcatenated formatives can have antepenultimate stress",
    ComboReferentialWithSchwa => "combination referential cannot have ë mid-word",
    DefaultCnShortcut => "Cn shortcuts cannot indicate default FAC/CCN mood/case-scope",
    DoublyGlottalizedVx => "only one Vx form in a formative may have a glottal stop",
    DoublyGlottalizedFormative =>
        "only one form from slots IV-IX in a formative may have a glottal stop",
    GeminatedCs => "unexpected geminated affix Cs form",
    InvalidFormative => "converting from a general formative to a specific formative failed",
    MultipleEndOfSlotVMarkers => "expected only one end-of-slot-V marker",
    ReferentEmpty => "expected at least one referent",
    ReferentExpected => "expected a referent (e.g. l, g, ňň)",
    ReferentInvalid => "invalid referent list",
    TooFewSlotVAffixes => "not enough slot V affixes (indicated by glottal stop in Vv)",
    TooManySlotVAffixes => "too many slot V affixes (indicated by absence of a glottal stop in Vv)",
    TooManyTokens => "expected end of word, found more tokens",

    WordEmpty => "cannot parse nothing",
    WordInitialGlottalStop => "words cannot begin with glottal stops not followed by vowels",
    WordInitialÜA => "words cannot begin with -üa-",

    Unknown => "an unknown error occurred",
});
