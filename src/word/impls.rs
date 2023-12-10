use crate::prelude::{word::*, *};
use std::str::FromStr;

macro_rules! impls {
    ($($ty:ty,)+) => {
        $(impl FromStr for $ty {
            type Err = ParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::parse_str(s, FromTokenFlags::NONE)
            }
        }

        impl ToString for $ty {
            fn to_string(&self) -> String {
                self.to_string_with(IntoTokensFlags::NONE)
            }
        })+
    };
}

impls!(
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
