use crate::{
    affix::{AffixList, PlainAffix, RegularAffix},
    ca,
    category::{
        AffixDegree, AffixShortcut, AffixType, Case, CaseScope, Context, Function,
        IllocutionOrValidation, Mood, NominalMode, NormalCaShortcut, Phase, Specification, Stem,
        Valence, Version, Vn,
    },
    gloss::{Gloss, GlossFlags},
    prelude::{IntoTokens, IntoTokensFlags},
    relation,
    romanize::{stream::ParseError, transform::normalize},
    word::{
        formative::{
            additions::{
                NormalCaShortcutAdditions, NormalFormativeAdditions, NormalNonShortcutAdditions,
                ShortcutCheckedFormativeAdditions,
            },
            core::{NormalFormativeCore, ShortcutCheckedFormativeCore},
            relation::NormalRelation,
            root::{NormalFormativeRoot, ShortcutCheckedFormativeRoot},
        },
        CheckedFormative, ShortcutCheckedFormative, UncheckedFormative,
    },
};

#[test]
fn glosses() {
    let formative = CheckedFormative::Normal(
        NormalFormativeCore {
            root: NormalFormativeRoot::new("rr"),
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

    let formative = CheckedFormative::Normal(
        NormalFormativeCore {
            root: NormalFormativeRoot::new("rr"),
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

    assert_eq!(gloss, "S3.CPT-rr-CTE.RPS-FRE-ALL");

    assert_eq!(
        gloss_with_defaults,
        "S3.CPT-rr-STA.CTE.RPS-CSL.UPX.DEL.M.NRM-FRE.CCN-ALL\\UNF"
    );
}

#[test]
fn parsing() -> Result<(), ParseError> {
    let formative: ShortcutCheckedFormative = "rrata".parse()?;

    assert_eq!(
        formative,
        ShortcutCheckedFormative(
            ShortcutCheckedFormativeCore {
                root: ShortcutCheckedFormativeRoot::new_normal("rr"),
                stem: Stem::S1,
                version: Version::PRC,
                slot_vii_affixes: AffixList::Normal(Vec::new()),
            },
            ShortcutCheckedFormativeAdditions::Normal(NormalNonShortcutAdditions {
                relation: NormalRelation::Nominal {
                    mode: NominalMode::NOM,
                    case_scope: CaseScope::CCN,
                    case: Case::THM,
                },
                affix_shortcut: AffixShortcut::None,
                function: Function::STA,
                specification: Specification::BSC,
                context: Context::EXS,
                slot_v_affixes: AffixList::Normal(Vec::new()),
                ca: ca!(MSS),
                vn: Vn::Valence(Valence::MNO),
            })
        )
    );

    let formative: ShortcutCheckedFormative = "watteteihnáu".parse()?;

    assert_eq!(
        formative,
        ShortcutCheckedFormative(
            ShortcutCheckedFormativeCore {
                root: ShortcutCheckedFormativeRoot::new_normal("tt"),
                stem: Stem::S1,
                version: Version::PRC,
                slot_vii_affixes: AffixList::Normal(vec![RegularAffix::Plain(PlainAffix {
                    cs: "t".to_owned(),
                    degree: AffixDegree::D3,
                    r#type: AffixType::T1,
                })]),
            },
            ShortcutCheckedFormativeAdditions::CaShortcut(NormalCaShortcutAdditions {
                relation: NormalRelation::Verbal {
                    mood: Mood::COU,
                    ivl: IllocutionOrValidation::DEC,
                },
                slot_v_affixes: AffixList::Normal(Vec::new()),
                ca: NormalCaShortcut::Default,
                vn: Vn::Phase(Phase::REP),
            })
        )
    );

    Ok(())
}

#[test]
fn parse_and_gloss() -> Result<(), ParseError> {
    fn check(source: &str, gloss: &str) -> Result<(), ParseError> {
        let formative: ShortcutCheckedFormative = source.parse()?;
        assert_eq!(formative.gloss(GlossFlags::NONE), gloss);

        let formative: CheckedFormative = source.parse()?;
        assert_eq!(formative.gloss(GlossFlags::NONE), gloss);

        let formative: UncheckedFormative = source.parse()?;
        assert_eq!(formative.gloss(GlossFlags::NONE), gloss);
        assert_eq!(
            formative.to_string_with(IntoTokensFlags::NONE),
            normalize(source)
        );

        Ok(())
    }

    check("hliosulţe", "T1-S2.N-s-lţ/9₁-ABS")?;
    check("ašflaleče", "S1-šfl-č/3₁-ABS")?;
    check("aesmlal", "[2m+ma+1m]")?;
    check("holřäks", "T1-S0-lř-CTE-DSC")?;
    check("açbal", "S1-çb")?;
    check("ırburučpaızya", "S2.CPT-rb-DYN-G-čp/9₁-(acc:ACT)₂")?;
    check("second", "S1-s-CSV-DSS-nd/7₁")?;
    check("changed", "S1-ch-MSC.GRA-d/3₁")?;
    check("las", "S1-l-DPX-OBS")?;
    check("nomic", "S1-n-DYN.CSV-N.RPV-c/4₁")?;
    check("moved", "S1-m-DYN.CSV-N-d/3₁")?;
    check("slot", "S1-sl-DYN.CSV-MSS-OBS")?;
    check("psalaekpa", "S1-ps-kp/0₁")?;
    check("psakpaevv", "S1-ps-kp/0₁-N")?;
    check("oëtil", "CPT.DYN-t/4-D4")?;

    Ok(())
}
