//! Contains types related to affixual adjuncts.

use crate::{
    affix::RegularAffix,
    category::{AffixualAdjunctMode, AffixualAdjunctScope},
    gloss::{Gloss, GlossFlags, GlossHelpers, GlossStatic},
    romanize::stream::{FromTokenStream, ParseError, TokenStream},
};
use vec1::Vec1;

/// An affixual adjunct.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AffixualAdjunct {
    /// A variant containing a single affix.
    Single {
        /// The affix of this adjunct.
        affix: RegularAffix,

        /// The scope of this adjunct (dominant over slot V, subordinate to slot VII, etc.).
        scope: AffixualAdjunctScope,

        /// The mode of this adjunct (full or concatenated stem only).
        mode: AffixualAdjunctMode,
    },

    /// A variant containing multiple affixes.
    Multiple {
        /// The first affix of this adjunct.
        first_affix: RegularAffix,

        /// The scope of this adjunct (dominant over slot V, subordinate to slot VII, etc.).
        first_scope: AffixualAdjunctScope,

        /// The other affixes of this adjunct.
        other_affixes: Vec1<RegularAffix>,

        /// The scope of this adjunct (dominant over slot V, subordinate to slot VII, etc.).
        other_scope: Option<AffixualAdjunctScope>,

        /// The mode of this adjunct (full or concatenated stem only).
        mode: AffixualAdjunctMode,
    },
}

impl Gloss for AffixualAdjunct {
    fn gloss(&self, flags: GlossFlags) -> String {
        match self {
            AffixualAdjunct::Single { affix, scope, mode } => {
                let mut output = affix.gloss(flags);
                output.add_dashed(scope.gloss_static_non_default(flags));
                output.add_dashed(mode.gloss_static_non_default(flags));
                output
            }

            AffixualAdjunct::Multiple {
                first_affix,
                first_scope,
                other_affixes,
                other_scope,
                mode,
            } => {
                let mut output = first_affix.gloss(flags);
                output.add_dashed(first_scope.gloss_static_non_default(flags));
                for affix in other_affixes {
                    output.add_dashed(&affix.gloss(flags));
                }
                if let Some(other_scope) = other_scope {
                    output.add_dashed(other_scope.gloss_static_non_default(flags));
                }
                output.add_dashed(mode.gloss_static_non_default(flags));
                output
            }
        }
    }
}

impl FromTokenStream for AffixualAdjunct {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        // affixual adjunct: VxCs (Vs)
        // affixual adjunct: (ë) CsVx Cz VxCs... (Vz)
        todo!()
    }
}
