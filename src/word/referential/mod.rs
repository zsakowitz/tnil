//! Contains types related to referentials.

pub(super) mod aliases;

use crate::{
    affix::RegularAffix,
    category::{Case, Essence, NormalReferentList, Specification, SuppletiveAdjunctMode},
    gloss::{Gloss, GlossFlags, GlossHelpers, GlossStatic},
    specificity::{AsGeneral, TryAsSpecific},
};
use aliases::*;

/// A referential. The type parameter indicates which kind of referents are received.
///
/// Typically used either as [`NormalReferential`], one where the initial referent is an actual
/// referent, or as [`SuppletiveReferential`], one where the initial referent is a suppletive
/// adjunct mode.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Referential<T> {
    /// A single referential.
    Single {
        /// The referent of this referential.
        referent: T,

        /// The first case of this referential.
        first_case: Case,

        /// The second case of this referential.
        second_case: Option<Case>,

        /// The essence of this referential.
        essence: Essence,
    },

    /// A dual referential.
    Dual {
        /// The first referent of this referential.
        first_referent: T,

        /// The first case of this referential.
        first_case: Case,

        /// The second case of this referential.
        second_case: Case,

        /// The second referent of this referential.
        second_referent: NormalReferentList,

        /// The essence of this referential.
        essence: Essence,
    },

    /// A combination referential.
    Combination {
        /// The first referent of this referential.
        referent: T,

        /// The first case of this referential.
        first_case: Case,

        /// The specification of this referential.
        specification: Specification,

        /// The affixes in this referential.
        affixes: Vec<RegularAffix>,

        /// The second case of this referential.
        second_case: Option<Case>,

        /// The essence of this referential.
        essence: Essence,
    },
}

impl<T> Gloss for Referential<T>
where
    T: Gloss,
{
    fn gloss(&self, flags: GlossFlags) -> String {
        match self {
            Self::Single {
                referent,
                first_case,
                second_case,
                essence,
            } => {
                let mut output = referent.gloss(flags);
                if let Some(second_case) = second_case {
                    output.add_dashed(first_case.gloss_static(flags));
                    output.add_dashed(second_case.gloss_static(flags));
                } else {
                    output.add_dashed(first_case.gloss_static_non_default(flags));
                }
                output.add_dashed(essence.gloss_static_non_default(flags));
                output
            }

            Self::Dual {
                first_referent,
                first_case,
                second_case,
                second_referent,
                essence,
            } => {
                let mut output = first_referent.gloss(flags);
                output.add_dashed(first_case.gloss_static(flags));
                output.add_dashed(second_case.gloss_static(flags));
                output.add_dashed(&second_referent.gloss(flags));
                output.add_dashed(essence.gloss_static_non_default(flags));
                output
            }

            Self::Combination {
                referent,
                first_case,
                specification,
                affixes,
                second_case,
                essence,
            } => {
                let mut output = referent.gloss(flags);
                output.add_dashed(first_case.gloss_static_non_default(flags));
                output.add_dashed(specification.gloss_static(flags));
                for affix in affixes {
                    output.add_dashed(&affix.gloss(flags));
                }
                if let Some(second_case) = second_case {
                    output.add_dashed(&second_case.gloss(flags));
                }
                output.add_dashed(essence.gloss_static_non_default(flags));
                output
            }
        }
    }
}

/// A general referential's first referent, which may either be a [`NormalReferentList`] or
/// [`SuppletiveAdjunctMode`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum GeneralReferent {
    /// A normal referent list.
    Normal(NormalReferentList),

    /// A suppletive adjunct mode.
    Suppletive(SuppletiveAdjunctMode),
}

impl Gloss for GeneralReferent {
    fn gloss(&self, flags: GlossFlags) -> String {
        match self {
            Self::Normal(value) => value.gloss(flags),
            Self::Suppletive(value) => value.gloss(flags),
        }
    }
}

impl AsGeneral<GeneralReferent> for NormalReferentList {
    fn as_general(self) -> GeneralReferent {
        GeneralReferent::Normal(self)
    }
}

impl From<NormalReferentList> for GeneralReferent {
    fn from(value: NormalReferentList) -> Self {
        value.as_general()
    }
}

impl TryAsSpecific<NormalReferentList> for GeneralReferent {
    fn try_as_specific(self) -> Option<NormalReferentList> {
        match self {
            Self::Normal(value) => Some(value),
            _ => None,
        }
    }
}

impl AsGeneral<GeneralReferent> for SuppletiveAdjunctMode {
    fn as_general(self) -> GeneralReferent {
        GeneralReferent::Suppletive(self)
    }
}

impl From<SuppletiveAdjunctMode> for GeneralReferent {
    fn from(value: SuppletiveAdjunctMode) -> Self {
        value.as_general()
    }
}

impl TryAsSpecific<SuppletiveAdjunctMode> for GeneralReferent {
    fn try_as_specific(self) -> Option<SuppletiveAdjunctMode> {
        match self {
            Self::Suppletive(value) => Some(value),
            _ => None,
        }
    }
}

impl AsGeneral<GeneralReferential> for NormalReferential {
    fn as_general(self) -> GeneralReferential {
        match self {
            Self::Single {
                referent,
                first_case,
                second_case,
                essence,
            } => GeneralReferential::Single {
                referent: referent.as_general(),
                first_case,
                second_case,
                essence,
            },

            Self::Dual {
                first_referent,
                first_case,
                second_case,
                second_referent,
                essence,
            } => GeneralReferential::Dual {
                first_referent: first_referent.as_general(),
                first_case,
                second_case,
                second_referent,
                essence,
            },

            Self::Combination {
                referent,
                first_case,
                specification,
                affixes,
                second_case,
                essence,
            } => GeneralReferential::Combination {
                referent: referent.as_general(),
                first_case,
                specification,
                affixes,
                second_case,
                essence,
            },
        }
    }
}

impl From<NormalReferential> for GeneralReferential {
    fn from(value: NormalReferential) -> Self {
        value.as_general()
    }
}

impl TryAsSpecific<NormalReferential> for GeneralReferential {
    fn try_as_specific(self) -> Option<NormalReferential> {
        Some(match self {
            Self::Single {
                referent,
                first_case,
                second_case,
                essence,
            } => NormalReferential::Single {
                referent: referent.try_as_specific()?,
                first_case,
                second_case,
                essence,
            },

            Self::Dual {
                first_referent,
                first_case,
                second_case,
                second_referent,
                essence,
            } => NormalReferential::Dual {
                first_referent: first_referent.try_as_specific()?,
                first_case,
                second_case,
                second_referent,
                essence,
            },

            Self::Combination {
                referent,
                first_case,
                specification,
                affixes,
                second_case,
                essence,
            } => NormalReferential::Combination {
                referent: referent.try_as_specific()?,
                first_case,
                specification,
                affixes,
                second_case,
                essence,
            },
        })
    }
}

impl AsGeneral<GeneralReferential> for SuppletiveReferential {
    fn as_general(self) -> GeneralReferential {
        match self {
            Self::Single {
                referent,
                first_case,
                second_case,
                essence,
            } => GeneralReferential::Single {
                referent: referent.as_general(),
                first_case,
                second_case,
                essence,
            },

            Self::Dual {
                first_referent,
                first_case,
                second_case,
                second_referent,
                essence,
            } => GeneralReferential::Dual {
                first_referent: first_referent.as_general(),
                first_case,
                second_case,
                second_referent,
                essence,
            },

            Self::Combination {
                referent,
                first_case,
                specification,
                affixes,
                second_case,
                essence,
            } => GeneralReferential::Combination {
                referent: referent.as_general(),
                first_case,
                specification,
                affixes,
                second_case,
                essence,
            },
        }
    }
}

impl From<SuppletiveReferential> for GeneralReferential {
    fn from(value: SuppletiveReferential) -> Self {
        value.as_general()
    }
}

impl TryAsSpecific<SuppletiveReferential> for GeneralReferential {
    fn try_as_specific(self) -> Option<SuppletiveReferential> {
        Some(match self {
            Self::Single {
                referent,
                first_case,
                second_case,
                essence,
            } => SuppletiveReferential::Single {
                referent: referent.try_as_specific()?,
                first_case,
                second_case,
                essence,
            },

            Self::Dual {
                first_referent,
                first_case,
                second_case,
                second_referent,
                essence,
            } => SuppletiveReferential::Dual {
                first_referent: first_referent.try_as_specific()?,
                first_case,
                second_case,
                second_referent,
                essence,
            },

            Self::Combination {
                referent,
                first_case,
                specification,
                affixes,
                second_case,
                essence,
            } => SuppletiveReferential::Combination {
                referent: referent.try_as_specific()?,
                first_case,
                specification,
                affixes,
                second_case,
                essence,
            },
        })
    }
}
