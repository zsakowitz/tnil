use super::{GeneralReferent, Referential};
use crate::{
    category::{
        Essence, HFormDegree, HFormSequence, NormalReferentList, Specification, Stress,
        SuppletiveAdjunctMode, VowelFormDegree, VowelFormSequence,
    },
    romanize::{
        flags::FromTokenFlags,
        segment::{Vc2, VxCs},
        stream::{ParseError, TokenStream},
        token::{HForm, OwnedConsonantForm, Schwa, Token, VowelForm},
        traits::FromTokens,
    },
};

/// A normal referential (i.e. one where the initial referent is an actual referent).
pub type NormalReferential = Referential<NormalReferentList>;

/// A suppletive referential (i.e. one where the initial referent is a suppletive adjunct mode).
pub type SuppletiveReferential = Referential<SuppletiveAdjunctMode>;

/// A general referential.
pub type GeneralReferential = Referential<GeneralReferent>;

macro_rules! from_token_stream_impl {
    (
        $ty:ty,
        $stream:ident,
        $flags:ident,
        $first_referent:ident,
        { $($stmt:stmt)* },
        $can_be_single_dual:expr,
        $can_be_combo:expr,
    ) => {
        impl FromTokens for $ty {
            #[allow(redundant_semicolons)]
            fn parse_volatile(
                $stream: &mut TokenStream,
                $flags: FromTokenFlags,
            ) -> Result<Self, ParseError> {
                $($stmt)*

                let first_case = $stream.parse($flags)?;

                let essence = match $stream.stress() {
                    Some(Stress::Ultimate) => Essence::RPV,
                    Some(Stress::Antepenultimate) => return Err(ParseError::AntepenultimateStress),
                    _ => Essence::NRM,
                };

                match $stream.next_any() {
                    Some(Token::H(HForm {
                        sequence: HFormSequence::SW | HFormSequence::SY,
                        degree: HFormDegree::D1,
                    })) if $can_be_single_dual => {
                        let second_case = $stream.parse($flags)?;

                        match $stream.next::<OwnedConsonantForm>() {
                            Some(cr) => {
                                let _ = $stream.next::<Schwa>();
                                let second_referent = cr.parse()?;

                                Ok(Referential::Dual {
                                    $first_referent,
                                    first_case,
                                    second_case,
                                    second_referent,
                                    essence,
                                })
                            }
                            None => Ok(Referential::Single {
                                referent: $first_referent,
                                first_case,
                                second_case: Some(second_case),
                                essence,
                            }),
                        }
                    }

                    Some(Token::C(OwnedConsonantForm(specification))) if $can_be_combo => {
                        let specification = match &specification[..] {
                            "x" => Specification::BSC,
                            "xt" => Specification::CTE,
                            "xp" => Specification::CSV,
                            "xx" => Specification::OBJ,
                            _ => return Err(ParseError::ExpectedReferentSpecification),
                        };

                        let mut affixes = Vec::new();

                        while let Ok(VxCs { affix }) = $stream.parse($flags) {
                            affixes.push(affix);
                        }

                        let Vc2 { case: second_case } = $stream.parse($flags)?;

                        Ok(Referential::Combination {
                            referent: $first_referent,
                            first_case,
                            specification,
                            affixes,
                            second_case,
                            essence,
                        })
                    }

                    None if $can_be_single_dual => Ok(Referential::Single {
                        referent: $first_referent,
                        first_case,
                        second_case: None,
                        essence,
                    }),

                    _ => Err(ParseError::ExpectedWYSpecification),
                }
            }
        }
    };
}

from_token_stream_impl!(
    NormalReferential,
    stream,
    flags,
    first_referent,
    {
        let _ = stream.next::<Schwa>();

        let mut has_word_mid_schwa = false;

        let mut consonants = {
            let cr = stream.next_cs().ok_or(ParseError::ReferentExpected)?;
            (*cr).to_string()
        };

        while let Some(Schwa) = stream.next() {
            has_word_mid_schwa = true;
            let cr = stream.next_cs().ok_or(ParseError::ReferentExpected)?;
            consonants += &cr;
        }

        let first_referent = consonants.parse()?;
    },
    true,
    !has_word_mid_schwa || flags.matches(FromTokenFlags::PERMISSIVE),
);

from_token_stream_impl!(
    SuppletiveReferential,
    stream,
    flags,
    first_referent,
    {
        let is_combo = match stream.next() {
            Some(VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S1,
                degree: VowelFormDegree::D1,
            }) => false,

            Some(VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S3,
                degree: VowelFormDegree::D0,
            }) => true,

            _ => return Err(ParseError::ExpectedSuppletiveReferential),
        };

        let first_referent = stream.parse(flags)?;
    },
    !is_combo,
    is_combo,
);

from_token_stream_impl!(
    GeneralReferential,
    stream,
    flags,
    first_referent,
    {
        let schwa: Option<Schwa> = stream.next();

        let mut can_be_single_dual = true;
        let mut can_be_combo = true;

        let first_referent = match stream.next_any() {
            Some(Token::C(first)) => {
                let mut consonants = first.0.clone();

                while let Some(Schwa) = stream.next() {
                    if !flags.matches(FromTokenFlags::PERMISSIVE) {
                        can_be_combo = false;
                    }

                    consonants += stream.next_cs().ok_or(ParseError::ReferentExpected)?;
                }

                GeneralReferent::Normal(consonants.parse()?)
            }

            Some(Token::V(VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S1,
                degree: VowelFormDegree::D1,
            })) if schwa.is_none() => {
                can_be_single_dual = false;
                GeneralReferent::Suppletive(stream.parse(flags)?)
            }

            Some(Token::V(VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S3,
                degree: VowelFormDegree::D0,
            })) if schwa.is_none() => {
                can_be_combo = false;
                GeneralReferent::Suppletive(stream.parse(flags)?)
            }

            _ => return Err(ParseError::ExpectedSuppletiveReferential),
        };
    },
    can_be_single_dual,
    can_be_combo,
);
