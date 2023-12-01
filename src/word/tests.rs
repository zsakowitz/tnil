use super::{
    AffixualAdjunct, BiasAdjunct, Formative, GeneralReferential, MCSAdjunct, ModularAdjunct,
    NormalReferential, ParsingAdjunct, RegisterAdjunct, SuppletiveAdjunct, SuppletiveReferential,
};
use crate::{
    gloss::Gloss,
    word::{GeneralFormative, NumericAdjunct},
};

#[test]
fn test() {
    fn assert_gloss<T: Gloss>() {}

    assert_gloss::<Formative>();
    assert_gloss::<GeneralFormative>();
    assert_gloss::<NormalReferential>();
    assert_gloss::<SuppletiveReferential>();
    assert_gloss::<GeneralReferential>();
    assert_gloss::<AffixualAdjunct>();
    assert_gloss::<BiasAdjunct>();
    assert_gloss::<MCSAdjunct>();
    assert_gloss::<ModularAdjunct>();
    assert_gloss::<NumericAdjunct>();
    assert_gloss::<ParsingAdjunct>();
    assert_gloss::<RegisterAdjunct>();
    assert_gloss::<SuppletiveAdjunct>();
}
