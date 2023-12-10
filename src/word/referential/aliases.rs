use super::{GeneralReferent, Referential};
use crate::{
    category::{
        Case, Essence, NormalReferentList, Specification, Stress, SuppletiveAdjunctMode,
        VowelFormDegree, VowelFormSequence,
    },
    prelude::{token::WYForm, IntoTokens, TokenList},
    romanize::{
        flags::{FromTokenFlags, IntoTokensFlags},
        segment::{Vc2, VxCs},
        stream::{ParseError, TokenStream},
        token::{HForm, OwnedConsonantForm, Schwa, Token, VowelForm, ÜA},
        traits::{FromTokens, IntoVxCs},
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
                    Some(Token::H(HForm::W | HForm::Y)) if $can_be_single_dual => {
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

macro_rules! into_tokens_impl {
    (
        $ty:ident,
        $list:ident,
        $flags:ident,
        $vowels:ident,
        $referent:ident,
        $essence:ident,
        $second_case:ident,
        { $($single_stmt:stmt)* },
        { $($dual_stmt:stmt)* },
        { $($combo_stmt:stmt)* }
    ) => {
        #[allow(redundant_semicolons)]
        impl IntoTokens for $ty {
            fn append_tokens_to(&self, $list: &mut TokenList, $flags: IntoTokensFlags) {
                match self {
                    Self::Single {
                        $referent,
                        first_case,
                        $second_case,
                        $essence,
                    } => {
                        $($single_stmt)*
                        $list.push(*first_case);
                        if let Some($second_case) = *$second_case {
                            $list.push(WYForm::W);
                            $list.push($second_case);
                        }
                        $list.set_stress(match $essence {
                            Essence::NRM => Stress::Penultimate,
                            Essence::RPV => Stress::Ultimate,
                        });
                    }

                    Self::Dual {
                        first_referent: $referent,
                        first_case,
                        $second_case,
                        second_referent,
                        $essence,
                    } => {
                        $($dual_stmt)*
                        $list.push(*first_case);
                        $list.push(WYForm::W);
                        $list.push(*$second_case);
                        let cs2 = OwnedConsonantForm(second_referent.to_string());
                        let should_add_schwa = !cs2.is_valid_word_final();
                        $list.push(cs2);
                        if should_add_schwa {
                            $list.push(Schwa);
                        }
                        $list.set_stress(match $essence {
                            Essence::NRM => Stress::Penultimate,
                            Essence::RPV => Stress::Ultimate,
                        });
                    }

                    Self::Combination {
                        $referent,
                        first_case,
                        specification,
                        affixes,
                        $second_case,
                        $essence,
                    } => {
                        $($combo_stmt)*
                        $list.push(*first_case);
                        $vowels += 1;
                        $list.push(OwnedConsonantForm(
                            match specification {
                                Specification::BSC => "x",
                                Specification::CTE => "xt",
                                Specification::CSV => "xp",
                                Specification::OBJ => "xx",
                            }
                            .to_owned(),
                        ));
                        let mut is_last_cs_permitted_word_final = true;
                        for affix in affixes {
                            let (vx, cs) = affix.into_vx_cs();
                            $list.push(vx);
                            $vowels += 1;
                            is_last_cs_permitted_word_final = cs.is_valid_word_final();
                            $list.push(cs);
                        }
                        if let Some($second_case) = *$second_case {
                            if $second_case == Case::THM {
                                $list.push(ÜA);
                            } else {
                                $list.push($second_case);
                            }
                        } else if $flags.matches(IntoTokensFlags::WORD_FINAL_VOWEL)
                            || (*$essence == Essence::RPV && $vowels < 2)
                            || !is_last_cs_permitted_word_final
                        {
                            $list.push(VowelForm::default());
                        }
                        $list.set_stress(match $essence {
                            Essence::NRM => Stress::Penultimate,
                            Essence::RPV => Stress::Ultimate,
                        });
                    }
                }
            }
        }
    };
}

into_tokens_impl!(
    NormalReferential,
    list,
    flags,
    vowels,
    referent,
    essence,
    second_case,
    {
        // TODO: Build referents with interconsonantal schwas.
        let cs = OwnedConsonantForm(referent.to_string());
        if flags.matches(IntoTokensFlags::WORD_INITIAL_VOWEL)
            || !cs.is_valid_word_initial()
            || (*essence == Essence::RPV && second_case.is_none())
        {
            list.push(Schwa);
        }
        list.push(cs);
    },
    {
        // TODO: Build referents with interconsonantal schwas.
        let cs = OwnedConsonantForm(referent.to_string());
        if flags.matches(IntoTokensFlags::WORD_INITIAL_VOWEL) || !cs.is_valid_word_initial() {
            list.push(Schwa);
        }
        list.push(cs);
    },
    {
        let mut vowels = 0;
        let cs = OwnedConsonantForm(referent.to_string());
        if flags.matches(IntoTokensFlags::WORD_INITIAL_VOWEL) || !cs.is_valid_word_initial() {
            list.push(Schwa);
            vowels += 1;
        }
        list.push(cs);
    }
);

into_tokens_impl!(
    SuppletiveReferential,
    list,
    flags,
    vowels,
    referent,
    essence,
    second_case,
    {
        list.push(VowelForm {
            has_glottal_stop: false,
            sequence: VowelFormSequence::S3,
            degree: VowelFormDegree::D0,
        });
        list.push(*referent);
    },
    {
        list.push(VowelForm {
            has_glottal_stop: false,
            sequence: VowelFormSequence::S3,
            degree: VowelFormDegree::D0,
        });
        list.push(*referent);
    },
    {
        list.push(VowelForm::default());
        let mut vowels = 1;
        list.push(*referent);
    }
);

into_tokens_impl!(
    GeneralReferential,
    list,
    flags,
    vowels,
    referent,
    essence,
    second_case,
    {
        match referent {
            GeneralReferent::Normal(referent) => {
                // TODO: Build referents with interconsonantal schwas.
                let cs = OwnedConsonantForm(referent.to_string());
                if flags.matches(IntoTokensFlags::WORD_INITIAL_VOWEL)
                    || !cs.is_valid_word_initial()
                    || (*essence == Essence::RPV && second_case.is_none())
                {
                    list.push(Schwa);
                }
                list.push(cs);
            }
            GeneralReferent::Suppletive(referent) => {
                list.push(VowelForm {
                    has_glottal_stop: false,
                    sequence: VowelFormSequence::S3,
                    degree: VowelFormDegree::D0,
                });
                list.push(*referent);
            }
        }
    },
    {
        match referent {
            GeneralReferent::Normal(referent) => {
                // TODO: Build referents with interconsonantal schwas.
                let cs = OwnedConsonantForm(referent.to_string());
                if flags.matches(IntoTokensFlags::WORD_INITIAL_VOWEL) || !cs.is_valid_word_initial()
                {
                    list.push(Schwa);
                }
                list.push(cs);
            }
            GeneralReferent::Suppletive(referent) => {
                list.push(VowelForm {
                    has_glottal_stop: false,
                    sequence: VowelFormSequence::S3,
                    degree: VowelFormDegree::D0,
                });
                list.push(*referent);
            }
        }
    },
    {
        let mut vowels = 0;

        match referent {
            GeneralReferent::Normal(referent) => {
                let cs = OwnedConsonantForm(referent.to_string());
                if flags.matches(IntoTokensFlags::WORD_INITIAL_VOWEL) || !cs.is_valid_word_initial()
                {
                    list.push(Schwa);
                    vowels += 1;
                }
                list.push(cs);
            }
            GeneralReferent::Suppletive(referent) => {
                list.push(VowelForm::default());
                vowels = 1;
                list.push(*referent);
            }
        }
    }
);
