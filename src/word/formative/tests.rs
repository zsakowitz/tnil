use crate::{
    affix::AffixList,
    ca,
    category::{
        AffixShortcut, Case, CaseScope, Context, Function, NominalMode, Phase, Specification, Stem,
        Valence, Version, Vn,
    },
    gloss::{Gloss, GlossFlags},
    relation,
    romanize::{
        flags::FromTokenFlags,
        stream::{FromTokenStream, ParseError},
    },
    word::{
        formative::{
            additions::{
                GeneralFormativeAdditions, GeneralNonShortcutAdditions, NormalFormativeAdditions,
                NormalNonShortcutAdditions,
            },
            core::{GeneralFormativeCore, NormalFormativeCore},
            relation::NormalRelation,
            root::{GeneralFormativeRoot, NormalFormativeRoot},
        },
        Formative, GeneralFormative,
    },
};

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
        },
        NormalFormativeAdditions::Normal(NormalNonShortcutAdditions {
            relation: relation!(NOM, CCN, THM),
            affix_shortcut: AffixShortcut::None,
            function: Function::STA,
            specification: Specification::BSC,
            context: Context::EXS,
            slot_v_affixes: AffixList::Normal(Vec::new()),
            ca: ca!(),
            vn: Vn::Valence(Valence::MNO),
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
        },
        NormalFormativeAdditions::Normal(NormalNonShortcutAdditions {
            relation: relation!(NOM, CCN, ALL),
            affix_shortcut: AffixShortcut::None,
            function: Function::STA,
            specification: Specification::CTE,
            context: Context::RPS,
            slot_v_affixes: AffixList::Normal(Vec::new()),
            ca: ca!(),
            vn: Vn::Phase(Phase::FRE),
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

#[test]
fn parsing() -> Result<(), ParseError> {
    let formative = GeneralFormative::from_str("rrata", FromTokenFlags::NONE)?;

    assert_eq!(
        formative,
        GeneralFormative(
            GeneralFormativeCore {
                root: GeneralFormativeRoot::Normal(NormalFormativeRoot {
                    cr: "rr".to_owned(),
                }),
                stem: Some(Stem::S1),
                version: Version::PRC,
                slot_vii_affixes: AffixList::Normal(Vec::new()),
            },
            GeneralFormativeAdditions::Normal(GeneralNonShortcutAdditions {
                relation: NormalRelation::Nominal {
                    mode: NominalMode::NOM,
                    case_scope: CaseScope::CCN,
                    case: Case::THM,
                },
                affix_shortcut: Some(AffixShortcut::None),
                function: Function::STA,
                specification: Some(Specification::BSC),
                context: Context::EXS,
                slot_v_affixes: AffixList::Normal(Vec::new()),
                ca: ca!(MSS),
                vn: Vn::Valence(Valence::MNO),
            })
        )
    );

    Ok(())
}
