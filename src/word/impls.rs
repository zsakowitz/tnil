use super::{
    AffixualAdjunct, BiasAdjunct, CheckedFormative, GeneralReferential, MCSAdjunct, ModularAdjunct,
    NormalReferential, NumericAdjunct, ParsingAdjunct, RegisterAdjunct, ShortcutCheckedFormative,
    SuppletiveAdjunct, SuppletiveReferential,
};
use crate::romanize::{
    flags::FromTokenFlags,
    stream::{FromTokenStream, ParseError},
};
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
    CheckedFormative,
    ShortcutCheckedFormative,
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