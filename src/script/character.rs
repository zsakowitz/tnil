//! Defines character types.

use crate::category::{Aspect, Effect, Level, Phase, Valence};

macro_rules! item {
    (
        $enum_name:ident,
        $name:literal,
        $($variant:ident,)+
    ) => {
        ::paste::paste! {
            #[repr(u8)]
            #[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
            #[doc = "Represents a " $enum_name " value."]
            pub enum $enum_name {
                #[default]
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
    TonalPlaceholder,
    StressedSyllablePlaceholder,
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
    Extension,
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
    CoreGeminate,
    ExtensionGeminate,
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
    HorizontalBar,
    HorizontalBarWithTopLine,
    HorizontalBarWithBottomLine,
    VerticalBar,
    VerticalBarWithLeftLine,
    VerticalBarWithRightLine,
    DiagonalBar,
    TwoPartHorizontalAndDiagonalBar,
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Secondary {
    /// Whether this character is rotated.
    pub is_rotated: bool,

    /// The core of this character.
    pub core: Core,

    /// The top extension of this character.
    pub top: Option<Extension>,

    /// The bottom extension of this character.
    pub bottom: Option<Extension>,

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
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
