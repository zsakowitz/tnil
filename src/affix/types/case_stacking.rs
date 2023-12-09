use crate::{
    category::Case,
    gloss::{Gloss, GlossFlags, GlossStatic},
    prelude::{
        token::{OwnedConsonantForm, Token, VowelForm},
        IntoVowelForm, IntoVxCs,
    },
};

/// A case-stacking affix.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CaseStackingAffix {
    /// The case of this affix.
    pub case: Case,
}

impl CaseStackingAffix {
    /// Constructs a new [`CaseStackingAffix`].
    pub fn new(case: Case) -> Self {
        Self { case }
    }
}

impl Gloss for CaseStackingAffix {
    fn gloss(&self, flags: GlossFlags) -> String {
        let mut output = if flags.matches(GlossFlags::LONG) {
            "(case_stacking:"
        } else {
            "(case:"
        }
        .to_owned();
        output += self.case.gloss_static(flags);
        output += ")";

        output
    }
}

impl IntoVxCs for CaseStackingAffix {
    fn into_vx_cs(&self) -> (VowelForm, Token) {
        (
            {
                let mut case = self.case.into_vowel_form();
                case.has_glottal_stop = false;
                case
            },
            Token::C(OwnedConsonantForm(
                if self.case >= Case::PRN { "ly" } else { "lw" }.to_owned(),
            )),
        )
    }
}
