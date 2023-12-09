//! Implements [`TokenType`] and [`FromTokens`] for many types, and provides wrappers on sequences
//! of types (such as a VnCn pair) to aid with parsing them.

use super::{
    flags::FromTokenFlags,
    stream::ParseError,
    stream::TokenStream,
    token::{
        GlottalStop, HForm, Hh, Hr, NumeralForm, OwnedConsonantForm, Schwa, Token, VowelForm,
        WYForm, ÜA,
    },
    traits::{FromToken, FromTokens, IntoToken, IntoTokens, IntoVowelForm},
};
use crate::{
    affix::RegularAffix,
    category::{
        AffixualAdjunctScope, ArbitraryMoodOrCaseScope, Aspect, Bias, Case, CaseScope, HFormDegree,
        HFormSequence, ModularAdjunctMode, ModularAdjunctScope, Mood, MoodOrCaseScope,
        NonAspectualVn, Register, Stress, SuppletiveAdjunctMode, Vn, VowelFormDegree,
        VowelFormSequence,
    },
    prelude::TokenList,
};

impl FromToken for OwnedConsonantForm {
    fn from_token(token: &Token) -> Option<Self> {
        match token {
            Token::C(value) => Some(value.clone()),
            _ => None,
        }
    }
}

impl IntoToken for OwnedConsonantForm {
    fn into_token(self) -> Token {
        Token::C(self)
    }
}

impl IntoVowelForm for VowelForm {
    fn into_vowel_form(self) -> VowelForm {
        self
    }
}

impl FromToken for ÜA {
    fn from_token(token: &Token) -> Option<Self> {
        match token {
            Token::ÜA => Some(Self),
            _ => None,
        }
    }
}

impl IntoToken for ÜA {
    fn into_token(self) -> Token {
        Token::ÜA
    }
}

impl FromToken for Schwa {
    fn from_token(token: &Token) -> Option<Self> {
        match token {
            Token::Schwa => Some(Self),
            _ => None,
        }
    }
}

impl IntoToken for Schwa {
    fn into_token(self) -> Token {
        Token::Schwa
    }
}

impl FromToken for HForm {
    fn from_token(token: &Token) -> Option<Self> {
        match token {
            Token::H(value) => Some(*value),
            _ => None,
        }
    }
}

impl IntoToken for HForm {
    fn into_token(self) -> Token {
        Token::H(self)
    }
}

impl FromToken for WYForm {
    fn from_token(token: &Token) -> Option<Self> {
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

impl IntoToken for WYForm {
    fn into_token(self) -> Token {
        match self {
            Self::W => Token::H(HForm {
                sequence: HFormSequence::SW,
                degree: HFormDegree::D1,
            }),
            Self::Y => Token::H(HForm {
                sequence: HFormSequence::SY,
                degree: HFormDegree::D1,
            }),
        }
    }
}

impl FromToken for NumeralForm {
    fn from_token(token: &Token) -> Option<Self> {
        match token {
            Token::N(value) => Some(*value),
            _ => None,
        }
    }
}

impl IntoToken for NumeralForm {
    fn into_token(self) -> Token {
        Token::N(self)
    }
}

impl FromToken for GlottalStop {
    fn from_token(token: &Token) -> Option<Self> {
        match token {
            Token::GlottalStop => Some(Self),
            _ => None,
        }
    }
}

impl IntoToken for GlottalStop {
    fn into_token(self) -> Token {
        Token::GlottalStop
    }
}

impl IntoToken for Hh {
    fn into_token(self) -> Token {
        Token::H(HForm {
            sequence: HFormSequence::S0,
            degree: HFormDegree::D1,
        })
    }
}

impl FromTokens for Hh {
    fn parse_volatile(stream: &mut TokenStream, _: FromTokenFlags) -> Result<Self, ParseError> {
        match stream.next_any() {
            Some(Token::H(HForm {
                sequence: HFormSequence::S0,
                degree: HFormDegree::D1,
            })) => Ok(Self),
            _ => Err(ParseError::ExpectedHh),
        }
    }
}

impl IntoToken for Hr {
    fn into_token(self) -> Token {
        Token::H(HForm {
            sequence: HFormSequence::S0,
            degree: HFormDegree::D3,
        })
    }
}

impl FromTokens for Hr {
    fn parse_volatile(stream: &mut TokenStream, _: FromTokenFlags) -> Result<Self, ParseError> {
        match stream.next_any() {
            Some(Token::H(HForm {
                sequence: HFormSequence::S0,
                degree: HFormDegree::D3,
            })) => Ok(Self),
            _ => Err(ParseError::ExpectedHr),
        }
    }
}

impl IntoToken for Bias {
    fn into_token(self) -> Token {
        Token::C(self.as_cb().into())
    }
}

impl FromTokens for Bias {
    fn parse_volatile(stream: &mut TokenStream, _: FromTokenFlags) -> Result<Self, ParseError> {
        match stream.next_any() {
            Some(Token::C(value)) => match value.0.parse() {
                Ok(value) => Ok(value),
                Err(_) => Err(ParseError::ExpectedCb),
            },
            _ => Err(ParseError::ExpectedCb),
        }
    }
}

impl IntoToken for SuppletiveAdjunctMode {
    fn into_token(self) -> Token {
        Token::H(HForm {
            sequence: HFormSequence::S0,
            degree: match self {
                SuppletiveAdjunctMode::CAR => HFormDegree::D2,
                SuppletiveAdjunctMode::QUO => HFormDegree::D4,
                SuppletiveAdjunctMode::NAM => HFormDegree::D5,
                SuppletiveAdjunctMode::PHR => HFormDegree::D6,
            },
        })
    }
}

impl FromTokens for SuppletiveAdjunctMode {
    fn parse_volatile(stream: &mut TokenStream, _: FromTokenFlags) -> Result<Self, ParseError> {
        match stream.next_any() {
            Some(Token::H(HForm {
                sequence: HFormSequence::S0,
                degree,
            })) => match degree {
                HFormDegree::D2 => Ok(SuppletiveAdjunctMode::CAR),
                HFormDegree::D4 => Ok(SuppletiveAdjunctMode::QUO),
                HFormDegree::D5 => Ok(SuppletiveAdjunctMode::NAM),
                HFormDegree::D6 => Ok(SuppletiveAdjunctMode::PHR),

                _ => Err(ParseError::ExpectedCp),
            },

            _ => Err(ParseError::ExpectedCp),
        }
    }
}

impl IntoVowelForm for Case {
    fn into_vowel_form(self) -> VowelForm {
        let value = self as u8;

        VowelForm {
            has_glottal_stop: value >= 36,
            sequence: match value / 9 {
                0 => VowelFormSequence::S1,
                1 => VowelFormSequence::S2,
                2 => VowelFormSequence::S3,
                3 => VowelFormSequence::S4,
                _ => unreachable!(),
            },
            degree: match value % 9 {
                0 => VowelFormDegree::D1,
                1 => VowelFormDegree::D2,
                2 => VowelFormDegree::D3,
                3 => VowelFormDegree::D4,
                4 => VowelFormDegree::D5,
                5 => VowelFormDegree::D6,
                6 => VowelFormDegree::D7,
                7 => VowelFormDegree::D8,
                8 => VowelFormDegree::D9,
                _ => unreachable!(),
            },
        }
    }
}

impl FromTokens for Case {
    fn parse_volatile(
        stream: &mut TokenStream,
        _flags: FromTokenFlags,
    ) -> Result<Self, ParseError> {
        match stream.next_any() {
            Some(Token::V(vc)) => Case::from_vc(*vc),
            _ => Err(ParseError::ExpectedVc),
        }
    }
}

impl IntoVowelForm for Register {
    fn into_vowel_form(self) -> VowelForm {
        match self {
            Register::DSV => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S1,
                degree: VowelFormDegree::D1,
            },
            Register::PNT => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S1,
                degree: VowelFormDegree::D3,
            },
            Register::SPF => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S1,
                degree: VowelFormDegree::D4,
            },
            Register::EXM => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S1,
                degree: VowelFormDegree::D7,
            },
            Register::CGT => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S1,
                degree: VowelFormDegree::D9,
            },
            Register::DSV_END => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S2,
                degree: VowelFormDegree::D1,
            },
            Register::PNT_END => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S2,
                degree: VowelFormDegree::D3,
            },
            Register::SPF_END => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S2,
                degree: VowelFormDegree::D8,
            },
            Register::EXM_END => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S2,
                degree: VowelFormDegree::D7,
            },
            Register::CGT_END => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S2,
                degree: VowelFormDegree::D9,
            },
            Register::END => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S1,
                degree: VowelFormDegree::D8,
            },
        }
    }
}

impl FromTokens for Register {
    fn parse_volatile(stream: &mut TokenStream, _: FromTokenFlags) -> Result<Self, ParseError> {
        match stream.next_any() {
            Some(Token::V(VowelForm {
                has_glottal_stop: false,
                sequence,
                degree,
            })) => match (sequence, degree) {
                (VowelFormSequence::S1, VowelFormDegree::D1) => Ok(Register::DSV),
                (VowelFormSequence::S1, VowelFormDegree::D3) => Ok(Register::PNT),
                (VowelFormSequence::S1, VowelFormDegree::D4) => Ok(Register::SPF),
                (VowelFormSequence::S1, VowelFormDegree::D7) => Ok(Register::EXM),
                (VowelFormSequence::S1, VowelFormDegree::D9) => Ok(Register::CGT),
                (VowelFormSequence::S2, VowelFormDegree::D1) => Ok(Register::DSV_END),
                (VowelFormSequence::S2, VowelFormDegree::D3) => Ok(Register::PNT_END),
                (VowelFormSequence::S2, VowelFormDegree::D8) => Ok(Register::SPF_END),
                (VowelFormSequence::S2, VowelFormDegree::D7) => Ok(Register::EXM_END),
                (VowelFormSequence::S2, VowelFormDegree::D9) => Ok(Register::CGT_END),
                (VowelFormSequence::S1, VowelFormDegree::D8) => Ok(Register::END),
                _ => Err(ParseError::ExpectedVm),
            },
            _ => Err(ParseError::ExpectedVm),
        }
    }
}

impl IntoVowelForm for Stress {
    fn into_vowel_form(self) -> VowelForm {
        VowelForm {
            has_glottal_stop: false,
            sequence: VowelFormSequence::S1,
            degree: match self {
                Stress::Monosyllabic => VowelFormDegree::D1,
                Stress::Ultimate => VowelFormDegree::D3,
                Stress::Penultimate => VowelFormDegree::D7,
                Stress::Antepenultimate => VowelFormDegree::D9,
            },
        }
    }
}

impl FromTokens for Stress {
    fn parse_volatile(stream: &mut TokenStream, _: FromTokenFlags) -> Result<Self, ParseError> {
        match stream.next_any() {
            Some(Token::V(VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S1,
                degree,
            })) => match degree {
                VowelFormDegree::D1 => Ok(Stress::Monosyllabic),
                VowelFormDegree::D3 => Ok(Stress::Ultimate),
                VowelFormDegree::D7 => Ok(Stress::Penultimate),
                VowelFormDegree::D9 => Ok(Stress::Antepenultimate),
                _ => Err(ParseError::ExpectedVp),
            },
            _ => Err(ParseError::ExpectedVp),
        }
    }
}

impl IntoVowelForm for MoodOrCaseScope {
    fn into_vowel_form(self) -> VowelForm {
        match self {
            MoodOrCaseScope::Mood(Mood::FAC) => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S1,
                degree: VowelFormDegree::D1,
            },
            MoodOrCaseScope::Mood(Mood::SUB) => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S1,
                degree: VowelFormDegree::D3,
            },
            MoodOrCaseScope::Mood(Mood::ASM) => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S1,
                degree: VowelFormDegree::D4,
            },
            MoodOrCaseScope::Mood(Mood::SPC) => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S1,
                degree: VowelFormDegree::D7,
            },
            MoodOrCaseScope::Mood(Mood::COU) => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S1,
                degree: VowelFormDegree::D6,
            },
            MoodOrCaseScope::Mood(Mood::HYP) => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S1,
                degree: VowelFormDegree::D9,
            },
            MoodOrCaseScope::CaseScope(CaseScope::CCN) => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S2,
                degree: VowelFormDegree::D1,
            },
            MoodOrCaseScope::CaseScope(CaseScope::CCA) => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S2,
                degree: VowelFormDegree::D3,
            },
            MoodOrCaseScope::CaseScope(CaseScope::CCS) => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S2,
                degree: VowelFormDegree::D8,
            },
            MoodOrCaseScope::CaseScope(CaseScope::CCQ) => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S2,
                degree: VowelFormDegree::D7,
            },
            MoodOrCaseScope::CaseScope(CaseScope::CCP) => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S1,
                degree: VowelFormDegree::D8,
            },
            MoodOrCaseScope::CaseScope(CaseScope::CCV) => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S2,
                degree: VowelFormDegree::D9,
            },
        }
    }
}

impl FromTokens for MoodOrCaseScope {
    fn parse_volatile(stream: &mut TokenStream, _: FromTokenFlags) -> Result<Self, ParseError> {
        use VowelFormDegree as D;
        use VowelFormSequence as S;

        match stream.next_any() {
            Some(Token::V(VowelForm {
                has_glottal_stop: false,
                sequence,
                degree,
            })) => match (sequence, degree) {
                (S::S1, D::D1) => Ok(MoodOrCaseScope::Mood(Mood::FAC)),
                (S::S1, D::D3) => Ok(MoodOrCaseScope::Mood(Mood::SUB)),
                (S::S1, D::D4) => Ok(MoodOrCaseScope::Mood(Mood::ASM)),
                (S::S1, D::D7) => Ok(MoodOrCaseScope::Mood(Mood::SPC)),
                (S::S1, D::D6) => Ok(MoodOrCaseScope::Mood(Mood::COU)),
                (S::S1, D::D9) => Ok(MoodOrCaseScope::Mood(Mood::HYP)),

                (S::S2, D::D1) => Ok(MoodOrCaseScope::CaseScope(CaseScope::CCN)),
                (S::S2, D::D3) => Ok(MoodOrCaseScope::CaseScope(CaseScope::CCA)),
                (S::S2, D::D8) => Ok(MoodOrCaseScope::CaseScope(CaseScope::CCS)),
                (S::S2, D::D7) => Ok(MoodOrCaseScope::CaseScope(CaseScope::CCQ)),
                (S::S1, D::D8) => Ok(MoodOrCaseScope::CaseScope(CaseScope::CCP)),
                (S::S2, D::D9) => Ok(MoodOrCaseScope::CaseScope(CaseScope::CCV)),

                _ => Err(ParseError::ExpectedCn),
            },
            _ => Err(ParseError::ExpectedCn),
        }
    }
}

impl IntoTokens for ModularAdjunctMode {
    fn append_to(&self, list: &mut TokenList) {
        match self {
            ModularAdjunctMode::Parent => list.push(WYForm::W),
            ModularAdjunctMode::Concatenated => list.push(WYForm::Y),
            ModularAdjunctMode::Full => {}
        }
    }
}

impl FromTokens for ModularAdjunctMode {
    fn parse_volatile(stream: &mut TokenStream, _: FromTokenFlags) -> Result<Self, ParseError> {
        match stream.next() {
            Some(WYForm::W) => Ok(ModularAdjunctMode::Parent),
            Some(WYForm::Y) => Ok(ModularAdjunctMode::Concatenated),
            None => Ok(ModularAdjunctMode::Full),
        }
    }
}

impl IntoVowelForm for NonAspectualVn {
    fn into_vowel_form(self) -> VowelForm {
        let (sequence, degree) = match self {
            NonAspectualVn::Valence(value) => (VowelFormSequence::S1, value as u8),
            NonAspectualVn::Phase(value) => (VowelFormSequence::S1, value as u8),
            NonAspectualVn::Effect(value) => (VowelFormSequence::S1, value as u8),
            NonAspectualVn::Level(value) => (VowelFormSequence::S1, value as u8),
        };

        VowelForm {
            has_glottal_stop: false,
            sequence,
            degree: match degree {
                0 => VowelFormDegree::D1,
                1 => VowelFormDegree::D2,
                2 => VowelFormDegree::D3,
                3 => VowelFormDegree::D4,
                4 => VowelFormDegree::D5,
                5 => VowelFormDegree::D6,
                6 => VowelFormDegree::D7,
                7 => VowelFormDegree::D8,
                8 => VowelFormDegree::D9,
                _ => unreachable!(),
            },
        }
    }
}

impl FromTokens for NonAspectualVn {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        let vn: VowelForm = stream.next().ok_or(ParseError::ExpectedVn)?;
        if vn.has_glottal_stop && !flags.matches(FromTokenFlags::PERMISSIVE) {
            return Err(ParseError::GlottalizedVn);
        }
        Self::from_vowel_form(vn).ok_or(ParseError::ExpectedVn)
    }
}

impl IntoVowelForm for Aspect {
    fn into_vowel_form(self) -> VowelForm {
        let value = self as u8;

        VowelForm {
            has_glottal_stop: false,
            sequence: match value / 9 {
                0 => VowelFormSequence::S1,
                1 => VowelFormSequence::S2,
                2 => VowelFormSequence::S3,
                3 => VowelFormSequence::S4,
                _ => unreachable!(),
            },
            degree: match value % 9 {
                0 => VowelFormDegree::D1,
                1 => VowelFormDegree::D2,
                2 => VowelFormDegree::D3,
                3 => VowelFormDegree::D4,
                4 => VowelFormDegree::D5,
                5 => VowelFormDegree::D6,
                6 => VowelFormDegree::D7,
                7 => VowelFormDegree::D8,
                8 => VowelFormDegree::D9,
                _ => unreachable!(),
            },
        }
    }
}

impl FromTokens for Aspect {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        let vn: VowelForm = stream.next().ok_or(ParseError::ExpectedVn)?;
        if vn.has_glottal_stop && !flags.matches(FromTokenFlags::PERMISSIVE) {
            return Err(ParseError::GlottalizedVn);
        }
        Self::from_vowel_form(vn).ok_or(ParseError::ExpectedVn)
    }
}

impl IntoVowelForm for ModularAdjunctScope {
    fn into_vowel_form(self) -> VowelForm {
        VowelForm {
            has_glottal_stop: false,
            sequence: VowelFormSequence::S1,
            degree: match self {
                ModularAdjunctScope::Formative => VowelFormDegree::D1,
                ModularAdjunctScope::MCS => VowelFormDegree::D3,
                ModularAdjunctScope::OverAdj => VowelFormDegree::D4,
                ModularAdjunctScope::UnderAdj => VowelFormDegree::D7,
            },
        }
    }
}

impl FromTokens for ModularAdjunctScope {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        let Some(VowelForm {
            has_glottal_stop,
            sequence: VowelFormSequence::S1,
            degree,
        }) = stream.next()
        else {
            return Err(ParseError::ExpectedVh);
        };

        if has_glottal_stop && !flags.matches(FromTokenFlags::PERMISSIVE) {
            return Err(ParseError::GlottalizedVh);
        }

        match degree {
            VowelFormDegree::D1 => Ok(ModularAdjunctScope::Formative),
            VowelFormDegree::D3 => Ok(ModularAdjunctScope::MCS),
            VowelFormDegree::D4 | VowelFormDegree::D9 => Ok(ModularAdjunctScope::OverAdj),
            VowelFormDegree::D7 => Ok(ModularAdjunctScope::UnderAdj),
            _ => Err(ParseError::ExpectedVh),
        }
    }
}

/// A parsed Cn that is arbitrarily either a mood or a case-scope and marks its corresponding Vn as
/// aspectual or non-aspectual.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cn {
    /// The mood/case-scope of this Cn form.
    pub mcs: ArbitraryMoodOrCaseScope,

    /// Whether the corresponding Vn is an aspect.
    pub is_aspect: bool,
}

impl IntoToken for Cn {
    fn into_token(self) -> Token {
        Token::H(HForm {
            sequence: match self.is_aspect {
                false => HFormSequence::S0,
                true => HFormSequence::SW,
            },
            degree: match self.mcs {
                ArbitraryMoodOrCaseScope::FAC_CCN => HFormDegree::D1,
                ArbitraryMoodOrCaseScope::SUB_CCA => HFormDegree::D2,
                ArbitraryMoodOrCaseScope::ASM_CCS => HFormDegree::D3,
                ArbitraryMoodOrCaseScope::SPC_CCQ => HFormDegree::D4,
                ArbitraryMoodOrCaseScope::COU_CCP => HFormDegree::D5,
                ArbitraryMoodOrCaseScope::HYP_CCV => HFormDegree::D6,
            },
        })
    }
}

impl FromTokens for Cn {
    fn parse_volatile(stream: &mut TokenStream, _: FromTokenFlags) -> Result<Self, ParseError> {
        let cn: HForm = stream.next().ok_or(ParseError::ExpectedCn)?;

        let is_aspect = matches!(cn.sequence, HFormSequence::SW | HFormSequence::SY);

        let mcs = match cn.degree {
            HFormDegree::D1 => ArbitraryMoodOrCaseScope::FAC_CCN,
            HFormDegree::D2 => ArbitraryMoodOrCaseScope::SUB_CCA,
            HFormDegree::D3 => ArbitraryMoodOrCaseScope::ASM_CCS,
            HFormDegree::D4 => ArbitraryMoodOrCaseScope::SPC_CCQ,
            HFormDegree::D5 => ArbitraryMoodOrCaseScope::COU_CCP,
            HFormDegree::D6 => ArbitraryMoodOrCaseScope::HYP_CCV,
        };

        Ok(Self { mcs, is_aspect })
    }
}

/// A parsed Cm that marks its corresponding Vn as aspectual or non-aspectual.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cm {
    /// Whether the corresponding Vn is an aspect.
    pub is_aspect: bool,
}

impl IntoToken for Cm {
    fn into_token(self) -> Token {
        Token::C(OwnedConsonantForm(
            if self.is_aspect { "ň" } else { "n" }.to_owned(),
        ))
    }
}

impl FromTokens for Cm {
    fn parse_volatile(stream: &mut TokenStream, _: FromTokenFlags) -> Result<Self, ParseError> {
        match stream.next_any() {
            Some(Token::C(OwnedConsonantForm(source))) => match &source[..] {
                "n" => Ok(Self { is_aspect: false }),
                "ň" => Ok(Self { is_aspect: true }),
                _ => Err(ParseError::ExpectedCm),
            },
            _ => Err(ParseError::ExpectedCm),
        }
    }
}

/// A VnCn pair.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VnCn {
    /// The Vn of this pair.
    pub vn: Vn,

    /// The Cn of this pair.
    pub cn: ArbitraryMoodOrCaseScope,
}

impl FromTokens for VnCn {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        let vn: VowelForm = stream.next().ok_or(ParseError::ExpectedVn)?;

        if vn.has_glottal_stop && !flags.matches(FromTokenFlags::PERMISSIVE) {
            return Err(ParseError::GlottalizedVn);
        }

        let cn: Cn = stream.parse(flags)?;

        Ok(Self {
            vn: Vn::from_vowel_form(vn, cn.is_aspect).ok_or(ParseError::ExpectedVn)?,
            cn: cn.mcs,
        })
    }
}

impl IntoTokens for VnCn {
    fn append_to(&self, list: &mut TokenList) {
        match self.vn.as_non_aspectual_vn() {
            Ok(non_aspectual) => {
                list.push(non_aspectual);

                list.push(Cn {
                    mcs: self.cn,
                    is_aspect: false,
                });
            }
            Err(aspect) => {
                list.push(aspect);
                list.push(Cn {
                    mcs: self.cn,
                    is_aspect: true,
                });
            }
        }
    }
}

/// A VnCm pair.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VnCm {
    /// The Vn of this pair.
    pub vn: Vn,
}

impl FromTokens for VnCm {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        let vn: VowelForm = stream.next().ok_or(ParseError::ExpectedVn)?;

        if vn.has_glottal_stop && !flags.matches(FromTokenFlags::PERMISSIVE) {
            return Err(ParseError::GlottalizedVn);
        }

        let cm: Cm = stream.parse(flags)?;

        Ok(Self {
            vn: Vn::from_vowel_form(vn, cm.is_aspect).ok_or(ParseError::ExpectedVn)?,
        })
    }
}

impl IntoTokens for VnCm {
    fn append_to(&self, list: &mut TokenList) {
        match self.vn.as_non_aspectual_vn() {
            Ok(non_aspectual) => {
                list.push(non_aspectual);
                list.push(Cm { is_aspect: false });
            }
            Err(aspect) => {
                list.push(aspect);
                list.push(Cm { is_aspect: true });
            }
        }
    }
}

/// A VnCn pair with an optional glottal stop.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VnCnWithGlottalStop {
    /// The Vn of this pair.
    pub vn: Vn,

    /// Whether the Vn form had a glottal stop.
    pub has_glottal_stop: bool,

    /// The Cn of this pair.
    pub cn: ArbitraryMoodOrCaseScope,
}

impl FromTokens for VnCnWithGlottalStop {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        let vn: VowelForm = stream.next().ok_or(ParseError::ExpectedVn)?;

        let cn: Cn = stream.parse(flags)?;

        Ok(Self {
            vn: Vn::from_vowel_form(vn, cn.is_aspect).ok_or(ParseError::ExpectedVn)?,
            has_glottal_stop: vn.has_glottal_stop,
            cn: cn.mcs,
        })
    }
}

impl IntoTokens for VnCnWithGlottalStop {
    fn append_to(&self, list: &mut TokenList) {
        match self.vn.as_non_aspectual_vn() {
            Ok(vn) => {
                let mut vn = vn.into_vowel_form();
                vn.has_glottal_stop = self.has_glottal_stop;
                list.push(vn);
                list.push(Cn {
                    mcs: self.cn,
                    is_aspect: false,
                });
            }
            Err(vn) => {
                let mut vn = vn.into_vowel_form();
                vn.has_glottal_stop = self.has_glottal_stop;
                list.push(vn);
                list.push(Cn {
                    mcs: self.cn,
                    is_aspect: true,
                });
            }
        }
    }
}

/// A VxCs pair with no glottal stop.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VxCs {
    /// The affix represented by this VxCs form.
    pub affix: RegularAffix,
}

impl FromTokens for VxCs {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        let vx: VowelForm = stream.next().ok_or(ParseError::ExpectedVx)?;
        if vx.has_glottal_stop && !flags.matches(FromTokenFlags::PERMISSIVE) {
            return Err(ParseError::GlottalizedVx);
        }
        let cs = stream.next_cs().ok_or(ParseError::ExpectedCs)?;
        Ok(VxCs {
            affix: RegularAffix::from_vxcs(vx, &cs)?,
        })
    }
}

/// A VxCs pair with an optional glottal stop.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VxCsWithGlottalStop {
    /// The affix represented by this VxCs form.
    pub affix: RegularAffix,

    /// Whether the Vx form had a glottal stop.
    pub has_glottal_stop: bool,
}

impl FromTokens for VxCsWithGlottalStop {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        let vx: VowelForm = stream.next().ok_or(ParseError::ExpectedVx)?;
        if vx.has_glottal_stop && !flags.matches(FromTokenFlags::PERMISSIVE) {
            return Err(ParseError::GlottalizedVx);
        }
        let cs = stream.next_cs().ok_or(ParseError::ExpectedCs)?;
        Ok(Self {
            affix: RegularAffix::from_vxcs(vx, &cs)?,
            has_glottal_stop: vx.has_glottal_stop,
        })
    }
}

/// A CsVx pair with an optional glottal stop.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CsVxWithGlottalStop {
    /// The affix represented by this CsVx form.
    pub affix: RegularAffix,

    /// Whether the Vx form had a glottal stop.
    pub has_glottal_stop: bool,
}

impl FromTokens for CsVxWithGlottalStop {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        let cs = stream.next_cs().ok_or(ParseError::ExpectedCs)?.to_owned();
        let vx: VowelForm = stream.next().ok_or(ParseError::ExpectedVx)?;
        if vx.has_glottal_stop && !flags.matches(FromTokenFlags::PERMISSIVE) {
            return Err(ParseError::GlottalizedVx);
        }
        Ok(Self {
            affix: RegularAffix::from_vxcs(vx, &cs)?,
            has_glottal_stop: vx.has_glottal_stop,
        })
    }
}

/// A Vs form.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vs {
    /// The scope marked by this Vs form.
    pub scope: AffixualAdjunctScope,
}

impl FromTokens for Vs {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        let Some(vowel_form): Option<VowelForm> = stream.next() else {
            return Ok(Vs {
                scope: AffixualAdjunctScope::VDom,
            });
        };

        if vowel_form.has_glottal_stop && !flags.matches(FromTokenFlags::PERMISSIVE) {
            return Err(ParseError::GlottalizedVs);
        }

        if vowel_form.sequence != VowelFormSequence::S1 {
            return Err(ParseError::ExpectedVs);
        }

        match vowel_form.degree {
            VowelFormDegree::D1 => Ok(Vs {
                scope: AffixualAdjunctScope::VDom,
            }),
            VowelFormDegree::D9 => Ok(Vs {
                scope: AffixualAdjunctScope::VSub,
            }),
            VowelFormDegree::D3 => Ok(Vs {
                scope: AffixualAdjunctScope::VIIDom,
            }),
            VowelFormDegree::D4 => Ok(Vs {
                scope: AffixualAdjunctScope::VIISub,
            }),
            VowelFormDegree::D7 => Ok(Vs {
                scope: AffixualAdjunctScope::Formative,
            }),
            VowelFormDegree::D6 => Ok(Vs {
                scope: AffixualAdjunctScope::OverAdj,
            }),
            _ => Err(ParseError::ExpectedVs),
        }
    }
}

impl IntoVowelForm for Vs {
    fn into_vowel_form(self) -> VowelForm {
        VowelForm {
            has_glottal_stop: false,
            sequence: VowelFormSequence::S1,
            degree: match self.scope {
                AffixualAdjunctScope::VDom => VowelFormDegree::D1,
                AffixualAdjunctScope::VSub => VowelFormDegree::D9,
                AffixualAdjunctScope::VIIDom => VowelFormDegree::D3,
                AffixualAdjunctScope::VIISub => VowelFormDegree::D4,
                AffixualAdjunctScope::Formative => VowelFormDegree::D7,
                AffixualAdjunctScope::OverAdj => VowelFormDegree::D6,
            },
        }
    }
}

/// A Vz form.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vz {
    /// The scope marked by this Vz form.
    pub scope: Option<AffixualAdjunctScope>,
}

impl FromTokens for Vz {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        let Some(vowel_form): Option<VowelForm> = stream.next() else {
            return Ok(Vz { scope: None });
        };

        if vowel_form.has_glottal_stop && !flags.matches(FromTokenFlags::PERMISSIVE) {
            return Err(ParseError::GlottalizedVz);
        }

        if matches!(
            vowel_form,
            VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S2,
                degree: VowelFormDegree::D1
            }
        ) {
            return Ok(Vz { scope: None });
        }

        if vowel_form.sequence != VowelFormSequence::S1 {
            return Err(ParseError::ExpectedVz);
        }

        match vowel_form.degree {
            VowelFormDegree::D1 => Ok(Vz {
                scope: Some(AffixualAdjunctScope::VDom),
            }),
            VowelFormDegree::D9 => Ok(Vz {
                scope: Some(AffixualAdjunctScope::VSub),
            }),
            VowelFormDegree::D3 => Ok(Vz {
                scope: Some(AffixualAdjunctScope::VIIDom),
            }),
            VowelFormDegree::D4 => Ok(Vz {
                scope: Some(AffixualAdjunctScope::VIISub),
            }),
            VowelFormDegree::D7 => Ok(Vz {
                scope: Some(AffixualAdjunctScope::Formative),
            }),
            VowelFormDegree::D6 => Ok(Vz {
                scope: Some(AffixualAdjunctScope::OverAdj),
            }),
            _ => Err(ParseError::ExpectedVz),
        }
    }
}

impl IntoVowelForm for Vz {
    fn into_vowel_form(self) -> VowelForm {
        match self.scope {
            None => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S2,
                degree: VowelFormDegree::D1,
            },
            Some(AffixualAdjunctScope::VDom) => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S1,
                degree: VowelFormDegree::D1,
            },

            Some(AffixualAdjunctScope::VSub) => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S1,
                degree: VowelFormDegree::D9,
            },

            Some(AffixualAdjunctScope::VIIDom) => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S1,
                degree: VowelFormDegree::D3,
            },

            Some(AffixualAdjunctScope::VIISub) => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S1,
                degree: VowelFormDegree::D4,
            },

            Some(AffixualAdjunctScope::Formative) => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S1,
                degree: VowelFormDegree::D7,
            },

            Some(AffixualAdjunctScope::OverAdj) => VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S1,
                degree: VowelFormDegree::D6,
            },
        }
    }
}

/// A CsVxCz triplet. These must be parsed together because the value of Cz is influenced by whether
/// the Vx form has a glottal stop or not.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CsVxCz {
    /// The affix marked by the CsVx pair.
    pub affix: RegularAffix,

    /// The scope marked by the Cz form.
    pub scope: AffixualAdjunctScope,
}

impl FromTokens for CsVxCz {
    fn parse_volatile(stream: &mut TokenStream, _: FromTokenFlags) -> Result<Self, ParseError> {
        let cs = stream.next_cs().ok_or(ParseError::ExpectedCs)?.to_owned();
        let vx: VowelForm = stream.next().ok_or(ParseError::ExpectedVx)?;
        let cz: HForm = stream.next().ok_or(ParseError::ExpectedCz)?;
        Ok(Self {
            affix: RegularAffix::from_vxcs(vx, &cs)?,
            scope: match vx.has_glottal_stop {
                false => match cz {
                    HForm {
                        sequence: HFormSequence::S0,
                        degree: HFormDegree::D1,
                    } => AffixualAdjunctScope::VDom,
                    HForm {
                        sequence: HFormSequence::SW,
                        degree: HFormDegree::D2,
                    } => AffixualAdjunctScope::Formative,
                    _ => return Err(ParseError::ExpectedCz),
                },
                true => match cz {
                    HForm {
                        sequence: HFormSequence::S0,
                        degree: HFormDegree::D1,
                    } => AffixualAdjunctScope::VSub,
                    HForm {
                        sequence: HFormSequence::S0,
                        degree: HFormDegree::D2,
                    } => AffixualAdjunctScope::VIIDom,
                    HForm {
                        sequence: HFormSequence::S0,
                        degree: HFormDegree::D3,
                    } => AffixualAdjunctScope::VIISub,
                    HForm {
                        sequence: HFormSequence::SW,
                        degree: HFormDegree::D2,
                    } => AffixualAdjunctScope::OverAdj,
                    _ => return Err(ParseError::ExpectedCz),
                },
            },
        })
    }
}

/// A case form found at the end of a combination referential.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vc2 {
    /// The case of this Vc2 form.
    pub case: Option<Case>,
}

impl FromTokens for Vc2 {
    fn parse_volatile(
        stream: &mut TokenStream,
        _flags: FromTokenFlags,
    ) -> Result<Self, ParseError> {
        match stream.next_any() {
            Some(Token::V(VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S1,
                degree: VowelFormDegree::D1,
            })) => Ok(Vc2 { case: None }),

            Some(Token::V(vc)) => Ok(Vc2 {
                case: Some(Case::from_vc(*vc)?),
            }),

            Some(Token::ÜA) => Ok(Vc2 {
                case: Some(Case::THM),
            }),

            None => Ok(Vc2 { case: None }),

            _ => Err(ParseError::ExpectedVc2),
        }
    }
}
