use crate::gloss::{Gloss, GlossFlags};

/// A numeric adjunct.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub struct NumericAffix {
    /// The integer part of this number.
    pub integer_part: u64,
}

impl NumericAffix {
    /// Constructs a new [`NumericAffix`].
    pub const fn new(integer_part: u64) -> Self {
        Self { integer_part }
    }
}

impl Gloss for NumericAffix {
    fn gloss(&self, _flags: GlossFlags) -> String {
        self.integer_part.to_string()
    }
}
