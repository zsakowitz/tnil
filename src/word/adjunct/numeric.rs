use crate::{
    gloss::{Gloss, GlossFlags},
    romanize::{
        stream::{FromTokenStream, ParseError, TokenStream},
        token::NumeralForm,
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

impl FromTokenStream for NumericAdjunct {
    fn parse_volatile(stream: &mut TokenStream) -> Result<Self, ParseError> {
        let nn: NumeralForm = stream.parse()?;

        Ok(NumericAdjunct {
            integer_part: nn.integer_part,
        })
    }
}
