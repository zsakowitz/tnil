//! Defines formative additions.

use super::relation::{NonDefaultRelation, NormalRelation};
use crate::{
    affix::AffixList,
    category::{AffixShortcut, Ca, CaShortcut, Context, Function, Specification},
    AsGeneral, TryAsSpecific,
};
use paste::paste;

/// Additions to a non-shortcut formative.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonShortcutAdditions<AffixShortcutType, SpecificationType> {
    /// The relation of this formative.
    pub relation: NormalRelation,

    /// The affix shortcut of this formative.
    pub affix_shortcut: AffixShortcutType,

    /// The function of this formative.
    pub function: Function,

    /// The specification of this formative.
    pub specification: SpecificationType,

    /// The context of this formative.
    pub context: Context,

    /// The slot V affixes of this formative.
    pub slot_v_affixes: AffixList,

    /// The Ca of this formative.
    pub ca: Ca,
}

/// Additions for normal and numeric non-shortcut formatives.
pub type NormalNonShortcutAdditions = NonShortcutAdditions<AffixShortcut, Specification>;

/// Additions for referential non-shortcut formatives.
pub type ReferentialNonShortcutAdditions = NonShortcutAdditions<(), Specification>;

/// Additions for affixual non-shortcut formatives.
pub type AffixualNonShortcutAdditions = NonShortcutAdditions<(), ()>;

/// Additions for general non-shortcut formatives.
pub type GeneralNonShortcutAdditions =
    NonShortcutAdditions<Option<AffixShortcut>, Option<Specification>>;

/// Additions to a Cn-shortcut formative.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CnShortcutAdditions<AffixShortcutType, SpecificationType> {
    /// The relation of this formative.
    pub relation: NonDefaultRelation,

    /// The affix shortcut of this formative.
    pub affix_shortcut: AffixShortcutType,

    /// The function of this formative.
    pub function: Function,

    /// The specification of this formative.
    pub specification: SpecificationType,

    /// The context of this formative.
    pub context: Context,
}

/// Additions for normal and numeric Cn-shortcut formatives.
pub type NormalCnShortcutAdditions = CnShortcutAdditions<AffixShortcut, Specification>;

/// Additions for referential non-shortcut formatives.
pub type ReferentialCnShortcutAdditions = CnShortcutAdditions<(), Specification>;

/// Additions for affixual Cn-shortcut formatives.
pub type AffixualCnShortcutAdditions = CnShortcutAdditions<(), ()>;

/// Additions for general Cn-shortcut formatives.
pub type GeneralCnShortcutAdditions =
    CnShortcutAdditions<Option<AffixShortcut>, Option<Specification>>;

/// Additions to a Ca-shortcut formative.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CaShortcutAdditions {
    /// The relation of this formative.
    pub relation: NormalRelation,

    /// The slot V affixes of this formative.
    pub slot_v_affixes: AffixList,

    /// The Ca of this formative.
    pub ca: CaShortcut,
}

/// Additions to a normal or numeric formative.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum NormalFormativeAdditions {
    /// Additions to a non-shortcut formatives.
    Normal(NormalNonShortcutAdditions),

    /// Additions to a Cn-shortcut formative.
    CnShortcut(NormalCnShortcutAdditions),

    /// Additions to a Ca-shortcut formative.
    CaShortcut(CaShortcutAdditions),
}

impl Default for NormalFormativeAdditions {
    fn default() -> Self {
        NormalFormativeAdditions::Normal(Default::default())
    }
}

/// Additions to a referential formative.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReferentialFormativeAdditions {
    /// Additions to a non-shortcut formatives.
    Normal(ReferentialNonShortcutAdditions),

    /// Additions to a Cn-shortcut formative.
    CnShortcut(ReferentialCnShortcutAdditions),

    /// Additions to a Ca-shortcut formative.
    CaShortcut(CaShortcutAdditions),
}

impl Default for ReferentialFormativeAdditions {
    fn default() -> Self {
        ReferentialFormativeAdditions::Normal(Default::default())
    }
}

/// Additions to an affixual formative.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AffixualFormativeAdditions {
    /// Additions to a non-shortcut formatives.
    Normal(AffixualNonShortcutAdditions),

    /// Additions to a Cn-shortcut formative.
    CnShortcut(AffixualCnShortcutAdditions),
}

impl Default for AffixualFormativeAdditions {
    fn default() -> Self {
        AffixualFormativeAdditions::Normal(Default::default())
    }
}

/// Additions to a general formative.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum GeneralFormativeAdditions {
    /// Additions to a non-shortcut formatives.
    Normal(GeneralNonShortcutAdditions),

    /// Additions to a Cn-shortcut formative.
    CnShortcut(GeneralCnShortcutAdditions),

    /// Additions to a Ca-shortcut formative.
    CaShortcut(CaShortcutAdditions),
}

/// See the examples below for "documentation" on how this works.
macro_rules! as_general_impl {
    (
        $specific:ident,
        $general:ident,
        $($ca_name:ident)?,
        $($slot_v_affixes_name:ident)?,
        $affix_shortcut_name:ident,
        $specification_name:ident,
        $affix_shortcut:tt,
        $affix_shortcut_value:expr,
        $specification:tt,
        $specification_value:expr,
    ) => {
        paste! {
            #[allow(unused_parens)]
            impl AsGeneral<$general> for $specific {
                fn as_general(self) -> $general {
                    #[allow(unused_variables)]
                    let Self {
                        $affix_shortcut_name,
                        $($ca_name,)?
                        context,
                        function,
                        relation,
                        $($slot_v_affixes_name,)?
                        $specification_name,
                    } = self;

                    $general {
                        affix_shortcut: $affix_shortcut,
                        $($ca_name,)?
                        context,
                        function,
                        relation,
                        $($slot_v_affixes_name,)?
                        specification: $specification,
                    }
                }
            }

            #[allow(unused_parens)]
            impl From<$specific> for $general {
                fn from(value: $specific) -> Self {
                    value.as_general()
                }
            }

            #[allow(unused_parens)]
            impl TryAsSpecific<$specific> for $general {
                fn try_as_specific(self) -> Option<$specific> {
                    #[allow(unused_variables)]
                    if let $general {
                        $affix_shortcut_name: $affix_shortcut,
                        $($ca_name,)?
                        context,
                        function,
                        relation,
                        $($slot_v_affixes_name,)?
                        $specification_name: $specification,
                    } = self {
                        Some($specific {
                            $affix_shortcut_name: $affix_shortcut_value,
                            $($ca_name,)?
                            context,
                            function,
                            relation,
                            $($slot_v_affixes_name,)?
                            $specification_name: $specification_value,
                        })
                    } else {
                        None
                    }
                }
            }
        }
    };
}

as_general_impl!(
    NormalNonShortcutAdditions,
    GeneralNonShortcutAdditions,
    ca,
    slot_v_affixes,
    affix_shortcut,
    specification,
    (Some(affix_shortcut)),
    affix_shortcut,
    (Some(specification)),
    specification,
);

as_general_impl!(
    ReferentialNonShortcutAdditions, // the specific type
    GeneralNonShortcutAdditions,     // the general type
    ca,                              // the Ca field, or empty if the type has no Ca field
    slot_v_affixes,                  // the slot V affixes field, or empty if the type has none
    affix_shortcut,                  // the affix shortcut field (because macro hygiene)
    specification,                   // the specification field (because macro hygiene)
    None,                            // the general value to put in `affix_shortcut`
    (),                              // the specific value to put in `affix_shortcut`
    (Some(specification)),           // the general value to put in `specification`
    specification,                   // the specific value to put in `specification`
);

// Note that parentheses are required around the `Some(...)` patterns because we match them as token
// trees so we can reuse them as expressions and patterns.

as_general_impl!(
    AffixualNonShortcutAdditions,
    GeneralNonShortcutAdditions,
    ca,
    slot_v_affixes,
    affix_shortcut,
    specification,
    None,
    (),
    None,
    (),
);

as_general_impl!(
    NormalCnShortcutAdditions,
    GeneralCnShortcutAdditions,
    , // Cn shortcut additions don't have Ca slots, so we pass nothing here
    , // Cn shortcut additions also don't have slot V affixes
    affix_shortcut,
    specification,
    (Some(affix_shortcut)),
    affix_shortcut,
    (Some(specification)),
    specification,
);

as_general_impl!(
    ReferentialCnShortcutAdditions,
    GeneralCnShortcutAdditions,
    ,
    ,
    affix_shortcut,
    specification,
    None,
    (),
    (Some(specification)),
    specification,
);

as_general_impl!(
    AffixualCnShortcutAdditions,
    GeneralCnShortcutAdditions,
    ,
    ,
    affix_shortcut,
    specification,
    None,
    (),
    None,
    (),
);

impl AsGeneral<GeneralFormativeAdditions> for NormalFormativeAdditions {
    fn as_general(self) -> GeneralFormativeAdditions {
        match self {
            Self::Normal(value) => GeneralFormativeAdditions::Normal(value.as_general()),
            Self::CnShortcut(value) => GeneralFormativeAdditions::CnShortcut(value.as_general()),
            Self::CaShortcut(value) => GeneralFormativeAdditions::CaShortcut(value),
        }
    }
}

impl From<NormalFormativeAdditions> for GeneralFormativeAdditions {
    fn from(value: NormalFormativeAdditions) -> Self {
        value.as_general()
    }
}

impl TryAsSpecific<NormalFormativeAdditions> for GeneralFormativeAdditions {
    fn try_as_specific(self) -> Option<NormalFormativeAdditions> {
        Some(match self {
            Self::Normal(value) => NormalFormativeAdditions::Normal(value.try_as_specific()?),
            Self::CnShortcut(value) => {
                NormalFormativeAdditions::CnShortcut(value.try_as_specific()?)
            }
            Self::CaShortcut(value) => NormalFormativeAdditions::CaShortcut(value),
        })
    }
}

impl AsGeneral<GeneralFormativeAdditions> for ReferentialFormativeAdditions {
    fn as_general(self) -> GeneralFormativeAdditions {
        match self {
            Self::Normal(value) => GeneralFormativeAdditions::Normal(value.as_general()),
            Self::CnShortcut(value) => GeneralFormativeAdditions::CnShortcut(value.as_general()),
            Self::CaShortcut(value) => GeneralFormativeAdditions::CaShortcut(value),
        }
    }
}

impl From<ReferentialFormativeAdditions> for GeneralFormativeAdditions {
    fn from(value: ReferentialFormativeAdditions) -> Self {
        value.as_general()
    }
}

impl TryAsSpecific<ReferentialFormativeAdditions> for GeneralFormativeAdditions {
    fn try_as_specific(self) -> Option<ReferentialFormativeAdditions> {
        Some(match self {
            Self::Normal(value) => ReferentialFormativeAdditions::Normal(value.try_as_specific()?),
            Self::CnShortcut(value) => {
                ReferentialFormativeAdditions::CnShortcut(value.try_as_specific()?)
            }
            Self::CaShortcut(value) => ReferentialFormativeAdditions::CaShortcut(value),
        })
    }
}

impl AsGeneral<GeneralFormativeAdditions> for AffixualFormativeAdditions {
    fn as_general(self) -> GeneralFormativeAdditions {
        match self {
            Self::Normal(value) => GeneralFormativeAdditions::Normal(value.as_general()),
            Self::CnShortcut(value) => GeneralFormativeAdditions::CnShortcut(value.as_general()),
        }
    }
}

impl From<AffixualFormativeAdditions> for GeneralFormativeAdditions {
    fn from(value: AffixualFormativeAdditions) -> Self {
        value.as_general()
    }
}

impl TryAsSpecific<AffixualFormativeAdditions> for GeneralFormativeAdditions {
    fn try_as_specific(self) -> Option<AffixualFormativeAdditions> {
        match self {
            Self::Normal(value) => {
                Some(AffixualFormativeAdditions::Normal(value.try_as_specific()?))
            }
            Self::CnShortcut(value) => Some(AffixualFormativeAdditions::CnShortcut(
                value.try_as_specific()?,
            )),
            Self::CaShortcut(_) => None,
        }
    }
}
