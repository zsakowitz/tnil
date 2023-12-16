use crate::{
    gloss::{Gloss, GlossFlags},
    prelude::{token::Token, IntoToken},
    romanize::{
        flags::FromTokenFlags,
        stream::{ParseError, TokenStream},
        token::NumeralForm,
        traits::FromTokens,
    },
};

/// A numeric adjunct.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NumericAdjunct {
    /// The numeral form of this adjunct.
    pub form: NumeralForm,
}

impl Gloss for NumericAdjunct {
    fn gloss(&self, _flags: GlossFlags) -> String {
        let mut output = "‘".to_owned();
        output += &self.form.integer_part.to_string();
        output += "’";
        output
    }
}

impl FromTokens for NumericAdjunct {
    fn parse_volatile(
        stream: &mut TokenStream,
        _flags: FromTokenFlags,
    ) -> Result<Self, ParseError> {
        Ok(NumericAdjunct {
            form: stream.next().ok_or(ParseError::ExpectedNn)?,
        })
    }
}

impl IntoToken for NumericAdjunct {
    fn into_token(self) -> Token {
        Token::N(self.form)
    }
}
