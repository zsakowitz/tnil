use super::{
    AffixualAdjunct, BiasAdjunct, CheckedFormative, GeneralReferential, MCSAdjunct, ModularAdjunct,
    NormalReferential, NumericAdjunct, ParsingAdjunct, RegisterAdjunct, ShortcutCheckedFormative,
    SuppletiveAdjunct, SuppletiveReferential, UncheckedFormative, Word,
};
use crate::romanize::{flags::FromTokenFlags, stream::ParseError, traits::FromTokens};
use std::str::FromStr;

macro_rules! from_str_impl {
    ($($ty:ty,)+) => {
        $(impl FromStr for $ty {
            type Err = ParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::parse_str(s, FromTokenFlags::NONE)
            }
        })+
    };
}

from_str_impl!(
    Word,
    CheckedFormative,
    ShortcutCheckedFormative,
    UncheckedFormative,
    NormalReferential,
    SuppletiveReferential,
    GeneralReferential,
    AffixualAdjunct,
    ModularAdjunct,
    MCSAdjunct,
    ParsingAdjunct,
    RegisterAdjunct,
    SuppletiveAdjunct,
    BiasAdjunct,
    NumericAdjunct,
);
