use crate::{
    gloss::{Gloss, GlossFlags, GlossStatic},
    word::formative::core::FormativeCore,
    AsGeneral, TryAsSpecific,
};

use super::{
    additions::{
        AffixualFormativeAdditions, GeneralFormativeAdditions, NormalFormativeAdditions,
        ReferentialFormativeAdditions,
    },
    core::{
        AffixualFormativeCore, GeneralFormativeCore, NormalFormativeCore, NumericFormativeCore,
        ReferentialFormativeCore,
    },
    root::GeneralFormativeRoot,
};

/// A formative.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Formative {
    /// A normal formative.
    Normal(NormalFormativeCore, NormalFormativeAdditions),

    /// A numeric formative.
    Numeric(NumericFormativeCore, NormalFormativeAdditions),

    /// A referential formative.
    Referential(ReferentialFormativeCore, ReferentialFormativeAdditions),

    /// A affixual formative.
    Affixual(AffixualFormativeCore, AffixualFormativeAdditions),
}

/// A general formative.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GeneralFormative(GeneralFormativeCore, GeneralFormativeAdditions);

impl AsGeneral for Formative {
    type Output = GeneralFormative;

    fn as_general(self) -> Self::Output {
        match self {
            Self::Normal(core, additions) => {
                GeneralFormative(core.as_general(), additions.as_general())
            }

            Self::Numeric(core, additions) => {
                GeneralFormative(core.as_general(), additions.as_general())
            }

            Self::Referential(core, additions) => {
                GeneralFormative(core.as_general(), additions.as_general())
            }

            Self::Affixual(core, additions) => {
                GeneralFormative(core.as_general(), additions.as_general())
            }
        }
    }
}

impl From<Formative> for GeneralFormative {
    fn from(value: Formative) -> Self {
        value.as_general()
    }
}

impl TryAsSpecific<Formative> for GeneralFormative {
    fn try_as_specific(self) -> Option<Formative> {
        let GeneralFormative(core, additions) = self;

        match core.root {
            GeneralFormativeRoot::Normal(_) => Some(Formative::Normal(
                core.try_as_specific()?,
                additions.try_as_specific()?,
            )),

            GeneralFormativeRoot::Numeric(_) => Some(Formative::Numeric(
                core.try_as_specific()?,
                additions.try_as_specific()?,
            )),

            GeneralFormativeRoot::Referential(_) => Some(Formative::Referential(
                core.try_as_specific()?,
                additions.try_as_specific()?,
            )),

            GeneralFormativeRoot::Affixual(_) => Some(Formative::Affixual(
                core.try_as_specific()?,
                additions.try_as_specific()?,
            )),
        }
    }
}

impl Gloss for Formative {
    fn gloss(&self, flags: GlossFlags) -> String {
        // `needs_iii_iv_separator` is `false` for affixual roots and `true` for everything else.
        let (root, needs_iii_iv_separator, stem, version, slot_vii_affixes, vn) = match self {
            Self::Normal(
                FormativeCore {
                    root,
                    stem,
                    version,
                    slot_vii_affixes,
                    vn,
                },
                _,
            ) => (
                root.gloss(flags),
                true,
                stem.gloss_static(flags),
                version,
                slot_vii_affixes,
                vn,
            ),

            Self::Numeric(
                FormativeCore {
                    root,
                    stem,
                    version,
                    slot_vii_affixes,
                    vn,
                },
                _,
            ) => (
                root.gloss(flags),
                true,
                stem.gloss_static(flags),
                version,
                slot_vii_affixes,
                vn,
            ),

            Self::Referential(
                FormativeCore {
                    root,
                    stem: _,
                    version,
                    slot_vii_affixes,
                    vn,
                },
                _,
            ) => (root.gloss(flags), true, "", version, slot_vii_affixes, vn),

            Self::Affixual(
                FormativeCore {
                    root,
                    stem: _,
                    version,
                    slot_vii_affixes,
                    vn,
                },
                _,
            ) => (root.gloss(flags), false, "", version, slot_vii_affixes, vn),
        };

        // T1-PRC.S1.‘[+head]’-“root”-STA.BSC.STA-‘affix’-{Ca}-‘affix’-MNO.CCN-THM\UNF
        // __-XXX.XX._________-XXXXXX-___.___.___-_______-____-XXXXXXX

        todo!()
    }
}
