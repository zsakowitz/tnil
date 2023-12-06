use super::{
    AffixualAdjunct, BiasAdjunct, Formative, GeneralReferential, MCSAdjunct, ModularAdjunct,
    NormalReferential, ParsingAdjunct, RegisterAdjunct, SuppletiveAdjunct, SuppletiveReferential,
};
use crate::{
    gloss::Gloss,
    romanize::stream::FromTokenStream,
    word::{GeneralFormative, NumericAdjunct},
};
use std::str::FromStr;

#[test]
fn test() {
    fn assert_is_word<T: FromStr + FromTokenStream + Gloss>() {}

    assert_is_word::<Formative>();
    assert_is_word::<GeneralFormative>();

    assert_is_word::<NormalReferential>();
    assert_is_word::<SuppletiveReferential>();
    assert_is_word::<GeneralReferential>();

    assert_is_word::<AffixualAdjunct>();
    assert_is_word::<ModularAdjunct>();
    assert_is_word::<MCSAdjunct>();
    assert_is_word::<ParsingAdjunct>();
    assert_is_word::<RegisterAdjunct>();
    assert_is_word::<SuppletiveAdjunct>();
    assert_is_word::<BiasAdjunct>();
    assert_is_word::<NumericAdjunct>();
}
