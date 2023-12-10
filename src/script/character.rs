//! Defines character types.

use crate::category::{
    AffixSlot, AffixType, Aspect, Ca, Case, CaseAccessorMode, CaseScope, Context, DatalessRelation,
    DestructuredConfiguration, Effect, Function, Level, Mood, Phase, Specification, Stem, Valence,
    VcOrVk, Version,
};

use super::traits::{IntoCharacter, IntoSecondary};

macro_rules! item {
    (
        $enum_name:ident,
        $name:literal,
        $($variant:ident,)+
    ) => {
        ::paste::paste! {
            #[repr(u8)]
            #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
            #[doc = "Represents a " $enum_name " value."]
            pub enum $enum_name {
                $(
                    #[doc = "The " $variant " " $name "."]
                    $variant,
                )+
            }
        }
    };
}

item!(
    Core,
    "secondary core",
    StandardPlaceholder,
    AlphabeticPlaceholder,
    GiantZ,
    VertBar,
    B,
    C,
    Ç,
    Č,
    D,
    Ḑ,
    F,
    G,
    H,
    J,
    K,
    L,
    Ļ,
    M,
    N,
    Ň,
    P,
    R,
    Ř,
    S,
    Š,
    T,
    Ţ,
    V,
    X,
    Z,
    Ż,
    Ž,
    Sigma,
    BSC,
    CTE,
    CSV,
    OBJ,
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
);

item!(
    Ext,
    "secondary extension",
    B,
    C,
    Ç,
    Č,
    D,
    DWithLine,
    Ḑ,
    F,
    G,
    GWithLine,
    H,
    J,
    K,
    L,
    Ļ,
    M,
    N,
    Ň,
    P,
    PWithLine,
    R,
    RFlipped,
    Ř,
    S,
    Š,
    T,
    Ţ,
    V,
    W,
    X,
    Y,
    Z,
    Ż,
    Ž,
    GeminateCore,
    GeminateExt,
    GlottalStop,
    Ejective,
    Velar,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
);

item!(
    Register,
    "register",
    StandardPNT,
    StandardSPF,
    StandardEXM,
    StandardCGT,
    AlphabeticNRR,
    AlphabeticDSV,
    AlphabeticPNT,
    AlphabeticSPF,
    AlphabeticEXM,
    AlphabeticCGT,
    TranscriptiveNRR,
    TranscriptiveDSV,
    TranscriptivePNT,
    TranscriptiveSPF,
    TranscriptiveEXM,
    TranscriptiveCGT,
    TransliterativeNRR,
    TransliterativeDSV,
    TransliterativePNT,
    TransliterativeSPF,
    TransliterativeEXM,
    TransliterativeCGT,
);

item!(
    Diacritic,
    "diacritic",
    Dot,
    HorizBar,
    HorizBarWithTopLine,
    HorizBarWithBottomLine,
    VertBar,
    VertBarWithLeftLine,
    VertBarWithRightLine,
    DiagBar,
    TwoPartHorizAndDiagBar,
    CurveTowardsLeft,
    CurveTowardsLeftWithDot,
    CurveTowardsRight,
    CurveTowardsRightWithDot,
    CurveTowardsTop,
    CurveTowardsBottom,
    CurveTowardsBottomWithLine,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
);

/// A secondary character.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Secondary {
    /// Whether this character is rotated.
    pub is_rotated: bool,

    /// The core of this character.
    pub core: Core,

    /// The top extension of this character.
    pub top: Option<Ext>,

    /// The bottom extension of this character.
    pub bottom: Option<Ext>,

    /// The diacritic superposed on this character.
    pub superposed: Option<Diacritic>,

    /// The diacritic underposed on this character.
    pub underposed: Option<Diacritic>,

    /// The diacritic leftposed on this character.
    pub leftposed: Option<Diacritic>,

    /// The diacritic rightposed on this character.
    pub rightposed: Option<Diacritic>,
}

/// A segment in a tertiary character.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TertiarySegment {
    /// A phase segment.
    Phase(Phase),

    /// An effect segment.
    Effect(Effect),

    /// An aspect segment.
    Aspect(Aspect),
}

/// A tertiary character.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tertiary {
    /// The valence of this character.
    pub valence: Valence,

    /// The top segment of this character.
    pub top: Option<TertiarySegment>,

    /// The bottom segment of this character.
    pub bottom: Option<TertiarySegment>,

    /// The level diacritic superposed on this character.
    pub superposed: Option<Level>,

    /// The level diacritic underposed on this character.
    pub underposed: Option<Level>,
}

/// A general character.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Character {
    /// A secondary character.
    Secondary(Secondary),

    /// A tertiary character.
    Tertiary(Tertiary),

    /// A register character.
    Register(Register),

    /// A diacritic character.
    Diacritic(Diacritic),

    /// A word break.
    WordBreak,

    /// A sentence break.
    SentenceBreak,
}

/// A primary character.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Primary {
    /// The specification of this primary.
    pub specification: Specification,

    /// The ca of this primary.
    pub ca: Ca,

    /// The function of this primary.
    pub function: Function,

    /// The version of this primary.
    pub version: Version,

    /// The stem of this primary.
    pub stem: Stem,

    /// The context of this primary.
    pub context: Context,

    /// The relation of this primary.
    pub relation: DatalessRelation,
}

/// A standard quaternary character.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct StandardQuaternary {
    /// The case, illocution, or validation of this quaternary character.
    pub vc_or_vk: VcOrVk,

    /// The case-scope of this quaternary character.
    pub case_scope: CaseScope,

    /// The mood of this quaternary character.
    pub mood: Mood,
}

/// A case-accessor quaternary character.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccessorQuaternary {
    /// The slot this case-accessor is in.
    pub slot: AffixSlot,

    /// The case of this quaternary character.
    pub case: Case,

    /// The mode of this case-accessor.
    pub mode: CaseAccessorMode,

    /// The type of this case-accessor.
    pub r#type: AffixType,
}

impl IntoSecondary for Secondary {
    fn into_secondary(self) -> Secondary {
        self
    }
}

impl IntoCharacter for Tertiary {
    fn into_character(self) -> Character {
        Character::Tertiary(self)
    }
}

impl IntoCharacter for Register {
    fn into_character(self) -> Character {
        Character::Register(self)
    }
}

impl IntoCharacter for Diacritic {
    fn into_character(self) -> Character {
        Character::Diacritic(self)
    }
}

impl IntoSecondary for Primary {
    fn into_secondary(self) -> Secondary {
        let DestructuredConfiguration {
            plexity,
            similarity_and_separability,
        } = self.ca.configuration.destructure();

        Secondary {
            is_rotated: false,
            core: Core::primary_core(self.specification),
            top: Ext::primary_top(self.ca.extension, self.ca.perspective),
            bottom: Ext::primary_bottom(self.function, self.version, plexity, self.stem),
            superposed: Diacritic::primary_superposed(self.context),
            underposed: Diacritic::primary_underposed(self.relation),
            leftposed: Diacritic::primary_leftposed(similarity_and_separability),
            rightposed: Diacritic::primary_rightposed(self.ca.affiliation, self.ca.essence),
        }
    }
}

impl IntoSecondary for StandardQuaternary {
    fn into_secondary(self) -> Secondary {
        let (top, bottom) = Ext::standard_quaternary_exts(self.vc_or_vk);

        Secondary {
            is_rotated: false,
            core: Core::VertBar,
            top,
            bottom,
            superposed: Diacritic::standard_quaternary_superposed(self.mood),
            underposed: Diacritic::standard_quaternary_underposed(self.case_scope),
            leftposed: None,
            rightposed: None,
        }
    }
}

impl IntoSecondary for AccessorQuaternary {
    fn into_secondary(self) -> Secondary {
        let (top, bottom) = Ext::accessor_quaternary_exts(self.case);

        Secondary {
            is_rotated: false,
            core: Core::VertBar,
            top,
            bottom,
            superposed: Diacritic::affix_type_superposed(self.r#type),
            underposed: Diacritic::accessor_quaternary_underposed(self.slot, self.mode).into(),
            leftposed: None,
            rightposed: None,
        }
    }
}
