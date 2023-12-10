use crate::prelude::*;
use std::str::FromStr;

#[test]
fn traits() {
    fn check<T: FromStr + FromTokens + IntoTokens + Gloss>() {}

    check::<Word>();

    check::<word::CheckedFormative>();
    check::<word::ShortcutCheckedFormative>();
    check::<word::UncheckedFormative>();

    check::<word::GeneralReferential>();
    check::<word::NormalReferential>();
    check::<word::SuppletiveReferential>();

    check::<word::AffixualAdjunct>();
    check::<word::SingleAffixAdjunct>();
    check::<word::MultipleAffixAdjunct>();

    check::<word::ModularAdjunct>();
    check::<word::MCSAdjunct>();
    check::<word::ParsingAdjunct>();
    check::<word::RegisterAdjunct>();
    check::<word::SuppletiveAdjunct>();
    check::<word::BiasAdjunct>();
    check::<word::NumericAdjunct>();
}

fn check(input_word: &str, expected_gloss: &str, expected_string: &str) -> Result<(), ParseError> {
    let word: Word = input_word.parse()?;
    let gloss = word.gloss(GlossFlags::NONE);
    let string = word.to_string();

    assert_eq!(gloss, expected_gloss, "original word was {input_word:?}");
    assert_eq!(string, expected_string, "original word was {input_word:?}");

    Ok(())
}

#[test]
#[rustfmt::skip]
fn parsing_and_glossing() -> Result<(), ParseError> {
    // ..("input word",    "expected gloss",                   "expected output")?;
    // Formatives
    check("hliosulţe",     "T1-S2.N-s-lţ/9₁-ABS",              "hliosulţe"      )?;
    check("ašflaleče",     "S1-šfl-č/3₁-ABS",                  "ašflaleče"      )?;
    check("aesmlal",       "[2m+ma+1m]",                       "aesmlal"        )?;
    check("holřäksa",      "T1-S0-lř-CTE-DSC",                 "holřäks"        )?;
    check("açbala",        "S1-çb",                            "açbal"          )?;
    check("ırburučpaızya", "S2.CPT-rb-DYN-G-čp/9₁-(acc:ACT)₂", "irburučpaizya"  )?;
    check("second",        "S1-s-CSV-DSS-nd/7₁",               "second"         )?;
    check("changed",       "S1-ch-MSC.GRA-d/3₁",               "changed"        )?;
    check("alasa",         "S1-l-DPX",                         "lasa"           )?;
    check("nomic",         "S1-n-DYN.CSV-N.RPV-c/4₁",          "nomic"          )?;
    check("moved",         "S1-m-DYN.CSV-N-d/3₁",              "moved"          )?;
    check("slot",          "S1-sl-DYN.CSV-MSS-OBS",            "slot"           )?;
    check("psalaekpa",     "S1-ps-kp/0₁",                      "psalaekpa"      )?;
    check("psakpaevv",     "S1-ps-kp/0₁-N",                    "psakpaevv"      )?;
    check("oëtil",         "CPT.DYN-t/4-D4",                   "oëtil"          )?;
    check("'oëtil",        "CPT.DYN-t/4-D4",                   "oëtil"          )?;
    // Referentials
    check("lo",            "1m-ERG",                           "lo"             )?;
    check("la",            "1m",                               "la"             )?;
    check("lawe",          "1m-THM-ABS",                       "lawe"           )?;
    check("ëlawe",         "1m-THM-ABS",                       "lawe"           )?;
    check("'ëlawe",        "1m-THM-ABS",                       "lawe"           )?;
    check("ahňax",         "[PHR]-BSC",                        "ahňax"          )?;
    check("ahňaxelta",     "[PHR]-BSC-lt/3₁",                  "ahňaxelt"       )?;
    check("ahňaxeltüa",    "[PHR]-BSC-lt/3₁-THM",              "ahňaxeltüa"     )?;
    // Affixual adjuncts
    check("er",            "r/3₁",                             "er"             )?;
    check("eru",           "r/3₁-{v.sub}",                     "eru"            )?;

    Ok(())
}
