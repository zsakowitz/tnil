use crate::{
    category::{
        AffixualReferentList, AppositiveCase, ThematicCase, VowelFormDegree, VowelFormSequence,
    },
    gloss::{Gloss, GlossFlags, GlossStatic},
    prelude::{
        token::{OwnedConsonantForm, Token, VowelForm},
        IntoVxCs,
    },
};

/// A referential affix. The `CaseType` generic specifies the type of case this affix uses, to allow
/// this type to be reused between thematic and appositive referential affixes.
///
/// Two common variants of this type, based on thematic and appositive cases, are exposed under the
/// aliases [`ThematicReferentialAffix`] and [`AppositiveReferentialAffix`] respectively.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReferentialAffix<CaseType> {
    /// The referents and perspective of this affix.
    pub referents: AffixualReferentList,

    /// The case of this affix.
    pub case: CaseType,
}

impl<CaseType> ReferentialAffix<CaseType> {
    /// Creates a new [`ReferentialAffix`].
    pub fn new(referents: AffixualReferentList, case: CaseType) -> Self {
        Self { referents, case }
    }
}

/// A referential affix containing a [`ThematicCase`].
pub type ThematicReferentialAffix = ReferentialAffix<ThematicCase>;

/// A referential affix containing an [`AppositiveCase`].
pub type AppositiveReferentialAffix = ReferentialAffix<AppositiveCase>;

impl<CaseType: Copy + GlossStatic> Gloss for ReferentialAffix<CaseType> {
    fn gloss(&self, flags: GlossFlags) -> String {
        let mut output = "(".to_owned();
        output += &self.referents.gloss(flags);
        output += "-";
        output += self.case.gloss_static(flags);
        output += ")";
        output
    }
}

impl IntoVxCs for ThematicReferentialAffix {
    fn into_vx_cs(&self) -> (VowelForm, Token) {
        (
            VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S4,
                degree: match self.case {
                    ThematicCase::THM => VowelFormDegree::D1,
                    ThematicCase::INS => VowelFormDegree::D2,
                    ThematicCase::ABS => VowelFormDegree::D3,
                    ThematicCase::AFF => VowelFormDegree::D4,
                    ThematicCase::STM => VowelFormDegree::D5,
                    ThematicCase::EFF => VowelFormDegree::D6,
                    ThematicCase::ERG => VowelFormDegree::D7,
                    ThematicCase::DAT => VowelFormDegree::D8,
                    ThematicCase::IND => VowelFormDegree::D9,
                },
            },
            Token::C(OwnedConsonantForm(self.referents.to_string())),
        )
    }
}

impl IntoVxCs for AppositiveReferentialAffix {
    fn into_vx_cs(&self) -> (VowelForm, Token) {
        (
            VowelForm {
                has_glottal_stop: false,
                sequence: VowelFormSequence::S3,
                degree: match self.case {
                    AppositiveCase::POS => VowelFormDegree::D1,
                    AppositiveCase::PRP => VowelFormDegree::D2,
                    AppositiveCase::GEN => VowelFormDegree::D3,
                    AppositiveCase::ATT => VowelFormDegree::D4,
                    AppositiveCase::PDC => VowelFormDegree::D5,
                    AppositiveCase::ITP => VowelFormDegree::D6,
                    AppositiveCase::OGN => VowelFormDegree::D7,
                    AppositiveCase::IDP => VowelFormDegree::D8,
                    AppositiveCase::PAR => VowelFormDegree::D9,
                },
            },
            Token::C(OwnedConsonantForm(self.referents.to_string())),
        )
    }
}
