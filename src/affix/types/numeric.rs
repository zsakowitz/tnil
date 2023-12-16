use crate::{
    category::{AffixDegree, AffixType},
    gloss::{Gloss, GlossFlags, GlossStatic},
    prelude::token::{NumeralForm, Token, VowelForm},
    romanize::traits::IntoVxCs,
};

/// A numeric adjunct.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NumericAffix {
    /// The integer part of this number.
    pub form: NumeralForm,

    /// The type of this affix.
    pub r#type: AffixType,

    /// The degree of this affix.
    pub degree: AffixDegree,
}

impl Gloss for NumericAffix {
    fn gloss(&self, _flags: GlossFlags) -> String {
        let mut output = "‘".to_owned();
        output += &self.form.integer_part.to_string();
        output += "’/";
        output += self.degree.gloss_static(GlossFlags::NONE);
        output += self.r#type.gloss_static(GlossFlags::NONE);
        output
    }
}

impl IntoVxCs for NumericAffix {
    fn into_vx_cs(&self) -> (VowelForm, Token) {
        (
            VowelForm {
                has_glottal_stop: false,
                sequence: self.r#type.into(),
                degree: self.degree.into(),
            },
            Token::N(self.form),
        )
    }
}
