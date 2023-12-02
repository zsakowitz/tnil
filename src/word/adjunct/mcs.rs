//! Defines types related to mood/case-scope adjuncts.

use std::convert::Infallible;

use crate::{
    category::MoodOrCaseScope,
    gloss::{GlossFlags, GlossStatic},
    macros::invalid_type_error,
    romanize::{
        parse::{FromTokenStream, Result},
        stream::TokenStream,
        token::Hr,
    },
};

/// A mood/case-scope adjunct.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MCSAdjunct {
    /// The mood or case-scope represented by this adjunct.
    pub mcs: MoodOrCaseScope,
}

impl GlossStatic for MCSAdjunct {
    fn gloss_static(&self, flags: GlossFlags) -> &'static str {
        self.mcs.gloss_static(flags)
    }
}

invalid_type_error!(
    /// a mood/case-scope adjunct
    enum MCSTokenError {
        ExpectedHr,
        ExpectedCy,
        TooManyTokens,
    }
);

impl FromTokenStream for MCSAdjunct {
    type TypeErr = MCSTokenError;
    type ValueErr = Infallible;

    fn from_token_stream(mut stream: TokenStream) -> Result<Self, Self::TypeErr, Self::ValueErr> {
        // MCS adjunct: Hr Cy
        stream.next_or_err::<Hr, _, _>(MCSTokenError::ExpectedHr)?;
        let mcs = stream.next_or_err(MCSTokenError::ExpectedCy)?;
        stream.done_or_err(MCSTokenError::TooManyTokens)?;
        Ok(MCSAdjunct { mcs })
    }
}
