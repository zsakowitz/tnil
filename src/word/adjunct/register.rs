//! Contains types related to register adjuncts.

use crate::{
    category::Register,
    gloss::{Gloss, GlossFlags},
    romanize::{
        flags::FromTokenFlags,
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
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        stream.parse::<Hh>(flags)?;

        Ok(RegisterAdjunct {
            register: stream.parse(flags)?,
        })
    }
}
