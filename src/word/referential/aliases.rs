use super::{GeneralReferent, Referential};
use crate::{
    category::{NormalReferentList, SuppletiveAdjunctMode},
    romanize::{
        flags::FromTokenFlags,
        stream::{FromTokenStream, ParseError, TokenStream},
    },
};

/// A normal referential (i.e. one where the initial referent is an actual referent).
pub type NormalReferential = Referential<NormalReferentList>;

/// A suppletive referential (i.e. one where the initial referent is a suppletive adjunct mode).
pub type SuppletiveReferential = Referential<SuppletiveAdjunctMode>;

/// A general referential.
pub type GeneralReferential = Referential<GeneralReferent>;

impl FromTokenStream for NormalReferential {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        let consonants = String::new();
        todo!()
    }
}
