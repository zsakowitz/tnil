use crate::{
    category::{Ca, VowelFormDegree, VowelFormSequence},
    gloss::{Gloss, GlossFlags},
    prelude::{
        token::{OwnedConsonantForm, Token, VowelForm},
        IntoVxCs,
    },
};

/// A case-stacking affix.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CaStackingAffix {
    /// The Ca of this affix.
    pub ca: Ca,
}

impl CaStackingAffix {
    /// Constructs a new [`CaStackingAffix`].
    pub fn new(ca: Ca) -> Self {
        Self { ca }
    }
}

impl Gloss for CaStackingAffix {
    fn gloss(&self, flags: GlossFlags) -> String {
        let mut output = "(".to_owned();
        output += &self.ca.gloss(flags);
        output += ")";
        output
    }
}

impl IntoVxCs for CaStackingAffix {
    fn into_vx_cs(&self) -> (VowelForm, Token) {
        (
            VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S4,
                degree: VowelFormDegree::D0,
            },
            Token::C(OwnedConsonantForm(self.ca.to_ungeminated_string())),
        )
    }
}
