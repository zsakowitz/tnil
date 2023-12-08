use crate::prelude::*;
use std::str::FromStr;

#[test]
fn traits() {
    fn assert_is_word<T: FromStr + FromTokens + Gloss>() {}
    fn check<T: FromStr + FromTokens + IntoTokens + Gloss>() {}

    assert_is_word::<Word>();

    assert_is_word::<word::CheckedFormative>();
    assert_is_word::<word::ShortcutCheckedFormative>();
    assert_is_word::<word::UncheckedFormative>();

    assert_is_word::<word::GeneralReferential>();
    assert_is_word::<word::NormalReferential>();
    assert_is_word::<word::SuppletiveReferential>();

    assert_is_word::<word::AffixualAdjunct>();
    assert_is_word::<word::SingleAffixAdjunct>();
    assert_is_word::<word::MultipleAffixAdjunct>();

    check::<word::ModularAdjunct>();
    check::<word::MCSAdjunct>();
    check::<word::ParsingAdjunct>();
    check::<word::RegisterAdjunct>();
    check::<word::SuppletiveAdjunct>();
    check::<word::BiasAdjunct>();
    check::<word::NumericAdjunct>();
}

#[test]
fn parsing_and_glossing() -> Result<(), ParseError> {
    fn check(input_word: &str, expected_gloss: &str) -> Result<(), ParseError> {
        let word: Word = input_word.parse()?;
        let gloss = word.gloss(GlossFlags::NONE);

        assert_eq!(gloss, expected_gloss, "original word was {input_word:?}");

        Ok(())
    }

    check("hliosulţe", "T1-S2.N-s-lţ/9₁-ABS")?;
    check("ašflaleče", "S1-šfl-č/3₁-ABS")?;
    check("aesmlal", "[2m+ma+1m]")?;
    check("holřäksa", "T1-S0-lř-CTE-DSC")?;
    check("açbala", "S1-çb")?;
    check("ırburučpaızya", "S2.CPT-rb-DYN-G-čp/9₁-(acc:ACT)₂")?;
    check("second", "S1-s-CSV-DSS-nd/7₁")?;
    check("changed", "S1-ch-MSC.GRA-d/3₁")?;
    check("alasa", "S1-l-DPX")?;
    check("nomic", "S1-n-DYN.CSV-N.RPV-c/4₁")?;
    check("moved", "S1-m-DYN.CSV-N-d/3₁")?;
    check("slot", "S1-sl-DYN.CSV-MSS-OBS")?;
    check("psalaekpa", "S1-ps-kp/0₁")?;
    check("psakpaevv", "S1-ps-kp/0₁-N")?;
    check("oëtil", "CPT.DYN-t/4-D4")?;
    check("'oëtil", "CPT.DYN-t/4-D4")?;

    check("lo", "1m-ERG")?;
    check("la", "1m")?;
    check("lawe", "1m-THM-ABS")?;
    check("ëlawe", "1m-THM-ABS")?;
    check("'ëlawe", "1m-THM-ABS")?;
    check("ahňax", "[PHR]-BSC")?;
    check("ahňaxelta", "[PHR]-BSC-lt/3₁")?;
    check("ahňaxeltüa", "[PHR]-BSC-lt/3₁-THM")?;

    check("er", "r/3₁")?;
    check("eru", "r/3₁-{v.sub}")?;

    Ok(())
}
