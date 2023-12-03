use super::{
    AffixualAdjunct, BiasAdjunct, Formative, GeneralReferential, MCSAdjunct, ModularAdjunct,
    NormalReferential, ParsingAdjunct, RegisterAdjunct, SuppletiveAdjunct, SuppletiveReferential,
};
use crate::{
    gloss::Gloss,
    romanize::stream::FromTokenStream,
    word::{GeneralFormative, NumericAdjunct},
};

#[test]
fn test() {
    fn assert_gloss<T: Gloss>() {}
    fn assert_is_word<T: FromTokenStream + Gloss>() {}

    assert_gloss::<Formative>();
    assert_gloss::<GeneralFormative>();
    assert_gloss::<NormalReferential>();
    assert_gloss::<SuppletiveReferential>();
    assert_gloss::<GeneralReferential>();

    assert_is_word::<AffixualAdjunct>();
    assert_is_word::<ModularAdjunct>();
    assert_is_word::<MCSAdjunct>();
    assert_is_word::<ParsingAdjunct>();
    assert_is_word::<RegisterAdjunct>();
    assert_is_word::<SuppletiveAdjunct>();
    assert_is_word::<BiasAdjunct>();
    assert_is_word::<NumericAdjunct>();
}
