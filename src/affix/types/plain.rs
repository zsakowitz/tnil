use crate::{
    category::{AffixDegree, AffixType},
    prelude::{
        token::{OwnedConsonantForm, Token, VowelForm},
        Gloss, GlossFlags, GlossStatic,
    },
    romanize::traits::IntoVxCs,
};

/// A plain affix.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlainAffix {
    /// The Cs form of this affix.
    pub cs: String,

    /// The type of this affix.
    pub r#type: AffixType,

    /// The degree of this affix.
    pub degree: AffixDegree,
}

impl PlainAffix {
    /// Creates a new [`PlainAffix`] instance from a Cs form, affix degree, and affix type.
    pub fn new(cs: impl Into<String>, r#type: AffixType, degree: AffixDegree) -> Self {
        Self {
            cs: cs.into(),
            degree,
            r#type,
        }
    }
}

impl Gloss for PlainAffix {
    // TODO: Use affix data
    fn gloss(&self, flags: GlossFlags) -> String {
        if flags.matches(GlossFlags::FORMAT_MARKDOWN) {
            let mut output = "**".to_owned();
            output += &self.cs; // We assume the Cs form doesn't need escaping.
            output += "**/";
            output += self.degree.gloss_static(GlossFlags::NONE);
            output += self.r#type.gloss_static(GlossFlags::NONE);
            output
        } else {
            let mut output = self.cs.to_owned();
            output += "/";
            output += self.degree.gloss_static(GlossFlags::NONE);
            output += self.r#type.gloss_static(GlossFlags::NONE);
            return output;
        }
    }
}

impl IntoVxCs for PlainAffix {
    fn into_vx_cs(&self) -> (VowelForm, Token) {
        (
            VowelForm {
                has_glottal_stop: false,
                sequence: self.r#type.into(),
                degree: self.degree.into(),
            },
            Token::C(OwnedConsonantForm(self.cs.clone())),
        )
    }
}
