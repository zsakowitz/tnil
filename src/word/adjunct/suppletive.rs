//! Contains types related to suppletive adjuncts.

use std::convert::Infallible;

use crate::{
    category::{Case, SuppletiveAdjunctMode},
    gloss::{Gloss, GlossFlags, GlossHelpers, GlossStatic},
    macros::invalid_type_error,
    romanize::{
        parse::{FromTokenStream, Result},
        stream::TokenStream,
    },
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

invalid_type_error!(
    /// a suppletive adjunct
    enum SuppletiveTokenError {
        ExpectedCp,
        ExpectedVc,
        TooManyTokens,
    }
);

impl FromTokenStream for SuppletiveAdjunct {
    type TypeErr = SuppletiveTokenError;
    type ValueErr = Infallible;

    fn from_token_stream(mut stream: TokenStream) -> Result<Self, Self::TypeErr, Self::ValueErr> {
        // Suppletive adjunct: Cp Vc
        let mode = stream.next_or_err(SuppletiveTokenError::ExpectedCp)?;
        let case = stream.next_or_err(SuppletiveTokenError::ExpectedVc)?;
        stream.done_or_err(SuppletiveTokenError::TooManyTokens)?;
        Ok(SuppletiveAdjunct { mode, case })
    }
}
