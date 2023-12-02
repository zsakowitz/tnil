//! Contains types related to register adjuncts.

use std::convert::Infallible;

use crate::{
    category::Register,
    gloss::{Gloss, GlossFlags},
    macros::invalid_type_error,
    romanize::{
        parse::{FromTokenStream, Result},
        stream::TokenStream,
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

invalid_type_error!(
    /// a register adjunct
    enum RegisterTokenError {
        ExpectedHh,
        ExpectedVm,
        TooManyTokens,
    }
);

impl FromTokenStream for RegisterAdjunct {
    type TypeErr = RegisterTokenError;
    type ValueErr = Infallible;

    fn from_token_stream(mut stream: TokenStream) -> Result<Self, Self::TypeErr, Self::ValueErr> {
        // Register adjunct: Hh Vm
        stream.next_or_err::<Hh, _, _>(RegisterTokenError::ExpectedHh)?;
        let register = stream.next_or_err(RegisterTokenError::ExpectedVm)?;
        stream.done_or_err(RegisterTokenError::TooManyTokens)?;
        Ok(RegisterAdjunct { register })
    }
}
