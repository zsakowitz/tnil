use crate::{
    affix::RegularAffix,
    prelude::{AsGeneral, TryAsSpecific},
    word::{CheckedFormative, ShortcutCheckedFormative, UncheckedFormative},
};

/// A formative with slot XI affixes that scope over the whole formative.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Extended<T> {
    /// The base of this formative.
    pub base: T,

    /// The slot XI affixes of this formative.
    pub slot_xi_affixes: Vec<RegularAffix>,
}

macro_rules! as_general_impl {
    ($general:ident, $specific:ident) => {
        impl AsGeneral<Extended<$general>> for Extended<$specific> {
            fn as_general(self) -> Extended<$general> {
                Extended {
                    base: self.base.as_general(),
                    slot_xi_affixes: self.slot_xi_affixes,
                }
            }
        }

        impl From<Extended<$specific>> for Extended<$general> {
            fn from(value: Extended<$specific>) -> Self {
                value.as_general()
            }
        }

        impl TryAsSpecific<Extended<$specific>> for Extended<$general> {
            fn try_as_specific(self) -> Option<Extended<$specific>> {
                Some(Extended {
                    base: self.base.try_as_specific()?,
                    slot_xi_affixes: self.slot_xi_affixes,
                })
            }
        }
    };
}

as_general_impl!(ShortcutCheckedFormative, CheckedFormative);
as_general_impl!(UncheckedFormative, CheckedFormative);
as_general_impl!(UncheckedFormative, ShortcutCheckedFormative);

macro_rules! as_extended_impl {
    ($ty:ident) => {
        impl AsGeneral<Extended<$ty>> for $ty {
            fn as_general(self) -> Extended<$ty> {
                Extended {
                    base: self,
                    slot_xi_affixes: Vec::new(),
                }
            }
        }

        impl From<$ty> for Extended<$ty> {
            fn from(value: $ty) -> Self {
                value.as_general()
            }
        }

        impl TryAsSpecific<$ty> for Extended<$ty> {
            fn try_as_specific(self) -> Option<$ty> {
                self.slot_xi_affixes.is_empty().then(|| self.base)
            }
        }
    };
}

as_extended_impl!(CheckedFormative);
as_extended_impl!(ShortcutCheckedFormative);
as_extended_impl!(UncheckedFormative);

macro_rules! as_extended_general_impl {
    ($general:ident, $specific:ident) => {
        impl AsGeneral<Extended<$general>> for $specific {
            fn as_general(self) -> Extended<$general> {
                Extended {
                    base: self.as_general(),
                    slot_xi_affixes: Vec::new(),
                }
            }
        }

        impl From<$specific> for Extended<$general> {
            fn from(value: $specific) -> Self {
                value.as_general()
            }
        }

        impl TryAsSpecific<$specific> for Extended<$general> {
            fn try_as_specific(self) -> Option<$specific> {
                self.slot_xi_affixes
                    .is_empty()
                    .then(|| self.base.try_as_specific())
                    .flatten()
            }
        }
    };
}

as_extended_general_impl!(ShortcutCheckedFormative, CheckedFormative);
as_extended_general_impl!(UncheckedFormative, CheckedFormative);
as_extended_general_impl!(UncheckedFormative, ShortcutCheckedFormative);
