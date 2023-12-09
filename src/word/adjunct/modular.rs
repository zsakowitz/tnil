//! Provides types for working with modular adjuncts.

use crate::{
    category::{
        ArbitraryMoodOrCaseScope, Aspect, ModularAdjunctMode, ModularAdjunctScope, NonAspectualVn,
        Stress, Valence, Vn,
    },
    gloss::{Gloss, GlossFlags, GlossHelpers},
    prelude::{IntoTokens, TokenList},
    romanize::{
        flags::FromTokenFlags,
        segment::{VnCm, VnCn},
        stream::{ParseError, TokenStream},
        traits::FromTokens,
    },
};

/// A modular adjunct.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ModularAdjunct {
    /// A variant containing a single aspect.
    Aspect {
        /// The mode, i.e. whether this word scopes over the parent or child formative.
        mode: ModularAdjunctMode,

        /// The single aspect represented by this adjunct.
        aspect: Aspect,
    },

    /// A variant containing 2-3 Vn segments and no special scope.
    NonScoped {
        /// The mode, i.e. whether this word scopes over the parent or child formative.
        mode: ModularAdjunctMode,

        /// The first Vn segment marked by this formative.
        vn1: Vn,

        /// The Cn segment marked by this formative.
        cn: ArbitraryMoodOrCaseScope,

        /// The optional second Vn segment marked by this formative.
        vn2: Option<Vn>,

        /// The third Vn segment marked by this formative. This may actually be the second one if
        /// `vn2` if [`None`]. Cannot be an [`Aspect`].
        vn3: NonAspectualVn,
    },

    /// A variant containing 1-2 Vn segments and a specialized scope.
    Scoped {
        /// The mode, i.e. whether this word scopes over the parent or child formative.
        mode: ModularAdjunctMode,

        /// The first Vn segment marked by this formative.
        vn1: Vn,

        /// The Cn segment marked by this formative.
        cn: ArbitraryMoodOrCaseScope,

        /// The optional second Vn segment marked by this formative.
        vn2: Option<Vn>,

        /// The specialized scope of this formative.
        scope: ModularAdjunctScope,
    },
}

impl Default for ModularAdjunct {
    fn default() -> Self {
        Self::Aspect {
            mode: ModularAdjunctMode::Full,
            aspect: Aspect::RTR,
        }
    }
}

impl Gloss for ModularAdjunct {
    fn gloss(&self, flags: GlossFlags) -> String {
        match self {
            Self::Aspect { mode, aspect } => {
                let mut output = mode.gloss_non_default(flags);
                output.add_dashed(&aspect.gloss(flags));
                output
            }

            Self::NonScoped {
                mode,
                vn1,
                cn,
                vn2,
                vn3,
            } => {
                let mut output = mode.gloss_non_default(flags);
                output.add_dashed(&vn1.gloss_non_default(flags));
                output.add_dashed(&cn.gloss_non_default(flags));
                if let Some(vn2) = vn2 {
                    output.add_dashed(&vn2.gloss_non_default(flags));
                }
                output.add_dashed(&vn3.gloss_non_default(flags));
                if output.is_empty() {
                    Valence::MNO.gloss(flags)
                } else {
                    output
                }
            }

            Self::Scoped {
                mode,
                vn1,
                cn,
                vn2,
                scope,
            } => {
                let mut output = mode.gloss_non_default(flags);
                output.add_dashed(&vn1.gloss_non_default(flags));
                output.add_dashed(&cn.gloss_non_default(flags));
                if let Some(vn2) = vn2 {
                    output.add_dashed(&vn2.gloss_non_default(flags));
                }
                output.add_dashed(&scope.gloss_non_default(flags));
                if output.is_empty() {
                    Valence::MNO.gloss(flags)
                } else {
                    output
                }
            }
        }
    }
}

impl FromTokens for ModularAdjunct {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        let mode: ModularAdjunctMode = stream.parse(flags)?;

        let Some(VnCn { vn: vn1, cn }): Option<VnCn> = stream.parse(flags).ok() else {
            return Ok(ModularAdjunct::Aspect {
                mode,
                aspect: stream.parse(flags)?,
            });
        };

        let vn2: Option<VnCm> = stream.parse(flags).ok();

        match stream.stress() {
            Some(Stress::Ultimate) => Ok(ModularAdjunct::Scoped {
                mode,
                vn1,
                cn,
                vn2: vn2.map(|x| x.vn),
                scope: stream.parse(flags)?,
            }),
            Some(Stress::Antepenultimate) => return Err(ParseError::AntepenultimateStress),
            _ => Ok(ModularAdjunct::NonScoped {
                mode,
                vn1,
                cn,
                vn2: vn2.map(|x| x.vn),
                vn3: stream.parse(flags)?,
            }),
        }
    }
}

impl IntoTokens for ModularAdjunct {
    fn append_to(&self, list: &mut TokenList) {
        match *self {
            Self::Aspect { mode, aspect } => {
                list.append(&mode);
                list.push(aspect);
                list.set_stress(Some(Stress::Penultimate));
            }

            Self::NonScoped {
                mode,
                vn1,
                cn,
                vn2,
                vn3,
            } => {
                list.append(&mode);
                list.append(&VnCn { vn: vn1, cn });
                if let Some(vn) = vn2 {
                    list.append(&VnCm { vn });
                }
                list.push(vn3);
                list.set_stress(Some(Stress::Penultimate));
            }

            Self::Scoped {
                mode,
                vn1,
                cn,
                vn2,
                scope,
            } => {
                list.append(&mode);
                list.append(&VnCn { vn: vn1, cn });
                if let Some(vn) = vn2 {
                    list.append(&VnCm { vn });
                }
                list.push(scope);
                list.set_stress(Some(Stress::Ultimate));
            }
        }
    }
}
