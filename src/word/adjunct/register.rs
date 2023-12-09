//! Contains types related to register adjuncts.

use crate::{
    category::Register,
    gloss::{Gloss, GlossFlags},
    prelude::{IntoTokens, IntoTokensFlags, TokenList},
    romanize::{
        flags::FromTokenFlags,
        stream::{ParseError, TokenStream},
        token::Hh,
        traits::FromTokens,
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

impl FromTokens for RegisterAdjunct {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        stream.parse::<Hh>(flags)?;

        Ok(RegisterAdjunct {
            register: stream.parse(flags)?,
        })
    }
}

impl IntoTokens for RegisterAdjunct {
    fn append_to(&self, list: &mut TokenList, _flags: IntoTokensFlags) {
        list.push(Hh);
        list.push(self.register);
    }
}
