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
use crate::{
    category::{NominalMode, Version, Vn},
    gloss::{Gloss, GlossFlags, GlossHelpers, GlossStatic},
    word::formative::{core::FormativeCore, relation::Relation},
    AsGeneral, TryAsSpecific,
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

impl AsGeneral<GeneralFormative> for Formative {
    fn as_general(self) -> GeneralFormative {
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

enum RootType {
    Normal,
    Numeric,
    Referential,
    Affixual,
}

struct FormativeGlossInput<'a> {
    root: String,
    stem: &'a str,
    version: Version,
    slot_vii: String,
    vn: &'a Vn,
    root_type: RootType,
    additions: &'a GeneralFormativeAdditions,
}

fn gloss_formative(data: FormativeGlossInput, flags: GlossFlags) -> String {
    let FormativeGlossInput {
        root,
        stem,
        version,
        slot_vii,
        vn,
        root_type,
        additions,
    } = data;

    enum ShortcutType {
        Normal,
        Cn,
        Ca,
    }

    let (shortcut_type, relation, ca, slot_v, function, specification, context) = match additions {
        GeneralFormativeAdditions::Normal(data) => (
            ShortcutType::Normal,
            data.relation,
            data.ca,
            data.slot_v_affixes.gloss(flags),
            data.function,
            data.specification,
            data.context,
        ),
        GeneralFormativeAdditions::CnShortcut(data) => (
            ShortcutType::Cn,
            data.relation.as_general(),
            Default::default(),
            String::new(),
            data.function,
            data.specification,
            data.context,
        ),
        GeneralFormativeAdditions::CaShortcut(data) => (
            ShortcutType::Ca,
            data.relation,
            data.ca.as_general(),
            data.slot_v_affixes.gloss(flags),
            Default::default(),
            Default::default(),
            Default::default(),
        ),
    };

    let (slot_vi, slot_viii) = match shortcut_type {
        ShortcutType::Normal => (
            {
                let ca = ca.gloss(flags);

                if ca == "" && slot_v != "" {
                    "{Ca}".to_owned()
                } else {
                    ca
                }
            },
            {
                let mut slot_viii = vn.gloss_non_default(flags);

                slot_viii.add_dotted(
                    relation
                        .mood_or_case_scope()
                        .gloss_static_non_fac_ccn(flags),
                );

                slot_viii
            },
        ),

        ShortcutType::Cn => (relation.mood_or_case_scope().gloss(flags), String::new()),

        ShortcutType::Ca => (if slot_v == "" { "" } else { "{Ca}" }.to_owned(), {
            let mut slot_viii = vn.gloss_non_default(flags);

            slot_viii.add_dotted(
                relation
                    .mood_or_case_scope()
                    .gloss_static_non_fac_ccn(flags),
            );

            slot_viii
        }),
    };

    let slots_iii_and_iv = match root_type {
        RootType::Affixual => {
            let mut output = root;

            match context.gloss_static_non_default(flags) {
                "" => {}
                value => {
                    output += ".";
                    output += value;
                }
            };

            output
        }
        _ => {
            let mut output = root;
            let mut has_added_dash = false;

            for el in [
                function.gloss_static_non_default(flags),
                specification
                    .unwrap_or_default()
                    .gloss_static_non_default(flags),
                context.gloss_static_non_default(flags),
            ] {
                if el == "" {
                    continue;
                }

                if has_added_dash {
                    output += ".";
                } else {
                    output += "-";
                    has_added_dash = true;
                }

                output += el;
            }

            output
        }
    };

    enum Mode {
        Nominal,
        Verbal,
        Framed,
        T1,
        T2,
    }

    let mode = match relation {
        Relation::Nominal { mode, .. } => match mode {
            NominalMode::T1 => Mode::T1,
            NominalMode::T2 => Mode::T2,
            NominalMode::NOM => Mode::Nominal,
            NominalMode::FRM => Mode::Framed,
        },
        Relation::Verbal { .. } => Mode::Verbal,
    };

    let slot_i = match mode {
        Mode::T1 => {
            if flags.matches(GlossFlags::LONG) {
                "type_one"
            } else {
                "T1"
            }
        }
        Mode::T2 => {
            if flags.matches(GlossFlags::LONG) {
                "type_two"
            } else {
                "T2"
            }
        }
        _ => "",
    };

    let slot_ii = match root_type {
        RootType::Normal | RootType::Numeric => {
            let mut output = version.gloss_non_default(flags);
            output.add_dotted(stem);
            if matches!(shortcut_type, ShortcutType::Ca) {
                output.add_dotted(&ca.gloss_non_default(flags));
            }
            output
        }
        RootType::Referential => {
            let mut output = version.gloss_non_default(flags);
            if matches!(shortcut_type, ShortcutType::Ca) {
                output.add_dotted(&ca.gloss_non_default(flags));
            }
            output
        }
        RootType::Affixual => {
            let mut output = version.gloss_non_default(flags);
            output.add_dotted(function.gloss_static_non_default(flags));
            output
        }
    };

    let slot_ix = match relation {
        Relation::Nominal { case, .. } => case.gloss_static_non_default(flags),

        // Illocution/Validation must always be shown for disambiguation
        Relation::Verbal { ivl, .. } => ivl.gloss_static(flags),
    };

    let slot_x = match mode {
        Mode::Nominal => ("", "\\UNF"),
        Mode::T1 => ("", ""),
        Mode::T2 => ("", ""),
        Mode::Framed => ("\\FRM", "\\FRM"),
        Mode::Verbal => ("", "\\UNF"),
    };

    let slot_x = if flags.matches(GlossFlags::SHOW_DEFAULTS) {
        slot_x.1
    } else {
        slot_x.0
    };

    let mut gloss = slot_i.to_owned();
    gloss.add_dashed(&slot_ii);
    gloss.add_dashed(&slots_iii_and_iv);
    gloss.add_dashed(&slot_v);
    gloss.add_dashed(&slot_vi);
    gloss.add_dashed(&slot_vii);
    gloss.add_dashed(&slot_viii);
    gloss.add_dashed(&slot_ix);
    gloss += &slot_x;
    gloss
}

impl Gloss for Formative {
    fn gloss(&self, flags: GlossFlags) -> String {
        let (root, stem, version, slot_vii, vn) = match self {
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
                stem.gloss_static(flags),
                *version,
                slot_vii_affixes.gloss(flags),
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
                stem.gloss_static(flags),
                *version,
                slot_vii_affixes.gloss(flags),
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
            ) => (
                root.gloss(flags),
                "",
                *version,
                slot_vii_affixes.gloss(flags),
                vn,
            ),

            Self::Affixual(
                FormativeCore {
                    root,
                    stem: _,
                    version,
                    slot_vii_affixes,
                    vn,
                },
                _,
            ) => (
                root.gloss(flags),
                "",
                *version,
                slot_vii_affixes.gloss(flags),
                vn,
            ),
        };

        let (root_type, additions) = match self {
            Formative::Normal(_, additions) => (RootType::Normal, additions.clone().as_general()),
            Formative::Numeric(_, additions) => (RootType::Numeric, additions.clone().as_general()),
            Formative::Referential(_, additions) => {
                (RootType::Referential, additions.clone().as_general())
            }
            Formative::Affixual(_, additions) => {
                (RootType::Affixual, additions.clone().as_general())
            }
        };

        gloss_formative(
            FormativeGlossInput {
                root,
                stem,
                version,
                slot_vii,
                vn,
                root_type,
                additions: &additions,
            },
            flags,
        )
    }
}

impl Gloss for GeneralFormative {
    fn gloss(&self, flags: GlossFlags) -> String {
        let root = self.0.root.gloss(flags);

        let stem = match self.0.stem {
            Some(value) => value.gloss_static(flags),
            None => "",
        };

        let version = self.0.version;

        let slot_vii = self.0.slot_vii_affixes.gloss(flags);

        let vn = &self.0.vn;

        let root_type = match self.0.root {
            GeneralFormativeRoot::Normal(_) => RootType::Normal,
            GeneralFormativeRoot::Numeric(_) => RootType::Numeric,
            GeneralFormativeRoot::Referential(_) => RootType::Referential,
            GeneralFormativeRoot::Affixual(_) => RootType::Affixual,
        };

        let additions = &self.1;

        gloss_formative(
            FormativeGlossInput {
                root,
                stem,
                version,
                slot_vii,
                vn,
                root_type,
                additions,
            },
            flags,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        affix::AffixList,
        ca,
        category::{
            AffixShortcut, Context, Function, Phase, Specification, Stem, Valence, Version, Vn,
        },
        gloss::{Gloss, GlossFlags},
        relation,
        word::formative::{
            additions::{NormalFormativeAdditions, NormalNonShortcutAdditions},
            core::NormalFormativeCore,
            root::NormalFormativeRoot,
        },
    };

    use super::Formative;

    #[test]
    fn glosses() {
        let formative = Formative::Normal(
            NormalFormativeCore {
                root: NormalFormativeRoot {
                    cr: "rr".to_string(),
                },
                stem: Stem::S1,
                version: Version::PRC,
                slot_vii_affixes: AffixList::Normal(Vec::new()),
                vn: Vn::Valence(Valence::MNO),
            },
            NormalFormativeAdditions::Normal(NormalNonShortcutAdditions {
                relation: relation!(NOM, CCN, THM),
                affix_shortcut: AffixShortcut::None,
                function: Function::STA,
                specification: Specification::BSC,
                context: Context::EXS,
                slot_v_affixes: AffixList::Normal(Vec::new()),
                ca: ca!(),
            }),
        );

        let gloss = formative.gloss(GlossFlags::NONE);

        assert_eq!(gloss, "S1-rr");

        let formative = Formative::Normal(
            NormalFormativeCore {
                root: NormalFormativeRoot {
                    cr: "rr".to_string(),
                },
                stem: Stem::S3,
                version: Version::CPT,
                slot_vii_affixes: AffixList::Normal(Vec::new()),
                vn: Vn::Phase(Phase::FRE),
            },
            NormalFormativeAdditions::Normal(NormalNonShortcutAdditions {
                relation: relation!(NOM, CCN, ALL),
                affix_shortcut: AffixShortcut::None,
                function: Function::STA,
                specification: Specification::CTE,
                context: Context::RPS,
                slot_v_affixes: AffixList::Normal(Vec::new()),
                ca: ca!(),
            }),
        );

        let gloss = formative.gloss(GlossFlags::NONE);
        let gloss_with_defaults = formative.gloss(GlossFlags::SHOW_DEFAULTS);

        assert_eq!(gloss, "CPT.S3-rr-CTE.RPS-FRE-ALL");

        assert_eq!(
            gloss_with_defaults,
            "CPT.S3-rr-STA.CTE.RPS-CSL.UPX.DEL.M.NRM-FRE.CCN-ALL\\UNF"
        );
    }
}
