//! Contains types related to bias adjuncts.

use crate::{
    category::Bias,
    gloss::{GlossFlags, GlossStatic},
};

/// A bias adjunct.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BiasAdjunct {
    /// The bias of this adjunct.
    pub bias: Bias,
}

impl GlossStatic for BiasAdjunct {
    fn gloss_static(&self, flags: GlossFlags) -> &'static str {
        self.bias.gloss_static(flags)
    }
}
