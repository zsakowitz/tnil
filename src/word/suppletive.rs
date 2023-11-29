//! Contains types related to suppletive adjuncts.

use crate::{
    category::{Case, SuppletiveAdjunctMode},
    gloss::{Gloss, GlossFlags, GlossHelpers, GlossStatic},
};

/// A suppletive adjunct.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SuppletiveAdjunct {
    /// The mode of this adjunct (CAR, QUO, NAM, or PHR).
    pub mode: SuppletiveAdjunctMode,

    /// The case of this adjunct.
    pub case: Case,
}

impl Gloss for SuppletiveAdjunct {
    fn gloss(&self, flags: GlossFlags) -> String {
        let mut output = self.mode.gloss(flags);
        output.add_dashed(self.case.gloss_static_non_default(flags));
        output
    }
}
