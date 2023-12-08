use crate::prelude::{word::*, *};
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
    SingleAffixAdjunct,
    MultipleAffixAdjunct,
    ModularAdjunct,
    MCSAdjunct,
    ParsingAdjunct,
    RegisterAdjunct,
    SuppletiveAdjunct,
    BiasAdjunct,
    NumericAdjunct,
);
