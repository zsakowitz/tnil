//! Contains types related to referentials.

use crate::{
    affix::{AffixList, RegularAffix},
    category::{Case, Essence, Referent, Specification},
};
use vec1::Vec1;

pub enum Referential<T> {
    Single {
        referent: T,
        first_case: Case,
        second_case: Option<Case>,
        essence: Essence,
    },

    Dual {
        first_referent: T,
        first_case: Case,
        second_case: Case,
        second_referent: Vec1<Referent>,
        essence: Essence,
    },

    Combination {
        referent: T,
        first_case: Case,
        specification: Specification,
        affixes: Vec<RegularAffix>,
        second_case: Option<Case>,
        essence: Essence,
    },
}
