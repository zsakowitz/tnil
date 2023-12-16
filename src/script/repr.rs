//! Contains ways to represent Ithkuil script.

use super::{
    character::{Core, Diacritic, Ext, Register, Secondary, Tertiary, TertiarySegment},
    traits::ScriptRepr,
};
use crate::{
    category::{Aspect, Effect, Phase, Valence},
    prelude::Character,
};
use std::fmt::Display;

/// A newtype over `String` containing the encoding used in the IthkuilBasic and IthkuilFlow fonts.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IthkuilBasicEncoding(pub String);

impl IthkuilBasicEncoding {
    /// Gets the string used to represent a core in this encoding.
    pub const fn repr_core(core: Core) -> &'static str {
        match core {
            Core::AlphabeticPlaceholder => "{",
            Core::StandardPlaceholder => "}",
            Core::GiantZ => "Z",
            Core::VertBar => "|",
            Core::B => "b",
            Core::C => "c",
            Core::Ç => "ç",
            Core::Č => "č",
            Core::D => "d",
            Core::Ḑ => "ḍ",
            Core::F => "f",
            Core::G => "g",
            Core::H => "h",
            Core::J => "j",
            Core::K => "k",
            Core::L => "l",
            Core::Ļ => "ļ",
            Core::M => "m",
            Core::N => "n",
            Core::Ň => "ň",
            Core::P => "p",
            Core::R => "r",
            Core::Ř => "ř",
            Core::S => "s",
            Core::Š => "š",
            Core::T => "t",
            Core::Ţ => "ţ",
            Core::V => "v",
            Core::X => "x",
            Core::Z => "z",
            Core::Ẓ => "ż",
            Core::Ž => "ž",
            Core::Sigma => "Ʃ",
            Core::BSC => "\\",
            Core::CTE => ":",
            Core::CSV => "(",
            Core::OBJ => ")",
            Core::N0 => "0",
            Core::N1 => "1",
            Core::N2 => "2",
            Core::N3 => "3",
            Core::N4 => "4",
            Core::N5 => "5",
            Core::N6 => "6",
            Core::N7 => "7",
            Core::N8 => "8",
            Core::N9 => "9",
        }
    }

    /// Gets the string used to represent an extension in this encoding.
    pub const fn repr_ext(ext: Ext) -> &'static str {
        match ext {
            Ext::B => "b",
            Ext::C => "c",
            Ext::D => "d",
            Ext::F => "f",
            Ext::G => "g",
            Ext::H => "h",
            Ext::J => "j",
            Ext::K => "k",
            Ext::L => "l",
            Ext::M => "m",
            Ext::N => "n",
            Ext::P => "p",
            Ext::R => "r",
            Ext::S => "s",
            Ext::T => "t",
            Ext::V => "v",
            Ext::X => "x",
            Ext::Z => "z",
            Ext::Ç => "ç",
            Ext::Č => "č",
            Ext::Ļ => "ļ",
            Ext::Ň => "ň",
            Ext::Ř => "ř",
            Ext::Š => "š",
            Ext::Ţ => "ţ",
            Ext::Ẓ => "ż",
            Ext::Ž => "ž",
            Ext::Ḑ => "ḍ",
            Ext::W => "w",
            Ext::Y => "y",
            Ext::DWithLine => "D",
            Ext::GWithLine => "G",
            Ext::PWithLine => "P",
            Ext::RFlipped => "ɹ",
            Ext::GlottalStop => "ʔ",
            Ext::Ejective => "ʖ",
            Ext::Velar => "ʕ",
            Ext::GeminateCore => "=",
            Ext::GeminateExt => "≈",
            Ext::N1 => "1",
            Ext::N2 => "2",
            Ext::N3 => "3",
            Ext::N4 => "4",
            Ext::N5 => "5",
            Ext::N6 => "6",
            Ext::N7 => "7",
            Ext::N8 => "8",
            Ext::N9 => "9",
        }
    }

    /// Gets the string used to represent a diacritic in this encoding.
    pub const fn repr_diacritic(diacritic: Diacritic) -> &'static str {
        match diacritic {
            Diacritic::Dot => "a",
            Diacritic::HorizBar => "ä",
            Diacritic::HorizBarWithBottomLine => "e",
            Diacritic::HorizBarWithTopLine => "ë",
            Diacritic::DiagBar => "i",
            Diacritic::VertBar => "ï",
            Diacritic::CurveTowardsTop => "o",
            Diacritic::CurveTowardsBottom => "ö",
            Diacritic::CurveTowardsBottomWithLine => "ő",
            Diacritic::VertBarWithRightLine => "u",
            Diacritic::VertBarWithLeftLine => "ü",
            Diacritic::CurveTowardsLeft => "ò",
            Diacritic::CurveTowardsRight => "ó",
            Diacritic::TwoPartHorizAndDiagBar => "äi",
            Diacritic::CurveTowardsLeftWithDot => "aò",
            Diacritic::CurveTowardsRightWithDot => "aó",
            Diacritic::N1 => "1",
            Diacritic::N2 => "2",
            Diacritic::N3 => "3",
            Diacritic::N4 => "4",
            Diacritic::N5 => "5",
            Diacritic::N6 => "6",
            Diacritic::N7 => "7",
            Diacritic::N8 => "8",
            Diacritic::N9 => "9",
        }
    }

    /// Gets the string used to represent a valence segment in this encoding.
    pub const fn repr_valence(valence: Valence) -> &'static str {
        match valence {
            Valence::MNO => "≡aa",
            Valence::PRL => "≡az",
            Valence::CRO => "≡za",
            Valence::RCP => "≡zz",
            Valence::CPL => "≡sa",
            Valence::DUP => "≡as",
            Valence::DEM => "≡af",
            Valence::CNG => "≡av",
            Valence::PTI => "≡sf",
        }
    }

    /// Gets the string used to represent a tertiary segment in this encoding.
    pub const fn repr_tertiary_segment(segment: TertiarySegment) -> &'static str {
        match segment {
            TertiarySegment::Phase(Phase::PUN) => "⋮aïa",
            TertiarySegment::Phase(Phase::FLC) => "⋮aïp",
            TertiarySegment::Phase(Phase::FRE) => "⋮pïs",
            TertiarySegment::Phase(Phase::FRG) => "⋮sïp",
            TertiarySegment::Phase(Phase::ITM) => "⋮sïs",
            TertiarySegment::Phase(Phase::ITR) => "⋮sïa",
            TertiarySegment::Phase(Phase::RCT) => "⋮aïz",
            TertiarySegment::Phase(Phase::REP) => "⋮aïs",
            TertiarySegment::Phase(Phase::VAC) => "⋮pïa",

            TertiarySegment::Effect(Effect::BEN1) => "⋮aia",
            TertiarySegment::Effect(Effect::DET1) => "⋮aim",
            TertiarySegment::Effect(Effect::BEN2) => "⋮sia",
            TertiarySegment::Effect(Effect::DET2) => "⋮mia",
            TertiarySegment::Effect(Effect::BEN3) => "⋮ais",
            TertiarySegment::Effect(Effect::DET3) => "⋮sim",
            TertiarySegment::Effect(Effect::BENSELF) => "⋮sis",
            TertiarySegment::Effect(Effect::DETSELF) => "⋮aiz",
            TertiarySegment::Effect(Effect::UNK) => "⋮zia",

            TertiarySegment::Aspect(Aspect::ATP) => "⋮fäz",
            TertiarySegment::Aspect(Aspect::CCL) => "⋮gäa",
            TertiarySegment::Aspect(Aspect::CLM) => "⋮aäg",
            TertiarySegment::Aspect(Aspect::CNT) => "⋮ḍäs",
            TertiarySegment::Aspect(Aspect::CSS) => "⋮ţäa",
            TertiarySegment::Aspect(Aspect::CUL) => "⋮≈äa",
            TertiarySegment::Aspect(Aspect::DCL) => "⋮käa",
            TertiarySegment::Aspect(Aspect::DLT) => "⋮aä≈",
            TertiarySegment::Aspect(Aspect::EPD) => "⋮aäř",
            TertiarySegment::Aspect(Aspect::EXP) => "⋮iäf",
            TertiarySegment::Aspect(Aspect::HAB) => "⋮aäḍ",
            TertiarySegment::Aspect(Aspect::ICS) => "⋮aäf",
            TertiarySegment::Aspect(Aspect::IMD) => "⋮šäa",
            TertiarySegment::Aspect(Aspect::IMM) => "⋮säţ",
            TertiarySegment::Aspect(Aspect::IRP) => "⋮zäf",
            TertiarySegment::Aspect(Aspect::ITC) => "⋮řäa",
            TertiarySegment::Aspect(Aspect::LIM) => "⋮aät",
            TertiarySegment::Aspect(Aspect::MTV) => "⋮xäa",
            TertiarySegment::Aspect(Aspect::PAU) => "⋮ḍäa",
            TertiarySegment::Aspect(Aspect::PCL) => "⋮ţäs",
            TertiarySegment::Aspect(Aspect::PCS) => "⋮säḍ",
            TertiarySegment::Aspect(Aspect::PMP) => "⋮aäk",
            TertiarySegment::Aspect(Aspect::PPR) => "⋮aä=",
            TertiarySegment::Aspect(Aspect::PRG) => "⋮säi",
            TertiarySegment::Aspect(Aspect::PRS) => "⋮aäţ",
            TertiarySegment::Aspect(Aspect::PTC) => "⋮aäx",
            TertiarySegment::Aspect(Aspect::REG) => "⋮fäa",
            TertiarySegment::Aspect(Aspect::RGR) => "⋮iäs",
            TertiarySegment::Aspect(Aspect::RSM) => "⋮iäa",
            TertiarySegment::Aspect(Aspect::RTR) => "⋮aäi",
            TertiarySegment::Aspect(Aspect::SMM) => "⋮fäi",
            TertiarySegment::Aspect(Aspect::SQN) => "⋮=äa",
            TertiarySegment::Aspect(Aspect::TMP) => "⋮aäš",
            TertiarySegment::Aspect(Aspect::TNS) => "⋮täa",
            TertiarySegment::Aspect(Aspect::TRD) => "⋮päa",
            TertiarySegment::Aspect(Aspect::XPD) => "⋮aäp",
        }
    }

    /// Gets the string used to represent a register in this encoding.
    pub const fn repr_register(register: Register) -> &'static str {
        match register {
            Register::StandardPNT => "·02",
            Register::StandardCGT => "·03",
            Register::StandardEXM => "·04",
            Register::StandardSPF => "·05",

            Register::AlphabeticNRR => "·10",
            Register::AlphabeticDSV => "·11",
            Register::AlphabeticPNT => "·12",
            Register::AlphabeticCGT => "·13",
            Register::AlphabeticEXM => "·14",
            Register::AlphabeticSPF => "·15",

            Register::TranscriptiveNRR => "·20",
            Register::TranscriptiveDSV => "·21",
            Register::TranscriptivePNT => "·22",
            Register::TranscriptiveCGT => "·23",
            Register::TranscriptiveEXM => "·24",
            Register::TranscriptiveSPF => "·25",

            Register::TransliterativeNRR => "·30",
            Register::TransliterativeDSV => "·31",
            Register::TransliterativePNT => "·32",
            Register::TransliterativeCGT => "·33",
            Register::TransliterativeEXM => "·34",
            Register::TransliterativeSPF => "·35",
        }
    }
}

impl Display for IthkuilBasicEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl ScriptRepr for IthkuilBasicEncoding {
    fn new() -> Self {
        Self(String::new())
    }

    fn push(&mut self, char: Character) {
        match char {
            Character::Secondary(Secondary {
                is_rotated,
                core,
                top,
                bottom,
                superposed,
                underposed,
                leftposed,
                rightposed,
            }) => {
                self.0 += Self::repr_core(core);

                if is_rotated {
                    self.0 += "'";
                }

                if let Some(top) = top {
                    self.0 += "^";
                    self.0 += Self::repr_ext(top);
                }

                if let Some(bottom) = bottom {
                    self.0 += "_";
                    self.0 += Self::repr_ext(bottom);
                }

                if let Some(superposed) = superposed {
                    self.0 += "^";
                    self.0 += Self::repr_diacritic(superposed);
                }

                if let Some(underposed) = underposed {
                    self.0 += "_";
                    self.0 += Self::repr_diacritic(underposed);
                }

                if let Some(leftposed) = leftposed {
                    self.0 += "<";
                    self.0 += Self::repr_diacritic(leftposed);
                }

                if let Some(rightposed) = rightposed {
                    self.0 += ">";
                    self.0 += Self::repr_diacritic(rightposed);
                }
            }

            Character::Tertiary(Tertiary {
                valence,
                top,
                bottom,
                superposed,
                underposed,
            }) => {
                self.0 += Self::repr_valence(valence);

                if let Some(top) = top {
                    self.0 += "^";
                    self.0 += Self::repr_tertiary_segment(top);
                }

                if let Some(bottom) = bottom {
                    self.0 += "_";
                    self.0 += Self::repr_tertiary_segment(bottom);
                }

                if let Some(superposed) = superposed {
                    self.0 += "^";
                    self.0 += Self::repr_diacritic(Diacritic::level(superposed));
                }

                if let Some(underposed) = underposed {
                    self.0 += "_";
                    self.0 += Self::repr_diacritic(Diacritic::level(underposed));
                }
            }

            Character::Diacritic(diacritic) => {
                self.0 += ";<";
                self.0 += Self::repr_diacritic(diacritic);
            }

            Character::Register(register) => {
                self.0 += Self::repr_register(register);
            }

            Character::WordBreak => {}

            Character::SentenceBreak => {
                if !self.0.is_empty() {
                    self.0 += " "
                }
            }
        }
    }
}
