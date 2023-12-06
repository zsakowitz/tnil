use crate::{
    prelude::*,
    romanize::{
        stream::{ParseError, TokenStream},
        token::Token,
    },
};

/// A general word.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Word {
    /// A formative.
    Formative(word::ShortcutCheckedFormative),

    /// A referential.
    Referential(word::GeneralReferential),

    /// A affixual adjunct.
    Affixual(word::AffixualAdjunct),

    /// A modular adjunct.
    Modular(word::ModularAdjunct),

    /// A mcs adjunct.
    MCS(word::MCSAdjunct),

    /// A parsing adjunct.
    Parsing(word::ParsingAdjunct),

    /// A register adjunct.
    Register(word::RegisterAdjunct),

    /// A suppletive adjunct.
    Suppletive(word::SuppletiveAdjunct),

    /// A bias adjunct.
    Bias(word::BiasAdjunct),

    /// A numeric adjunct.
    Numeric(word::NumericAdjunct),
}

impl Gloss for Word {
    fn gloss(&self, flags: GlossFlags) -> String {
        match self {
            Self::Formative(value) => value.gloss(flags),
            Self::Referential(value) => value.gloss(flags),
            Self::Affixual(value) => value.gloss(flags),
            Self::Modular(value) => value.gloss(flags),
            Self::MCS(value) => value.gloss(flags),
            Self::Parsing(value) => value.gloss(flags),
            Self::Register(value) => value.gloss(flags),
            Self::Suppletive(value) => value.gloss(flags),
            Self::Bias(value) => value.gloss(flags),
            Self::Numeric(value) => value.gloss(flags),
        }
    }
}

impl FromTokens for Word {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        macro_rules! check_all {
            ($last:ident, $($variant:ident),+) => {{
                $(if let Ok(value) = stream.parse_entire(flags) {
                    return Ok(Self::$variant(value));
                })+

                let value = stream.parse(flags)?;

                if stream.is_done() {
                    Ok(Self::$last(value))
                } else {
                    Err(ParseError::TooManyTokens)
                }
            }};
        }

        // Formative structure is ((H)V)C... or ((H)V)N...  ... they can start with VCHN
        // Referential structure is (ë)C... or VH...        ... they can start with VC  ë
        // Affixual structure is V... or (ë)C... or (ë)N... ... they can start with VC Në
        // Modular structure is (H)V...                     ... they can start with V H
        // MCS structure is HV                              ... they can start with   H
        // Parsing structure is V'                          ... they can start with V
        // Register structure is HV                         ... they can start with   H
        // Suppletive structure is HV                       ... they can start with   H
        // Bias structure is C                              ... they can start with  C
        // Numeric structure is N                           ... they can start with    N

        match stream.peek() {
            Some(Token::V(_)) => check_all!(Formative, Parsing, Modular, Affixual, Referential),
            Some(Token::C(_)) => check_all!(Formative, Bias, Affixual, Referential),
            Some(Token::H(_)) => check_all!(Formative, Suppletive, Register, MCS, Modular),
            Some(Token::N(_)) => check_all!(Formative, Numeric, Affixual),
            Some(Token::Schwa) => check_all!(Referential, Affixual),
            Some(Token::ÜA) => Err(ParseError::WordInitialÜA),
            Some(Token::GlottalStop) => Err(ParseError::WordInitialGlottalStop),
            None => Err(ParseError::WordEmpty),
        }
    }
}
