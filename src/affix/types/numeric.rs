use crate::{
    category::{AffixDegree, AffixType},
    gloss::{Gloss, GlossFlags, GlossStatic},
    prelude::{
        token::{NumeralForm, Token, VowelForm},
        IntoVxCs,
    },
};

/// A numeric adjunct.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive] // TODO: Remove non_exhaustive once we decide how to work with decimals.
pub struct NumericAffix {
    /// The integer part of this number.
    pub integer_part: u64,

    /// The type of this affix.
    pub r#type: AffixType,

    /// The degree of this affix.
    pub degree: AffixDegree,
}

impl NumericAffix {
    /// Creates a new [`NumericAffix`] instance from a Cs form, affix degree, and affix type.
    pub fn new(integer_part: u64, r#type: AffixType, degree: AffixDegree) -> Self {
        Self {
            integer_part,
            degree,
            r#type,
        }
    }
}

impl Gloss for NumericAffix {
    fn gloss(&self, _flags: GlossFlags) -> String {
        let mut output = "‘".to_owned();
        output += &self.integer_part.to_string();
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
            Token::N(NumeralForm {
                integer_part: self.integer_part,
            }),
        )
    }
}
