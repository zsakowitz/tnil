//! Contains types related to register adjuncts.

use crate::{
    category::Register,
    gloss::{Gloss, GlossFlags},
    romanize::{
        stream::{FromTokenStream, ParseError, TokenStream},
        token::Hh,
    },
};

/// A register adjunct.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RegisterAdjunct {
    /// The register of this adjunct.
    pub register: Register,
}

impl Gloss for RegisterAdjunct {
    fn gloss(&self, flags: GlossFlags) -> String {
        self.register.gloss(flags)
    }
}

impl FromTokenStream for RegisterAdjunct {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        stream.parse::<Hh>()?;

        Ok(RegisterAdjunct {
            register: stream.parse()?,
        })
    }
}
