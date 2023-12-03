//! Implements [`TokenType`] and [`FromTokenStream`] for many types, and provides wrappers on
//! sequences of types (such as a VnCn pair) to aid with parsing them.

use super::{
    stream::TokenStream,
    stream::{FromTokenStream, ParseError, TokenType},
    token::{
        GlottalStop, HForm, Hh, Hr, NumeralForm, OwnedConsonantForm, Schwa, Token, VowelForm,
        WYForm, ÜA,
    },
};
use crate::{
    affix::RegularAffix,
    category::{
        AffixualAdjunctScope, ArbitraryMoodOrCaseScope, Aspect, Bias, Case, CaseScope, HFormDegree,
        HFormSequence, ModularAdjunctMode, ModularAdjunctScope, Mood, MoodOrCaseScope,
        NonAspectualVn, Register, Stress, SuppletiveAdjunctMode, Vn, VowelFormDegree,
        VowelFormSequence,
    },
};

impl TokenType for OwnedConsonantForm {
    fn parse(token: &Token) -> Option<Self> {
        match token {
            Token::Consonant(value) => Some(value.clone()),
            _ => None,
        }
    }
}

impl TokenType for VowelForm {
    fn parse(token: &Token) -> Option<Self> {
        match token {
            Token::Vowel(value) => Some(*value),
            _ => None,
        }
    }
}

impl TokenType for ÜA {
    fn parse(token: &Token) -> Option<Self> {
        match token {
            Token::ÜA(value) => Some(*value),
            _ => None,
        }
    }
}

impl TokenType for Schwa {
    fn parse(token: &Token) -> Option<Self> {
        match token {
            Token::Schwa(value) => Some(*value),
            _ => None,
        }
    }
}

impl TokenType for HForm {
    fn parse(token: &Token) -> Option<Self> {
        match token {
            Token::H(value) => Some(*value),
            _ => None,
        }
    }
}

impl TokenType for WYForm {
    fn parse(token: &Token) -> Option<Self> {
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

impl TokenType for NumeralForm {
    fn parse(token: &Token) -> Option<Self> {
        match token {
            Token::Numeral(value) => Some(*value),
            _ => None,
        }
    }
}

impl TokenType for GlottalStop {
    fn parse(token: &Token) -> Option<Self> {
        match token {
            Token::GlottalStop => Some(Self),
            _ => None,
        }
    }
}

impl FromTokenStream for Hh {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        match stream.next_any() {
            Some(Token::H(HForm {
                sequence: HFormSequence::S0,
                degree: HFormDegree::D1,
            })) => Ok(Self),
            _ => Err(ParseError::ExpectedHh),
        }
    }
}

impl FromTokenStream for Hr {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        match stream.next_any() {
            Some(Token::H(HForm {
                sequence: HFormSequence::S0,
                degree: HFormDegree::D3,
            })) => Ok(Self),
            _ => Err(ParseError::ExpectedHr),
        }
    }
}

impl FromTokenStream for Bias {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        match stream.next_any() {
            Some(Token::Consonant(value)) => match value.0.parse() {
                Ok(value) => Ok(value),
                Err(_) => Err(ParseError::ExpectedCb),
            },
            _ => Err(ParseError::ExpectedCb),
        }
    }
}

impl FromTokenStream for SuppletiveAdjunctMode {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
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

impl FromTokenStream for Case {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        match stream.next_any() {
            Some(Token::Vowel(VowelForm {
                has_glottal_stop,
                sequence,
                degree,
            })) if *degree != VowelFormDegree::D0 => {
                let degree = *degree as u8 - 1;
                let sequence = 9 * (*sequence as u8);
                let shift = 36 * (*has_glottal_stop as u8);

                match Case::from_variant(shift + sequence + degree) {
                    Some(value) => Ok(value),
                    None => Err(ParseError::ExpectedVc),
                }
            }

            _ => Err(ParseError::ExpectedVc),
        }
    }
}

impl FromTokenStream for Register {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        match stream.next_any() {
            Some(Token::Vowel(VowelForm {
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

impl FromTokenStream for Stress {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        match stream.next_any() {
            Some(Token::Vowel(VowelForm {
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

impl FromTokenStream for MoodOrCaseScope {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        use VowelFormDegree as D;
        use VowelFormSequence as S;

        match stream.next_any() {
            Some(Token::Vowel(VowelForm {
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

impl FromTokenStream for NumeralForm {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        match stream.next() {
            Some(value) => Ok(value),
            _ => Err(ParseError::ExpectedNn),
        }
    }
}

impl FromTokenStream for GlottalStop {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        match stream.next() {
            Some(value) => Ok(value),
            _ => Err(ParseError::ExpectedGs),
        }
    }
}

impl FromTokenStream for ModularAdjunctMode {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        match stream.next() {
            Some(WYForm::W) => Ok(ModularAdjunctMode::Parent),
            Some(WYForm::Y) => Ok(ModularAdjunctMode::Concatenated),
            None => Ok(ModularAdjunctMode::Full),
        }
    }
}

impl FromTokenStream for NonAspectualVn {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        let vn: VowelForm = stream.next().ok_or(ParseError::ExpectedVn)?;
        if vn.has_glottal_stop {
            return Err(ParseError::GlottalizedVn);
        }
        Self::from_vowel_form(vn).ok_or(ParseError::ExpectedVn)
    }
}

impl FromTokenStream for Aspect {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        let vn: VowelForm = stream.next().ok_or(ParseError::ExpectedVn)?;
        if vn.has_glottal_stop {
            return Err(ParseError::GlottalizedVn);
        }
        Self::from_vowel_form(vn).ok_or(ParseError::ExpectedVn)
    }
}

impl FromTokenStream for ModularAdjunctScope {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        let Some(VowelForm {
            has_glottal_stop: false,
            sequence: VowelFormSequence::S1,
            degree,
        }) = stream.next()
        else {
            return Err(ParseError::ExpectedVh);
        };

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

impl FromTokenStream for Cn {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
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

impl FromTokenStream for Cm {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        match stream.next_any() {
            Some(Token::Consonant(OwnedConsonantForm(source))) => match &source[..] {
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

impl FromTokenStream for VnCn {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        let vn: VowelForm = stream.next().ok_or(ParseError::ExpectedVn)?;

        if vn.has_glottal_stop {
            return Err(ParseError::GlottalizedVn);
        }

        let cn: Cn = stream.parse()?;

        Ok(Self {
            vn: Vn::from_vowel_form(vn, cn.is_aspect).ok_or(ParseError::ExpectedVn)?,
            cn: cn.mcs,
        })
    }
}

/// A VnCm pair.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VnCm {
    /// The Vn of this pair.
    pub vn: Vn,
}

impl FromTokenStream for VnCm {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        let vn: VowelForm = stream.next().ok_or(ParseError::ExpectedVn)?;

        if vn.has_glottal_stop {
            return Err(ParseError::GlottalizedVn);
        }

        let cm: Cm = stream.parse()?;

        Ok(Self {
            vn: Vn::from_vowel_form(vn, cm.is_aspect).ok_or(ParseError::ExpectedVn)?,
        })
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

impl FromTokenStream for VnCnWithGlottalStop {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        let vn: VowelForm = stream.next().ok_or(ParseError::ExpectedVn)?;

        let cn: Cn = stream.parse()?;

        Ok(Self {
            vn: Vn::from_vowel_form(vn, cn.is_aspect).ok_or(ParseError::ExpectedVn)?,
            has_glottal_stop: vn.has_glottal_stop,
            cn: cn.mcs,
        })
    }
}

/// A VxCs pair with no glottal stop.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VxCs {
    /// The affix represented by this VxCs form.
    pub affix: RegularAffix,
}

impl FromTokenStream for VxCs {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        let vx: VowelForm = stream.next().ok_or(ParseError::ExpectedVx)?;
        if vx.has_glottal_stop {
            return Err(ParseError::GlottalizedVx);
        }
        let cs: OwnedConsonantForm = stream.next().ok_or(ParseError::ExpectedCs)?;
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

impl FromTokenStream for VxCsWithGlottalStop {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        let vx: VowelForm = stream.next().ok_or(ParseError::ExpectedVx)?;
        if vx.has_glottal_stop {
            return Err(ParseError::GlottalizedVx);
        }
        let cs: OwnedConsonantForm = stream.next().ok_or(ParseError::ExpectedCs)?;
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

impl FromTokenStream for CsVxWithGlottalStop {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        let cs: OwnedConsonantForm = stream.next().ok_or(ParseError::ExpectedCs)?;
        let vx: VowelForm = stream.next().ok_or(ParseError::ExpectedVx)?;
        if vx.has_glottal_stop {
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

impl FromTokenStream for Vs {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        let Some(vowel_form): Option<VowelForm> = stream.next() else {
            return Ok(Vs {
                scope: AffixualAdjunctScope::VDom,
            });
        };

        if vowel_form.has_glottal_stop {
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

/// A Vz form.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vz {
    /// The scope marked by this Vz form.
    pub scope: Option<AffixualAdjunctScope>,
}

impl FromTokenStream for Vz {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        let Some(vowel_form): Option<VowelForm> = stream.next() else {
            return Ok(Vz { scope: None });
        };

        if vowel_form.has_glottal_stop {
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

/// A CsVxCz triplet. These must be parsed together because the value of Cz is influenced by whether
/// the Vx form has a glottal stop or not.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CsVxCz {
    /// The affix marked by the CsVx pair.
    pub affix: RegularAffix,

    /// The scope marked by the Cz form.
    pub scope: AffixualAdjunctScope,
}

impl FromTokenStream for CsVxCz {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        let cs: OwnedConsonantForm = stream.next().ok_or(ParseError::ExpectedCs)?;
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
