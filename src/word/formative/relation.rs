//! Defines formative relations.

use crate::{
    category::{
        Case, CaseScope, IllocutionOrValidation, Mood, MoodOrCaseScope, NominalMode,
        NonDefaultCaseScope, NonDefaultMood,
    },
    AsGeneral, TryAsSpecific,
};

/// A formative relation.
///
/// See the type aliases of [`NormalRelation`] and [`NonDefaultRelation`] for common instantiations
/// of the generics to this enum.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Relation<CaseScopeType, MoodType> {
    /// A nominal formative.
    Nominal {
        /// The mode of this formative (plain nominal, framed, or concatenated).
        mode: NominalMode,

        /// The case scope of this formative.
        case_scope: CaseScopeType,

        /// The case of this formative.
        case: Case,
    },

    /// An unframed verbal formative.
    Verbal {
        /// The mood of this formative.
        mood: MoodType,

        /// The illocution or validation of this formative.
        ivl: IllocutionOrValidation,
    },
}

impl<CaseScopeType, MoodType> Default for Relation<CaseScopeType, MoodType>
where
    CaseScopeType: Default,
{
    fn default() -> Self {
        Self::Nominal {
            mode: NominalMode::NOM,
            case_scope: CaseScopeType::default(),
            case: Case::THM,
        }
    }
}

/// A formative relation that can contain a default case-scope or mood.
pub type NormalRelation = Relation<CaseScope, Mood>;

/// A formative relation that cannot contain a default case-scope or mood.
pub type NonDefaultRelation = Relation<NonDefaultCaseScope, NonDefaultMood>;

impl AsGeneral<NormalRelation> for NonDefaultRelation {
    /// Converts `self` into a [`NormalRelation`].
    fn as_general(self) -> NormalRelation {
        match self {
            Self::Nominal {
                mode,
                case_scope,
                case,
            } => NormalRelation::Nominal {
                mode,
                case_scope: case_scope.as_general(),
                case,
            },

            Self::Verbal { mood, ivl } => NormalRelation::Verbal {
                mood: mood.as_general(),
                ivl,
            },
        }
    }
}

impl From<NonDefaultRelation> for NormalRelation {
    fn from(value: NonDefaultRelation) -> Self {
        value.as_general()
    }
}

impl TryAsSpecific<NonDefaultRelation> for NormalRelation {
    fn try_as_specific(self) -> Option<NonDefaultRelation> {
        Some(match self {
            Self::Nominal {
                mode,
                case_scope,
                case,
            } => NonDefaultRelation::Nominal {
                mode,
                case_scope: match case_scope.try_as_specific() {
                    Some(value) => value,
                    None => return None,
                },
                case,
            },

            Self::Verbal { mood, ivl } => NonDefaultRelation::Verbal {
                mood: match mood.try_as_specific() {
                    Some(value) => value,
                    None => return None,
                },
                ivl,
            },
        })
    }
}

impl NormalRelation {
    /// Gets the mood or case-scope of this relation.
    pub fn mood_or_case_scope(&self) -> MoodOrCaseScope {
        match self {
            Self::Nominal { case_scope, .. } => case_scope.as_general(),
            Self::Verbal { mood, .. } => mood.as_general(),
        }
    }
}

impl NonDefaultRelation {
    /// Gets the mood or case-scope of this relation.
    pub fn mood_or_case_scope(&self) -> MoodOrCaseScope {
        self.as_general().mood_or_case_scope()
    }
}
