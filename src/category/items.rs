use crate::{
    category::Category,
    gloss::{Gloss, GlossFlags, GlossStatic},
    AsGeneral,
};
use paste::paste;

macro_rules! item {
    (
        $enum_name:ident,
        $(($variant:ident $(= $value:literal)?, $abbr:expr, $name:expr, $short_gloss:expr, $long_gloss:expr),)+
    ) => {
        paste! {
            #[repr(u8)]
            #[doc = "Represents a " $enum_name " value."]
            #[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
            pub enum $enum_name {
                #[default]
                $(
                    #[doc = "The " $variant " (" $name ") " $enum_name "."]
                    $variant $(= $value)?,
                )+
            }
        }
    };

    (
        #no_default,
        $enum_name:ident,
        $(($variant:ident $(= $value:literal)?, $abbr:expr, $name:expr, $short_gloss:expr, $long_gloss:expr),)+
    ) => {
        paste! {
            #[repr(u8)]
            #[doc = "Represents a " $enum_name " value."]
            #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
            pub enum $enum_name {
                $(
                    #[doc = "The " $variant " (" $name ") " $enum_name "."]
                    $variant $(= $value)?,
                )+
            }
        }
    };
}

/// Creates a category, represented as an enum name followed by tuples of `(EnumVariant,
/// "abbreviation", "name of variant", "short gloss", "long gloss")`.
macro_rules! custom_category {
    (
        $(#$flag:ident,)?
        $enum_name:ident,
        $(($variant:ident $(= $value:literal)?, $abbr:expr, $name:expr, $short_gloss:expr, $long_gloss:expr),)+
    ) => {
        item!(
            $(#$flag,)?
            $enum_name,
            $(($variant $(= $value)?, $abbr, $name, $short_gloss, $long_gloss),)+
        );

        impl GlossStatic for $enum_name {
            fn gloss_static(&self, flags: GlossFlags) -> &'static str {
                if flags.matches(GlossFlags::LONG) {
                    match self {
                        $(Self::$variant => $long_gloss,)+
                    }
                } else {
                    match self {
                        $(Self::$variant => $short_gloss,)+
                    }
                }
            }
        }

        impl Category for $enum_name {
            fn abbr(self) -> &'static str {
                match self {
                    $(Self::$variant => $abbr,)+
                }
            }

            fn name(self) -> &'static str {
                match self {
                    $(Self::$variant => $name,)+
                }
            }
        }
    };
}

/// Creates a category, represented as an enum name followed by tuples of `(EnumVariant,
/// "abbreviation", "name of variant", "what to show in a long gloss")`.
macro_rules! custom_abbr_category {
    (
        $(#$flag:ident,)?
        $enum_name:ident,
        $(($variant:ident $(= $value:literal)?, $abbr:expr, $name:expr, $long_gloss:expr),)+
    ) => {
        custom_category!(
            $(#$flag,)?
            $enum_name,
            $((
                $variant $(= $value)?,
                $abbr,
                $name,
                $abbr,
                $long_gloss),
            )+
        );
    };
}

/// Creates a category, represented as an enum name followed by tuples of `(EnumVariant, "name of
/// variant", "what to show in a long gloss")`.
macro_rules! category {
    (
        $(#$flag:ident,)?
        $enum_name:ident,
        $(($variant:ident $(= $value:literal)?, $name:expr, $long_gloss:expr),)+
    ) => {
        custom_category!(
            $(#$flag,)?
            $enum_name,
            $((
                $variant $(= $value)?,
                stringify!($variant),
                $name,
                stringify!($variant),
                $long_gloss),
            )+
        );
    };
}

category!(
    NominalMode,
    (NOM, "Nominal", "nominal"),
    (T1, "Type-1 Concatenated", "type_one"),
    (T2, "Type-2 Concatenated", "type_two"),
    (UNF, "Unframed Verb", "unframed_verb"),
);

category!(
    Version,
    (PRC, "Processual", "processual"),
    (CPT, "Completive", "completive"),
);

category!(
    Stem,
    (S1 = 1, "Stem One", "stem_one"),
    (S2 = 2, "Stem Two", "stem_two"),
    (S3 = 3, "Stem Three", "stem_three"),
    (S0 = 0, "Stem Zero", "stem_zero"),
);

category!(
    Function,
    (STA, "Static", "static"),
    (DYN, "Dynamic", "dynamic"),
);

category!(
    Specification,
    (BSC, "Basic", "basic"),
    (CTE, "Contential", "contential"),
    (CSV, "Constitutive", "constitutive"),
    (OBJ, "Objective", "objective"),
);

category!(
    Context,
    (EXS, "Existential", "existential"),
    (FNC, "Functional", "functional"),
    (RPS, "Represenational", "represenational"),
    (AMG, "Amalgamative", "amalgamative"),
);

custom_category!(
    AffixType,
    (T1, "T1", "Type-1", "₁", "type_one"),
    (T2, "T2", "Type-2", "₂", "type_two"),
    (T3, "T3", "Type-3", "₃", "type_three"),
);

custom_category!(
    AffixDegree,
    (D0, "D0", "Degree 0", "0", "degree_zero"),
    (D1, "D1", "Degree 1", "1", "degree_one"),
    (D2, "D2", "Degree 2", "2", "degree_two"),
    (D3, "D3", "Degree 3", "3", "degree_three"),
    (D4, "D4", "Degree 4", "4", "degree_four"),
    (D5, "D5", "Degree 5", "5", "degree_five"),
    (D6, "D6", "Degree 6", "6", "degree_six"),
    (D7, "D7", "Degree 7", "7", "degree_seven"),
    (D8, "D8", "Degree 8", "8", "degree_eight"),
    (D9, "D9", "Degree 9", "9", "degree_nine"),
);

category!(
    Affiliation,
    (CSL, "Consolidative", "consolidative"),
    (ASO, "Associative", "associative"),
    (COA, "Coalescent", "coalescent"),
    (VAR, "Variative", "variative"),
);

#[rustfmt::skip]
category!(
    Configuration,
    (UPX, "Uniplex", "uniplex"),
    (MSS, "Multiplex Similar Separate", "multiplex_similar_separate"),
    (MSC, "Multiplex Similar Connected", "multiplex_similar_connected"),
    (MSF, "Multiplex Similar Fused", "multiplex_similar_fused"),
    (MDS, "Multiplex Dissimilar Separate", "multiplex_dissimilar_separate"),
    (MDC, "Multiplex Dissimilar Connected", "multiplex_dissimilar_connected"),
    (MDF, "Multiplex Dissimilar Fused", "multiplex_dissimilar_fused"),
    (MFS, "Multiplex Fuzzy Separate", "multiplex_fuzzy_separate"),
    (MFC, "Multiplex Fuzzy Connected", "multiplex_fuzzy_connected"),
    (MFF, "Multiplex Fuzzy Fused", "multiplex_fuzzy_fused"),
    (DPX, "Duplex", "duplex"),
    (DSS, "Duplex Similar Separate", "duplex_similar_separate"),
    (DSC, "Duplex Similar Connected", "duplex_similar_connected"),
    (DSF, "Duplex Similar Fused", "duplex_similar_fused"),
    (DDS, "Duplex Dissimilar Separate", "duplex_dissimilar_separate"),
    (DDC, "Duplex Dissimilar Connected", "duplex_dissimilar_connected"),
    (DDF, "Duplex Dissimilar Fused", "duplex_dissimilar_fused"),
    (DFS, "Duplex Fuzzy Separate", "duplex_fuzzy_separate"),
    (DFC, "Duplex Fuzzy Connected", "duplex_fuzzy_connected"),
    (DFF, "Duplex Fuzzy Fused", "duplex_fuzzy_fused"),
);

category!(
    Plexity,
    (M, "Uniplex/Multiplex", "uniplex_multiplex"),
    (D, "Duplex", "duplex"),
);

category!(
    Similarity,
    (S, "Similar", "similar"),
    (D, "Dissimilar", "dissimilar"),
    (F, "Fuzzy", "fuzzy"),
);

category!(
    Separability,
    (S, "Separate", "separate"),
    (C, "Connected", "connected"),
    (F, "Fused", "fused"),
);

category!(
    Extension,
    (DEL, "Delimitive", "delimitive"),
    (PRX, "Proximal", "proximal"),
    (ICP, "Inceptive", "inceptive"),
    (ATV, "Attenuative", "attenuative"),
    (GRA, "Graduative", "graduative"),
    (DPL, "Depletive", "depletive"),
);

category!(
    Perspective,
    (M, "Monadic", "monadic"),
    (G, "Agglomerative", "agglomerative"),
    (N, "Nomic", "nomic"),
    (A, "Abstract", "abstract"),
);

category!(
    ReferentialAffixPerspective,
    (M, "Monadic", "monadic"),
    (G, "Agglomerative", "agglomerative"),
    (N, "Nomic", "nomic"),
);

category!(
    Essence,
    (NRM, "Normal", "normal"),
    (RPV, "Representative", "representative"),
);

category!(
    Valence,
    (MNO, "Monoactive", "monoactive"),
    (PRL, "Parallel", "parallel"),
    (CRO, "Corollary", "corollary"),
    (RCP, "Reciprocal", "reciprocal"),
    (CPL, "Complementary", "complementary"),
    (DUP, "Duplicative", "duplicative"),
    (DEM, "Demonstrative", "demonstrative"),
    (CNG, "Contingent", "contingent"),
    (PTI, "Participatory", "participatory"),
);

category!(
    Phase,
    (PUN, "Punctual", "punctual"),
    (ITR, "Iterative", "iterative"),
    (REP, "Repetitive", "repetitive"),
    (ITM, "Intermittent", "intermittent"),
    (RCT, "Recurrent", "recurrent"),
    (FRE, "Frequentative", "frequentative"),
    (FRG, "Fragmentative", "fragmentative"),
    (VAC, "Vacillitative", "vacillitative"),
    (FLC, "Fluctuative", "fluctuative"),
);

#[rustfmt::skip]
custom_abbr_category!(
    Effect,
    (BEN1,    "1:BEN",   "Beneficial to Speaker",    "beneficial_to_speaker"),
    (BEN2,    "2:BEN",   "Beneficial to Addressee",  "beneficial_to_addressee"),
    (BEN3,    "3:BEN",   "Beneficial to 3rd Part",   "beneficial_to_3rd_party"),
    (BENSELF, "SLF:BEN", "Beneficial to Self",       "beneficial_to_self"),
    (UNK,     "UNK",     "Unknown",                  "unknown"),
    (DETSELF, "SLF:DET", "Detrimental to Self",      "detrimental_to_self"),
    (DET3,    "3:DET",   "Detrimental to 3rd Part",  "detrimental_to_3rd_party"),
    (DET2,    "2:DET",   "Detrimental to Addressee", "detrimental_to_addressee"),
    (DET1,    "1:DET",   "Detrimental to Speaker",   "detrimental_to_speaker"),
);

category!(
    Level,
    (MIN, "Minimal", "minimal"),
    (SBE, "Subequative", "subequative"),
    (IFR, "Inferior", "inferior"),
    (DFC, "Deficient", "deficient"),
    (EQU, "Equative", "equative"),
    (SUR, "Surpassive", "surpassive"),
    (SPL, "Superlative", "superlative"),
    (SPQ, "Superequative", "superequative"),
    (MAX, "Maximal", "maximal"),
);

category!(
    Aspect,
    (RTR, "Retrospective", "retrospective"),
    (PRS, "Prospective", "prospective"),
    (HAB, "Habitual", "habitual"),
    (PRG, "Progressive", "progressive"),
    (IMM, "Imminent", "imminent"),
    (PCS, "Precessive", "precessive"),
    (REG, "Regulative", "regulative"),
    (SMM, "Summative", "summative"),
    (ATP, "Anticipatory", "anticipatory"),
    (RSM, "Resumptive", "resumptive"),
    (CSS, "Cessative", "cessative"),
    (PAU, "Pausal", "pausal"),
    (RGR, "Regressive", "regressive"),
    (PCL, "Preclusive", "preclusive"),
    (CNT, "Continuative", "continuative"),
    (ICS, "Incessative", "incessative"),
    (EXP, "Experiential", "experiential"),
    (IRP, "Interruptive", "interruptive"),
    (PMP, "Preemptive", "preemptive"),
    (CLM, "Climactic", "climactic"),
    (DLT, "Dilatory", "dilatory"),
    (TMP, "Temporary", "temporary"),
    (XPD, "Expenditive", "expenditive"),
    (LIM, "Limitative", "limitative"),
    (EPD, "Expeditive", "expeditive"),
    (PTC, "Protractive", "protractive"),
    (PPR, "Preparatory", "preparatory"),
    (DCL, "Disclusive", "disclusive"),
    (CCL, "Conclusive", "conclusive"),
    (CUL, "Culminative", "culminative"),
    (IMD, "Intermediative", "intermediative"),
    (TRD, "Tardative", "tardative"),
    (TNS, "Transitional", "transitional"),
    (ITC, "Intercommutative", "intercommutative"),
    (MTV, "Motive", "motive"),
    (SQN, "Sequential", "sequential"),
);

category!(
    Mood,
    (FAC, "Factual", "factual"),
    (SUB, "Subjunctive", "subjunctive"),
    (ASM, "Assumptive", "assumptive"),
    (SPC, "Speculative", "speculative"),
    (COU, "Counterfactive", "counterfactive"),
    (HYP, "Hypothetical", "hypothetical"),
);

category!(
    #no_default,
    NonDefaultMood,
    (SUB, "Subjunctive", "subjunctive"),
    (ASM, "Assumptive", "assumptive"),
    (SPC, "Speculative", "speculative"),
    (COU, "Counterfactive", "counterfactive"),
    (HYP, "Hypothetical", "hypothetical"),
);

category!(
    CaseScope,
    (CCN, "Natural", "natural"),
    (CCA, "Antecedent", "antecedent"),
    (CCS, "Subaltern", "subaltern"),
    (CCQ, "Qualifier", "qualifier"),
    (CCP, "Precedent", "precedent"),
    (CCV, "Successive", "successive"),
);

category!(
    #no_default,
    NonDefaultCaseScope,
    (CCA, "Antecedent", "antecedent"),
    (CCS, "Subaltern", "subaltern"),
    (CCQ, "Qualifier", "qualifier"),
    (CCP, "Precedent", "precedent"),
    (CCV, "Successive", "successive"),
);

custom_abbr_category!(
    ReferentTarget,
    (M1, "1m", "Speaker", "speaker"),
    (M2, "2m", "Monadic Addressee", "monadic_addressee"),
    (P2, "2p", "Polyadic Addressee", "polyadic_addressee"),
    (MA, "ma", "Monadic Animate", "monadic_animate"),
    (PA, "pa", "Polyadic Animate", "polyadic_animate"),
    (MI, "mi", "Monadic Inanimate", "monadic_inanimate"),
    (PI, "pi", "Polyadic Inanimate", "polyadic_inanimate"),
    (Mx, "Mx", "Mixed 3rd Party", "mixed_3rd_party"),
    (Rdp, "Rdp", "Reduplicative", "reduplicative"),
    (Obv, "Obv", "Obviative", "obviative"),
    (PVS, "PVS", "Provisional", "provisional"),
);

category!(
    ReferentEffect,
    (NEU, "Neutral", "neutral"),
    (BEN, "Beneficial", "beneficial"),
    (DET, "Detrimental", "detrimental"),
);

category!(
    Illocution,
    (ASR, "Assertive", "assertive"),
    (DIR, "Directive", "directive"),
    (DEC, "Declarative", "declarative"),
    (IRG, "Interrogative", "interrogative"),
    (VER, "Verificative", "verificative"),
    (ADM, "Admonitive", "admonitive"),
    (POT, "Potentiative", "potentiative"),
    (HOR, "Hortative", "hortative"),
    (CNJ, "Conjectural", "conjectural"),
);

category!(
    Validation,
    (OBS, "Observational", "observational"),
    (REC, "Recollective", "recollective"),
    (PUP, "Purportive", "purportive"),
    (RPR, "Reportive", "reportive"),
    (USP, "Unspecified", "unspecified"),
    (IMA, "Imaginary", "imaginary"),
    (CVN, "Conventional", "conventional"),
    (ITU, "Intuitive", "intuitive"),
    (INF, "Inferential", "inferential"),
);

category!(
    IllocutionOrValidation,
    (OBS, "Observational", "observational"),
    (REC, "Recollective", "recollective"),
    (PUP, "Purportive", "purportive"),
    (RPR, "Reportive", "reportive"),
    (USP, "Unspecified", "unspecified"),
    (IMA, "Imaginary", "imaginary"),
    (CVN, "Conventional", "conventional"),
    (ITU, "Intuitive", "intuitive"),
    (INF, "Inferential", "inferential"),
    (DIR, "Directive", "directive"),
    (DEC, "Declarative", "declarative"),
    (IRG, "Interrogative", "interrogative"),
    (VER, "Verificative", "verificative"),
    (ADM, "Admonitive", "admonitive"),
    (POT, "Potentiative", "potentiative"),
    (HOR, "Hortative", "hortative"),
    (CNJ, "Conjectural", "conjectural"),
);

category!(
    ThematicCase,
    (THM, "Thematic", "thematic"),
    (INS, "Instrumental", "instrumental"),
    (ABS, "Absolutive", "absolutive"),
    (AFF, "Affective", "affective"),
    (STM, "Stimulative", "stimulative"),
    (EFF, "Effectuative", "effectuative"),
    (ERG, "Ergative", "ergative"),
    (DAT, "Dative", "dative"),
    (IND, "Inducive", "inducive"),
);

category!(
    AppositiveCase,
    (POS, "Possessive", "possessive"),
    (PRP, "Proprietive", "proprietive"),
    (GEN, "Genitive", "genitive"),
    (ATT, "Attributive", "attributive"),
    (PDC, "Productive", "productive"),
    (ITP, "Interpretative", "interpretative"),
    (OGN, "Originative", "originative"),
    (IDP, "Interdependent", "interdependent"),
    (PAR, "Partitive", "partitive"),
);

category!(
    Case,
    (THM, "Thematic", "thematic"),
    (INS, "Instrumental", "instrumental"),
    (ABS, "Absolutive", "absolutive"),
    (AFF, "Affective", "affective"),
    (STM, "Stimulative", "stimulative"),
    (EFF, "Effectuative", "effectuative"),
    (ERG, "Ergative", "ergative"),
    (DAT, "Dative", "dative"),
    (IND, "Inducive", "inducive"),
    (POS, "Possessive", "possessive"),
    (PRP, "Proprietive", "proprietive"),
    (GEN, "Genitive", "genitive"),
    (ATT, "Attributive", "attributive"),
    (PDC, "Productive", "productive"),
    (ITP, "Interpretative", "interpretative"),
    (OGN, "Originative", "originative"),
    (IDP, "Interdependent", "interdependent"),
    (PAR, "Partitive", "partitive"),
    (APL, "Applicative", "applicative"),
    (PUR, "Purposive", "purposive"),
    (TRA, "Transmissive", "transmissive"),
    (DFR, "Deferential", "deferential"),
    (CRS, "Contrastive", "contrastive"),
    (TSP, "Transpositive", "transpositive"),
    (CMM, "Commutative", "commutative"),
    (CMP, "Comparative", "comparative"),
    (CSD, "Considerative", "considerative"),
    (FUN, "Functive", "functive"),
    (TFM, "Transformative", "transformative"),
    (CLA, "Classificative", "classificative"),
    (RSL, "Resultative", "resultative"),
    (CSM, "Consumptive", "consumptive"),
    (CON, "Concessive", "concessive"),
    (AVR, "Aversive", "aversive"),
    (CVS, "Conversive", "conversive"),
    (SIT, "Situative", "situative"),
    (PRN, "Pertinential", "pertinential"),
    (DSP, "Descriptive", "descriptive"),
    (COR, "Correlative", "correlative"),
    (CPS, "Compositive", "compositive"),
    (COM, "Comitative", "comitative"),
    (UTL, "Utilitative", "utilitative"),
    (PRD, "Predicative", "predicative"),
    (RLT = 44, "Relative", "relative"),
    (ACT, "Activative", "activative"),
    (ASI, "Assimilative", "assimilative"),
    (ESS, "Essive", "essive"),
    (TRM, "Terminative", "terminative"),
    (SEL, "Selective", "selective"),
    (CFM, "Conformative", "conformative"),
    (DEP, "Dependent", "dependent"),
    (VOC = 53, "Vocative", "vocative"),
    (LOC, "Locative", "locative"),
    (ATD, "Attendant", "attendant"),
    (ALL, "Allative", "allative"),
    (ABL, "Ablative", "ablative"),
    (ORI, "Orientative", "orientative"),
    (IRL, "Interrelative", "interrelative"),
    (INV, "Intrative", "intrative"),
    (NAV = 62, "Navigative", "navigative"),
    (CNR, "Concursive", "concursive"),
    (ASS, "Assessive", "assessive"),
    (PER, "Periodic", "periodic"),
    (PRO, "Prolapsive", "prolapsive"),
    (PCV, "Precursive", "precursive"),
    (PCR, "Postcursive", "postcursive"),
    (ELP, "Elapsive", "elapsive"),
    (PLM = 71, "Prolimitive", "prolimitive"),
);

custom_category!(
    CaseAccessorMode,
    (Normal, "acc", "Case Accessor", "acc", "case_accessor"),
    (Inverse, "ia", "Inverse Accessor", "ia", "inverse_accessor"),
);

/// Represents a AffixShortcut value.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AffixShortcut {
    #[default]
    /// The None (None) AffixShortcut.
    None,

    /// The NEG4 (NEG/4) AffixShortcut.
    NEG4,

    /// The DCD4 (DCD/4) AffixShortcut.
    DCD4,

    /// The DCD5 (DCD/5) AffixShortcut.
    DCD5,
}

impl GlossStatic for AffixShortcut {
    fn gloss_static(&self, flags: GlossFlags) -> &'static str {
        #[cfg(feature = "sheet-affix-data")]
        if flags.matches(GlossFlags::USE_SHEET_AFFIX_DATA) {
            return match self {
                Self::None => "{none}",
                Self::NEG4 => "‘relative negation’",
                Self::DCD4 => "‘previously mentioned’",
                Self::DCD5 => "‘[+head]’",
            };
        }

        if flags.matches(GlossFlags::FORMAT_MARKDOWN) {
            return match self {
                Self::None => "{none}",
                Self::NEG4 => "**r**/4",
                Self::DCD4 => "**t**/4",
                Self::DCD5 => "**t**/5",
            };
        }

        match self {
            Self::None => "{none}",
            Self::NEG4 => "r/4",
            Self::DCD4 => "t/4",
            Self::DCD5 => "t/5",
        }
    }
}

impl Category for AffixShortcut {
    fn abbr(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::NEG4 => "NEG/4",
            Self::DCD4 => "DCD/4",
            Self::DCD5 => "DCD/5",
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::NEG4 => "NEG/4",
            Self::DCD4 => "DCD/4",
            Self::DCD5 => "DCD/5",
        }
    }
}

#[repr(u8)]
/// Represents a CaShortcut value.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CaShortcut {
    #[default]

    /// The Default (Default) CaShortcut.
    Default,

    /// The PRX (Proximal) CaShortcut.
    PRX,

    /// The G (Agglomerative) CaShortcut.
    G,

    /// The RPV (Representative) CaShortcut.
    RPV,

    /// The N (Nomic) CaShortcut.
    N,

    /// The A (Abstract) CaShortcut.
    A,

    /// The G_RPV (Agglomerative + Representative) CaShortcut.
    #[allow(non_camel_case_types)]
    G_RPV,

    /// The PRX_RPV (Proximal + Representative) CaShortcut.
    #[allow(non_camel_case_types)]
    PRX_RPV,
}

// We could implement `GlossStatic` for `CaShortcut`, but it'd have to handle four separate cases
// (long + default), and it's easier to just implement `Gloss` and leave it to `Ca` to do it
// properly.

impl Gloss for CaShortcut {
    fn gloss(&self, flags: GlossFlags) -> String {
        self.as_general().gloss(flags)
    }
}

impl Category for CaShortcut {
    fn abbr(self) -> &'static str {
        match self {
            Self::Default => "Default",
            Self::PRX => "PRX",
            Self::G => "G",
            Self::RPV => "RPV",
            Self::N => "N",
            Self::A => "A",
            Self::G_RPV => "G/RPV",
            Self::PRX_RPV => "PRX/RPV",
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Default => "Default",
            Self::PRX => "Proximal",
            Self::G => "Agglomerative",
            Self::RPV => "Representative",
            Self::N => "Nomic",
            Self::A => "Abstract",
            Self::G_RPV => "Agglomerative + Representative",
            Self::PRX_RPV => "Proximal + Representative",
        }
    }
}
