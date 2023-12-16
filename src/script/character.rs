//! Defines character types.

use crate::category::{
    AffixSlot, AffixType, Aspect, Ca, Case, CaseAccessorMode, CaseScope, Context, DatalessRelation,
    Effect, Function, Level, Mood, Phase, Specification, Stem, Valence, VcOrVk, Version,
};

macro_rules! item {
    (
        $enum_name:ident,
        $name:literal,
        $($variant:ident,)+
    ) => {
        ::paste::paste! {
            #[repr(u8)]
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    Ẓ,
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
    Ẓ,
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TertiarySegment {
    /// A phase segment.
    Phase(Phase),

    /// An effect segment.
    Effect(Effect),

    /// An aspect segment.
    Aspect(Aspect),
}

/// A tertiary character.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StandardQuaternary {
    /// The case, illocution, or validation of this quaternary character.
    pub vc_or_vk: VcOrVk,

    /// The case-scope of this quaternary character.
    pub case_scope: CaseScope,

    /// The mood of this quaternary character.
    pub mood: Mood,
}

/// A case-accessor quaternary character.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

/// A numeral character.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Numeral {
    /// The value of this character. If greater than 9999, extra bits will be ignored during
    /// conversion to a secondary.
    pub value: u16,
}
