//! Contains types related to affixual adjuncts.

mod multiple;
mod single;

pub use multiple::*;
pub use single::*;

use self::{multiple::MultipleAffixAdjunct, single::SingleAffixAdjunct};
use crate::{
    gloss::{Gloss, GlossFlags},
    romanize::stream::{FromTokenStream, ParseError, TokenStream},
};

/// An affixual adjunct.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AffixualAdjunct {
    /// A variant containing a single affix.
    Single(SingleAffixAdjunct),

    /// A variant containing multiple affixes.
    Multiple(MultipleAffixAdjunct),
}

impl Gloss for AffixualAdjunct {
    fn gloss(&self, flags: GlossFlags) -> String {
        match self {
            AffixualAdjunct::Single(value) => value.gloss(flags),
            AffixualAdjunct::Multiple(value) => value.gloss(flags),
        }
    }
}

impl FromTokenStream for AffixualAdjunct {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        match stream.peek() {
            _ => todo!(),
        }

        // affixual adjunct: VxCs (Vs)
        // affixual adjunct: (ë) CsVx Cz VxCs... (Vz)
    }
}
