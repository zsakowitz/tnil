use crate::{
    category::Ca,
    gloss::{Gloss, GlossFlags},
};

/// A case-stacking affix.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CaStackingAffix {
    /// The Ca of this affix.
    pub ca: Ca,
}

impl CaStackingAffix {
    /// Constructs a new [`CaStackingAffix`].
    pub fn new(ca: Ca) -> Self {
        Self { ca }
    }
}

impl Gloss for CaStackingAffix {
    fn gloss(&self, flags: GlossFlags) -> String {
        let mut output = "(".to_owned();
        output += &self.ca.gloss(flags);
        output += ")";
        output
    }
}
