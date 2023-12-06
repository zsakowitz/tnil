use crate::{
    gloss::{Gloss, GlossFlags},
    romanize::{
        flags::FromTokenFlags,
        stream::{ParseError, TokenStream},
        token::NumeralForm,
        traits::FromTokens,
    },
};

/// A numeric adjunct.
#[non_exhaustive] // TODO: Remove this once we deal with decimals.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumericAdjunct {
    /// The integer part of this adjunct.
    pub integer_part: u64,
}

impl Gloss for NumericAdjunct {
    fn gloss(&self, _flags: GlossFlags) -> String {
        let mut output = "‘".to_owned();
        output += &self.integer_part.to_string();
        output += "’";
        output
    }
}

impl FromTokens for NumericAdjunct {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        let nn: NumeralForm = stream.parse(flags)?;

        Ok(NumericAdjunct {
            integer_part: nn.integer_part,
        })
    }
}
