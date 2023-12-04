//! Defines formative core types.

use super::root::{
    AffixualFormativeRoot, GeneralFormativeRoot, NormalFormativeRoot, NumericFormativeRoot,
    ReferentialFormativeRoot,
};
use crate::{
    affix::AffixList,
    category::{Stem, Version},
    specificity::{AsGeneral, TryAsSpecific},
};

/// The core of a formative.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FormativeCore<RootType, StemType> {
    /// The root of this formative.
    pub root: RootType,

    /// The stem of this formative.
    pub stem: StemType,

    /// The version of this formative.
    pub version: Version,

    /// The slot VII affixes of this formative.
    pub slot_vii_affixes: AffixList,
}

/// The core of a normal formative.
pub type NormalFormativeCore = FormativeCore<NormalFormativeRoot, Stem>;

/// The core of a numeric formative.
pub type NumericFormativeCore = FormativeCore<NumericFormativeRoot, Stem>;

/// The core of a referential formative.
pub type ReferentialFormativeCore = FormativeCore<ReferentialFormativeRoot, ()>;

/// The core of an affixual formative.
pub type AffixualFormativeCore = FormativeCore<AffixualFormativeRoot, ()>;

/// The core of a general formative.
pub type GeneralFormativeCore = FormativeCore<GeneralFormativeRoot, Option<Stem>>;

macro_rules! as_general_impl {
    ($specific:ident, $variant:ident, $stem:ident, $stem_pat:pat, $stem_expr:expr, $stem_value:expr) => {
        impl AsGeneral<GeneralFormativeCore> for $specific {
            fn as_general(self) -> GeneralFormativeCore {
                #[allow(unused_variables)]
                let Self {
                    root,
                    $stem,
                    version,
                    slot_vii_affixes,
                } = self;

                GeneralFormativeCore {
                    root: root.as_general(),
                    $stem: $stem_expr,
                    version,
                    slot_vii_affixes,
                }
            }
        }

        impl From<$specific> for GeneralFormativeCore {
            fn from(value: $specific) -> Self {
                value.as_general()
            }
        }

        impl TryAsSpecific<$specific> for GeneralFormativeCore {
            fn try_as_specific(self) -> Option<$specific> {
                match self {
                    Self {
                        root: GeneralFormativeRoot::$variant(root),
                        $stem: $stem_pat,
                        version,
                        slot_vii_affixes,
                    } => Some($specific {
                        root,
                        $stem: $stem_value,
                        version,
                        slot_vii_affixes,
                    }),
                    _ => None,
                }
            }
        }
    };
}

as_general_impl!(
    NormalFormativeCore,
    Normal,
    stem,
    Some(stem),
    Some(stem),
    stem
);

as_general_impl!(
    NumericFormativeCore,
    Numeric,
    stem,
    Some(stem),
    Some(stem),
    stem
);

as_general_impl!(ReferentialFormativeCore, Referential, stem, None, None, ());

as_general_impl!(AffixualFormativeCore, Affixual, stem, None, None, ());
