use super::{
    additions::{
        AffixualFormativeAdditions, GeneralFormativeAdditions, NormalFormativeAdditions,
        ReferentialFormativeAdditions,
    },
    core::{
        AffixualFormativeCore, GeneralFormativeCore, NormalFormativeCore, NumericFormativeCore,
        ReferentialFormativeCore,
    },
    root::GeneralFormativeRoot,
};
use crate::{
    affix::AffixList,
    category::{
        AffixShortcut, ArbitraryMoodOrCaseScope, Ca, Case, Context, Function, HFormDegree,
        HFormSequence, IllocutionOrValidation, Mood, NominalMode, NormalCaShortcut, Specification,
        Stem, Stress, Valence, Version, Vn, VowelFormDegree, VowelFormSequence,
    },
    gloss::{Gloss, GlossFlags, GlossHelpers, GlossStatic},
    romanize::{
        flags::FromTokenFlags,
        stream::{FromTokenStream, ParseError, TokenStream},
        token::{HForm, NumeralForm, OwnedConsonantForm, Token, VowelForm},
    },
    specificity::{AsGeneral, AsSpecific, TryAsSpecific},
    word::formative::{
        additions::{
            GeneralCnShortcutAdditions, GeneralNonShortcutAdditions, NormalCaShortcutAdditions,
        },
        core::FormativeCore,
        relation::Relation,
        root::{AffixualFormativeRoot, NormalFormativeRoot, NumericFormativeRoot},
    },
};

/// A formative.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Formative {
    /// A normal formative.
    Normal(NormalFormativeCore, NormalFormativeAdditions),

    /// A numeric formative.
    Numeric(NumericFormativeCore, NormalFormativeAdditions),

    /// A referential formative.
    Referential(ReferentialFormativeCore, ReferentialFormativeAdditions),

    /// A affixual formative.
    Affixual(AffixualFormativeCore, AffixualFormativeAdditions),
}

/// A general formative.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GeneralFormative(pub GeneralFormativeCore, pub GeneralFormativeAdditions);

impl AsGeneral<GeneralFormative> for Formative {
    fn as_general(self) -> GeneralFormative {
        match self {
            Self::Normal(core, additions) => {
                GeneralFormative(core.as_general(), additions.as_general())
            }

            Self::Numeric(core, additions) => {
                GeneralFormative(core.as_general(), additions.as_general())
            }

            Self::Referential(core, additions) => {
                GeneralFormative(core.as_general(), additions.as_general())
            }

            Self::Affixual(core, additions) => {
                GeneralFormative(core.as_general(), additions.as_general())
            }
        }
    }
}

impl From<Formative> for GeneralFormative {
    fn from(value: Formative) -> Self {
        value.as_general()
    }
}

impl TryAsSpecific<Formative> for GeneralFormative {
    fn try_as_specific(self) -> Option<Formative> {
        let GeneralFormative(core, additions) = self;

        match core.root {
            GeneralFormativeRoot::Normal(_) => Some(Formative::Normal(
                core.try_as_specific()?,
                additions.try_as_specific()?,
            )),

            GeneralFormativeRoot::Numeric(_) => Some(Formative::Numeric(
                core.try_as_specific()?,
                additions.try_as_specific()?,
            )),

            GeneralFormativeRoot::Referential(_) => Some(Formative::Referential(
                core.try_as_specific()?,
                additions.try_as_specific()?,
            )),

            GeneralFormativeRoot::Affixual(_) => Some(Formative::Affixual(
                core.try_as_specific()?,
                additions.try_as_specific()?,
            )),
        }
    }
}

enum RootType {
    Normal,
    Numeric,
    Referential,
    Affixual,
}

struct FormativeGlossInput<'a> {
    root: String,
    stem: &'a str,
    version: Version,
    slot_vii: String,
    root_type: RootType,
    additions: &'a GeneralFormativeAdditions,
}

fn gloss_formative(data: FormativeGlossInput, flags: GlossFlags) -> String {
    let FormativeGlossInput {
        root,
        stem,
        version,
        slot_vii,
        root_type,
        additions,
    } = data;

    enum ShortcutType {
        Normal,
        Cn,
        Ca,
    }

    let (shortcut_type, relation, ca, slot_v, function, specification, context, vn) =
        match additions {
            GeneralFormativeAdditions::Normal(data) => (
                ShortcutType::Normal,
                data.relation,
                data.ca,
                data.slot_v_affixes.gloss(flags),
                data.function,
                data.specification,
                data.context,
                Some(data.vn),
            ),
            GeneralFormativeAdditions::CnShortcut(data) => (
                ShortcutType::Cn,
                data.relation.as_general(),
                Default::default(),
                String::new(),
                data.function,
                data.specification,
                data.context,
                None,
            ),
            GeneralFormativeAdditions::CaShortcut(data) => (
                ShortcutType::Ca,
                data.relation,
                data.ca.as_general(),
                data.slot_v_affixes.gloss(flags),
                Default::default(),
                Default::default(),
                Default::default(),
                Some(data.vn),
            ),
        };

    let (slot_vi, slot_viii) = match shortcut_type {
        ShortcutType::Normal => (
            {
                let ca = ca.gloss(flags);

                if ca == "" && slot_v != "" {
                    "{Ca}".to_owned()
                } else {
                    ca
                }
            },
            {
                let mut slot_viii = match vn {
                    Some(vn) => vn.gloss_non_default(flags),
                    None => "".to_owned(),
                };

                slot_viii.add_dotted(
                    relation
                        .mood_or_case_scope()
                        .gloss_static_non_fac_ccn(flags),
                );

                slot_viii
            },
        ),

        ShortcutType::Cn => (relation.mood_or_case_scope().gloss(flags), String::new()),

        ShortcutType::Ca => (if slot_v == "" { "" } else { "{Ca}" }.to_owned(), {
            let mut slot_viii = match vn {
                Some(vn) => vn.gloss_non_default(flags),
                None => "".to_owned(),
            };

            slot_viii.add_dotted(
                relation
                    .mood_or_case_scope()
                    .gloss_static_non_fac_ccn(flags),
            );

            slot_viii
        }),
    };

    let slots_iii_and_iv = match root_type {
        RootType::Affixual => {
            let mut output = root;

            match context.gloss_static_non_default(flags) {
                "" => {}
                value => {
                    output += ".";
                    output += value;
                }
            };

            output
        }
        _ => {
            let mut output = root;
            let mut has_added_dash = false;

            for el in [
                function.gloss_static_non_default(flags),
                specification
                    .unwrap_or_default()
                    .gloss_static_non_default(flags),
                context.gloss_static_non_default(flags),
            ] {
                if el == "" {
                    continue;
                }

                if has_added_dash {
                    output += ".";
                } else {
                    output += "-";
                    has_added_dash = true;
                }

                output += el;
            }

            output
        }
    };

    enum Mode {
        Nominal,
        Verbal,
        Framed,
        T1,
        T2,
    }

    let mode = match relation {
        Relation::Nominal { mode, .. } => match mode {
            NominalMode::T1 => Mode::T1,
            NominalMode::T2 => Mode::T2,
            NominalMode::NOM => Mode::Nominal,
            NominalMode::FRM => Mode::Framed,
        },
        Relation::Verbal { .. } => Mode::Verbal,
    };

    let slot_i = match mode {
        Mode::T1 => {
            if flags.matches(GlossFlags::LONG) {
                "type_one"
            } else {
                "T1"
            }
        }
        Mode::T2 => {
            if flags.matches(GlossFlags::LONG) {
                "type_two"
            } else {
                "T2"
            }
        }
        _ => "",
    };

    let slot_ii = match root_type {
        RootType::Normal | RootType::Numeric => {
            let mut output = version.gloss_non_default(flags);
            output.add_dotted(stem);
            if matches!(shortcut_type, ShortcutType::Ca) {
                output.add_dotted(&ca.gloss_non_default(flags));
            }
            output
        }
        RootType::Referential => {
            let mut output = version.gloss_non_default(flags);
            if matches!(shortcut_type, ShortcutType::Ca) {
                output.add_dotted(&ca.gloss_non_default(flags));
            }
            output
        }
        RootType::Affixual => {
            let mut output = version.gloss_non_default(flags);
            output.add_dotted(function.gloss_static_non_default(flags));
            output
        }
    };

    let slot_ix = match relation {
        Relation::Nominal { case, .. } => case.gloss_static_non_default(flags),

        // Illocution/Validation must always be shown for disambiguation
        Relation::Verbal { ivl, .. } => ivl.gloss_static(flags),
    };

    let slot_x = match mode {
        Mode::Nominal => ("", "\\UNF"),
        Mode::T1 => ("", ""),
        Mode::T2 => ("", ""),
        Mode::Framed => ("\\FRM", "\\FRM"),
        Mode::Verbal => ("", "\\UNF"),
    };

    let slot_x = if flags.matches(GlossFlags::SHOW_DEFAULTS) {
        slot_x.1
    } else {
        slot_x.0
    };

    let mut gloss = slot_i.to_owned();
    gloss.add_dashed(&slot_ii);
    gloss.add_dashed(&slots_iii_and_iv);
    gloss.add_dashed(&slot_v);
    gloss.add_dashed(&slot_vi);
    gloss.add_dashed(&slot_vii);
    gloss.add_dashed(&slot_viii);
    gloss.add_dashed(&slot_ix);
    gloss += &slot_x;
    gloss
}

impl Gloss for Formative {
    fn gloss(&self, flags: GlossFlags) -> String {
        let (root, stem, version, slot_vii) = match self {
            Self::Normal(
                FormativeCore {
                    root,
                    stem,
                    version,
                    slot_vii_affixes,
                },
                _,
            ) => (
                root.gloss(flags),
                stem.gloss_static(flags),
                *version,
                slot_vii_affixes.gloss(flags),
            ),

            Self::Numeric(
                FormativeCore {
                    root,
                    stem,
                    version,
                    slot_vii_affixes,
                },
                _,
            ) => (
                root.gloss(flags),
                stem.gloss_static(flags),
                *version,
                slot_vii_affixes.gloss(flags),
            ),

            Self::Referential(
                FormativeCore {
                    root,
                    stem: _,
                    version,
                    slot_vii_affixes,
                },
                _,
            ) => (
                root.gloss(flags),
                "",
                *version,
                slot_vii_affixes.gloss(flags),
            ),

            Self::Affixual(
                FormativeCore {
                    root,
                    stem: _,
                    version,
                    slot_vii_affixes,
                },
                _,
            ) => (
                root.gloss(flags),
                "",
                *version,
                slot_vii_affixes.gloss(flags),
            ),
        };

        let (root_type, additions) = match self {
            Formative::Normal(_, additions) => (RootType::Normal, additions.clone().as_general()),
            Formative::Numeric(_, additions) => (RootType::Numeric, additions.clone().as_general()),
            Formative::Referential(_, additions) => {
                (RootType::Referential, additions.clone().as_general())
            }
            Formative::Affixual(_, additions) => {
                (RootType::Affixual, additions.clone().as_general())
            }
        };

        gloss_formative(
            FormativeGlossInput {
                root,
                stem,
                version,
                slot_vii,
                root_type,
                additions: &additions,
            },
            flags,
        )
    }
}

impl Gloss for GeneralFormative {
    fn gloss(&self, flags: GlossFlags) -> String {
        let root = self.0.root.gloss(flags);

        let stem = match self.0.stem {
            Some(value) => value.gloss_static(flags),
            None => "",
        };

        let version = self.0.version;

        let slot_vii = self.0.slot_vii_affixes.gloss(flags);

        let root_type = match self.0.root {
            GeneralFormativeRoot::Normal(_) => RootType::Normal,
            GeneralFormativeRoot::Numeric(_) => RootType::Numeric,
            GeneralFormativeRoot::Referential(_) => RootType::Referential,
            GeneralFormativeRoot::Affixual(_) => RootType::Affixual,
        };

        let additions = &self.1;

        gloss_formative(
            FormativeGlossInput {
                root,
                stem,
                version,
                slot_vii,
                root_type,
                additions,
            },
            flags,
        )
    }
}

impl FromTokenStream for GeneralFormative {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        // This function is scary. Be warned.

        // These are all possible formative structures:
        // 1. ((H)V)CVC(VC...)(VH)(V)
        // 2. ((H)V)CV(CV...)CC(VC...)(VH)(V)
        // 3. HVCV(VC...')(VC...)(VH)(V)
        // 4. ((H)V)CVH(VC...)(V)

        // We'll take care of parsing Vc/Vk first, because it's easy.
        // We'll leave it as a VowelForm though, because we don't want to interpret it into a Vc or
        // Vk form until the end of parsing.

        let vc_or_vk: Option<VowelForm> = stream.next_back();

        // The Cc and Vv slots tell us pretty much everything else about how the word is
        // constructed. So we'll parse those next. How convenient that they're at the beginning of
        // the word.

        #[derive(Clone, Copy, Debug)]
        enum CaShortcutMode {
            W,
            Y,
            None,
        }

        #[derive(Clone, Copy, Debug)]
        enum Concatenation {
            T1,
            T2,
            None,
        }

        // The Cc form gives us the Ca shortcut and concatenation type.

        let (ca_shortcut, concatenation_type, has_cc) = match stream.next() {
            None => (CaShortcutMode::None, Concatenation::None, false),

            Some(HForm { sequence, degree }) => match (sequence, degree) {
                (HFormSequence::SW, HFormDegree::D1) => {
                    (CaShortcutMode::W, Concatenation::None, true)
                }
                (HFormSequence::SY, HFormDegree::D1) => {
                    (CaShortcutMode::Y, Concatenation::None, true)
                }
                (HFormSequence::S0, HFormDegree::D1) => {
                    (CaShortcutMode::None, Concatenation::T1, true)
                }
                (HFormSequence::S0, HFormDegree::D2) => {
                    (CaShortcutMode::W, Concatenation::T1, true)
                }
                (HFormSequence::S0, HFormDegree::D3) => {
                    (CaShortcutMode::W, Concatenation::T2, true)
                }
                (HFormSequence::S0, HFormDegree::D4) => {
                    (CaShortcutMode::Y, Concatenation::T1, true)
                }
                (HFormSequence::S0, HFormDegree::D5) => {
                    (CaShortcutMode::Y, Concatenation::T2, true)
                }
                (HFormSequence::SW, HFormDegree::D2) => {
                    (CaShortcutMode::None, Concatenation::T2, true)
                }
                _ => return Err(ParseError::ExpectedCc),
            },
        };

        #[derive(Clone, Copy, Debug)]
        enum RelationType {
            /// The bool here indicates if the word has cases 37-68.
            T1(bool),

            /// The bool here indicates if the word has cases 37-68.
            T2(bool),

            Nominal,
            Verbal,
            Framed,
        }

        // Now we have enough information to figure out the relation of the word.

        let relation_type = match concatenation_type {
            Concatenation::None => match stream.stress() {
                Some(Stress::Ultimate) => RelationType::Verbal,
                Some(Stress::Antepenultimate) => RelationType::Framed,
                _ => RelationType::Nominal,
            },

            Concatenation::T1 => match stream.stress() {
                Some(Stress::Ultimate) => RelationType::T1(true),
                Some(Stress::Antepenultimate) => {
                    if flags.matches(FromTokenFlags::PERMISSIVE) {
                        RelationType::T1(false)
                    } else {
                        return Err(ParseError::AntepenultimateStress);
                    }
                }
                _ => RelationType::T1(false),
            },

            Concatenation::T2 => match stream.stress() {
                Some(Stress::Ultimate) => RelationType::T2(true),
                Some(Stress::Antepenultimate) => {
                    if flags.matches(FromTokenFlags::PERMISSIVE) {
                        RelationType::T2(false)
                    } else {
                        return Err(ParseError::AntepenultimateStress);
                    }
                }
                _ => RelationType::T2(false),
            },
        };

        // If we parse the Vv form too early, we'll be stuck having to deal with enum variants and
        // stuff. So we'll just capture it for now, detect the word type, and leave it for later.

        let vv: VowelForm = if has_cc {
            stream.next().ok_or(ParseError::ExpectedVv)?
        } else {
            stream.next().unwrap_or_default()
        };

        #[derive(Clone, Copy, Debug)]
        enum WordType {
            NormalOrNumeric {
                version: Version,
                stem: Stem,
                sequence: VowelFormSequence,
            },
            Referential {
                version: Version,
            },
            Affixual {
                version: Version,
                function: Function,
            },
        }

        let word_type = match vv {
            VowelForm {
                has_glottal_stop: _,
                sequence,
                degree: VowelFormDegree::D5,
            } => WordType::Affixual {
                version: match sequence {
                    VowelFormSequence::S1 | VowelFormSequence::S3 => Version::PRC,
                    VowelFormSequence::S2 | VowelFormSequence::S4 => Version::CPT,
                },
                function: match sequence {
                    VowelFormSequence::S1 | VowelFormSequence::S2 => Function::STA,
                    VowelFormSequence::S3 | VowelFormSequence::S4 => Function::DYN,
                },
            },
            VowelForm {
                has_glottal_stop: _,
                sequence,
                degree: VowelFormDegree::D0,
            } => WordType::Referential {
                version: match sequence {
                    VowelFormSequence::S1 => Version::PRC,
                    VowelFormSequence::S2 => Version::CPT,
                    VowelFormSequence::S3 | VowelFormSequence::S4 => {
                        return Err(ParseError::ExpectedVv)
                    }
                },
            },
            VowelForm {
                has_glottal_stop: _,
                sequence,
                degree,
            } => WordType::NormalOrNumeric {
                version: match degree {
                    VowelFormDegree::D1
                    | VowelFormDegree::D3
                    | VowelFormDegree::D7
                    | VowelFormDegree::D9 => Version::PRC,
                    VowelFormDegree::D2
                    | VowelFormDegree::D4
                    | VowelFormDegree::D6
                    | VowelFormDegree::D8 => Version::CPT,
                    _ => unreachable!("we caught degrees 5 and 0 in earlier match arms"),
                },
                stem: match degree {
                    VowelFormDegree::D1 | VowelFormDegree::D2 => Stem::S1,
                    VowelFormDegree::D3 | VowelFormDegree::D4 => Stem::S2,
                    VowelFormDegree::D9 | VowelFormDegree::D8 => Stem::S3,
                    VowelFormDegree::D7 | VowelFormDegree::D6 => Stem::S0,
                    _ => unreachable!("we caught degrees 5 and 0 in earlier match arms"),
                },
                sequence,
            },
        };

        if !matches!(ca_shortcut, CaShortcutMode::None)
            && matches!(word_type, WordType::Affixual { .. })
        {
            return Err(ParseError::AffixualFormativeWithCaShortcut);
        }

        #[derive(Debug)]
        enum Root {
            C(OwnedConsonantForm),
            N(NumeralForm),
        }

        // Again, we'll leave these unparsed for now.

        let root = match stream.next_any() {
            Some(Token::C(value)) => Root::C(value.clone()),
            Some(Token::N(value)) => {
                if !matches!(word_type, WordType::NormalOrNumeric { .. }) {
                    return Err(ParseError::ExpectedNonNumericRoot);
                }

                Root::N(*value)
            }
            _ => return Err(ParseError::ExpectedRoot),
        };

        // Still leaving it unparsed.

        let vr: Option<VowelForm> = match ca_shortcut {
            CaShortcutMode::None => Some(stream.next().ok_or(ParseError::ExpectedVr)?),
            _ => None,
        };

        // Now we've collected Cc, Vv, Cr, and Vr. That's the easy part done, and we can piece
        // together most of the formative now.
        //
        // We still need to get slots V, VI, VII, and VIII, but we'll do that later in case there's
        // an early error in the first four slots.
        //
        // The slots provide these values:
        // Cc = concatenation type, Ca shortcut type
        // Vv =
        //      normal: stem, version, affix shortcut, Ca shortcut index
        //    referent: version
        //    affixual: version, function
        // Cr = root
        // Vr =
        //      normal: specification, function, context
        //    referent: specification, function, context
        //    affixual: degree, context
        // [stress] = relation

        enum Shortcut {
            None(Option<AffixShortcut>),
            Ca(NormalCaShortcut),
        }

        let does_vv_have_glottal_stop = vv.has_glottal_stop;
        let does_vr_have_glottal_stop = vr.map(|x| x.has_glottal_stop).unwrap_or_default();

        let (stem, version, shortcut, specification, function, context, root) = match word_type {
            WordType::NormalOrNumeric {
                version,
                stem,
                sequence,
            } => (
                Some(stem),
                version,
                match ca_shortcut {
                    CaShortcutMode::None => Shortcut::None(match sequence {
                        VowelFormSequence::S1 => Some(AffixShortcut::None),
                        VowelFormSequence::S2 => Some(AffixShortcut::NEG4),
                        VowelFormSequence::S3 => Some(AffixShortcut::DCD4),
                        VowelFormSequence::S4 => Some(AffixShortcut::DCD5),
                    }),
                    CaShortcutMode::W => Shortcut::Ca(match sequence {
                        VowelFormSequence::S1 => NormalCaShortcut::Default,
                        VowelFormSequence::S2 => NormalCaShortcut::G,
                        VowelFormSequence::S3 => NormalCaShortcut::N,
                        VowelFormSequence::S4 => NormalCaShortcut::G_RPV,
                    }),
                    CaShortcutMode::Y => Shortcut::Ca(match sequence {
                        VowelFormSequence::S1 => NormalCaShortcut::PRX,
                        VowelFormSequence::S2 => NormalCaShortcut::RPV,
                        VowelFormSequence::S3 => NormalCaShortcut::A,
                        VowelFormSequence::S4 => NormalCaShortcut::PRX_RPV,
                    }),
                },
                Some(match ca_shortcut {
                    CaShortcutMode::None => match vr.ok_or(ParseError::ExpectedVr)?.degree {
                        VowelFormDegree::D0 => unreachable!("D0 isn't a normal Vv form"),
                        VowelFormDegree::D5 => unreachable!("D5 isn't a normal Vv form"),
                        VowelFormDegree::D1 | VowelFormDegree::D9 => Specification::BSC,
                        VowelFormDegree::D2 | VowelFormDegree::D8 => Specification::CTE,
                        VowelFormDegree::D3 | VowelFormDegree::D7 => Specification::CSV,
                        VowelFormDegree::D4 | VowelFormDegree::D6 => Specification::OBJ,
                    },
                    _ => Specification::BSC,
                }),
                match ca_shortcut {
                    CaShortcutMode::None => match vr.ok_or(ParseError::ExpectedVr)?.degree {
                        VowelFormDegree::D0 => unreachable!("D0 isn't a normal Vv form"),
                        VowelFormDegree::D5 => unreachable!("D5 isn't a normal Vv form"),
                        VowelFormDegree::D1
                        | VowelFormDegree::D2
                        | VowelFormDegree::D3
                        | VowelFormDegree::D4 => Function::STA,
                        VowelFormDegree::D6
                        | VowelFormDegree::D7
                        | VowelFormDegree::D8
                        | VowelFormDegree::D9 => Function::DYN,
                    },
                    _ => Function::STA,
                },
                match ca_shortcut {
                    CaShortcutMode::None => match vr.ok_or(ParseError::ExpectedVr)?.sequence {
                        VowelFormSequence::S1 => Context::EXS,
                        VowelFormSequence::S2 => Context::FNC,
                        VowelFormSequence::S3 => Context::RPS,
                        VowelFormSequence::S4 => Context::AMG,
                    },
                    _ => Context::EXS,
                },
                match root {
                    Root::C(cr) => GeneralFormativeRoot::Normal(NormalFormativeRoot { cr: cr.0 }),
                    Root::N(n) => GeneralFormativeRoot::Numeric(NumericFormativeRoot {
                        integer_part: n.integer_part,
                    }),
                },
            ),

            WordType::Referential { version } => (
                None,
                version,
                Shortcut::None(None),
                Some(match ca_shortcut {
                    CaShortcutMode::None => match vr.ok_or(ParseError::ExpectedVr)?.degree {
                        VowelFormDegree::D0 => unreachable!("D0 isn't a normal Vv form"),
                        VowelFormDegree::D5 => unreachable!("D5 isn't a normal Vv form"),
                        VowelFormDegree::D1 | VowelFormDegree::D9 => Specification::BSC,
                        VowelFormDegree::D2 | VowelFormDegree::D8 => Specification::CTE,
                        VowelFormDegree::D3 | VowelFormDegree::D7 => Specification::CSV,
                        VowelFormDegree::D4 | VowelFormDegree::D6 => Specification::OBJ,
                    },
                    _ => Specification::BSC,
                }),
                match ca_shortcut {
                    CaShortcutMode::None => match vr.ok_or(ParseError::ExpectedVr)?.degree {
                        VowelFormDegree::D0 => unreachable!("D0 isn't a normal Vv form"),
                        VowelFormDegree::D5 => unreachable!("D5 isn't a normal Vv form"),
                        VowelFormDegree::D1
                        | VowelFormDegree::D2
                        | VowelFormDegree::D3
                        | VowelFormDegree::D4 => Function::STA,
                        VowelFormDegree::D6
                        | VowelFormDegree::D7
                        | VowelFormDegree::D8
                        | VowelFormDegree::D9 => Function::DYN,
                    },
                    _ => Function::STA,
                },
                match ca_shortcut {
                    CaShortcutMode::None => match vr.ok_or(ParseError::ExpectedVr)?.sequence {
                        VowelFormSequence::S1 => Context::EXS,
                        VowelFormSequence::S2 => Context::FNC,
                        VowelFormSequence::S3 => Context::RPS,
                        VowelFormSequence::S4 => Context::AMG,
                    },
                    _ => Context::EXS,
                },
                match root {
                    Root::C(cr) => GeneralFormativeRoot::Referential(cr.parse()?),
                    Root::N(_) => return Err(ParseError::ExpectedReferentialRoot),
                },
            ),

            WordType::Affixual { version, function } => (
                None,
                version,
                Shortcut::None(None),
                None,
                function,
                match ca_shortcut {
                    CaShortcutMode::None => match vr.ok_or(ParseError::ExpectedVr)?.sequence {
                        VowelFormSequence::S1 => Context::EXS,
                        VowelFormSequence::S2 => Context::FNC,
                        VowelFormSequence::S3 => Context::RPS,
                        VowelFormSequence::S4 => Context::AMG,
                    },
                    _ => return Err(ParseError::AffixualFormativeWithCaShortcut),
                },
                match root {
                    Root::C(cr) => GeneralFormativeRoot::Affixual(AffixualFormativeRoot {
                        cs: cr.0,
                        degree: vr.ok_or(ParseError::ExpectedVr)?.degree.into(),
                    }),
                    Root::N(_) => return Err(ParseError::ExpectedReferentialRoot),
                },
            ),
        };

        // We've officially completed the easy part of parsing.
        //
        // There are now quite a few options left for how the token stream can be parsed.
        //
        // For formatives with Ca shortcuts, there is only one option:
        // 1. (VC...')(VC...)(VH)     (slot V affixes, then slot VII affixes, then VnCn)
        //
        // For other formatives, there are several choices:
        // 1. C(VC...)(VH)            (an ungeminated Ca, then slot VII affixes, then VnCn)
        // 2. (CV...)CC(VC...)(VH)    (slot V affixes, then a geminated Ca, then VnCn)
        // 3. H(VC...)                (a Cn shortcut, then slot VII affixes)
        //
        // Time to get to work!

        enum VnCn {
            VnCn(Vn, ArbitraryMoodOrCaseScope),
            JustCn(ArbitraryMoodOrCaseScope),
            Nothing,
        }

        let (vncn, does_vn_have_glottal_stop) = match stream.next_back::<HForm>() {
            Some(cn) => match stream.next_back_any() {
                Some(Token::V(vn)) => {
                    let is_aspect = matches!(cn.sequence, HFormSequence::SW | HFormSequence::SY);

                    let cn = match cn.degree {
                        HFormDegree::D1 => ArbitraryMoodOrCaseScope::FAC_CCN,
                        HFormDegree::D2 => ArbitraryMoodOrCaseScope::SUB_CCA,
                        HFormDegree::D3 => ArbitraryMoodOrCaseScope::ASM_CCS,
                        HFormDegree::D4 => ArbitraryMoodOrCaseScope::SPC_CCQ,
                        HFormDegree::D5 => ArbitraryMoodOrCaseScope::COU_CCP,
                        HFormDegree::D6 => ArbitraryMoodOrCaseScope::HYP_CCV,
                    };

                    (
                        VnCn::VnCn(
                            Vn::from_vowel_form(*vn, is_aspect).ok_or(ParseError::ExpectedVn)?,
                            cn,
                        ),
                        vn.has_glottal_stop,
                    )
                }
                None => (
                    VnCn::JustCn(match cn.sequence {
                        HFormSequence::S0 => match cn.degree {
                            HFormDegree::D1 => return Err(ParseError::DefaultCnShortcut),
                            HFormDegree::D2 => ArbitraryMoodOrCaseScope::SUB_CCA,
                            HFormDegree::D3 => ArbitraryMoodOrCaseScope::ASM_CCS,
                            HFormDegree::D4 => ArbitraryMoodOrCaseScope::SPC_CCQ,
                            HFormDegree::D5 => ArbitraryMoodOrCaseScope::COU_CCP,
                            HFormDegree::D6 => ArbitraryMoodOrCaseScope::HYP_CCV,
                        },
                        _ => return Err(ParseError::AspectualCnShortcut),
                    }),
                    false,
                ),
                _ => return Err(ParseError::ExpectedVn),
            },
            None => (VnCn::Nothing, false),
        };

        let mut does_vx_have_glottal_stop = false;

        enum MiddleSegments<AffixListType> {
            Normal {
                slot_v_affixes: AffixListType,
                ca: Ca,
                slot_vii_affixes: AffixListType,
                vncn: Option<(Vn, ArbitraryMoodOrCaseScope)>,
                affix_shortcut: Option<AffixShortcut>,
            },

            Ca {
                ca: NormalCaShortcut,
                slot_v_affixes: AffixListType,
                slot_vii_affixes: AffixListType,
                vncn: Option<(Vn, ArbitraryMoodOrCaseScope)>,
            },

            Cn {
                cn: ArbitraryMoodOrCaseScope,
                slot_vii_affixes: AffixListType,
                affix_shortcut: Option<AffixShortcut>,
            },
        }

        type UnparsedMiddleSegments = MiddleSegments<Vec<(VowelForm, OwnedConsonantForm)>>;

        let middle: UnparsedMiddleSegments = match shortcut {
            Shortcut::Ca(ca) => {
                let vncn = match vncn {
                    VnCn::Nothing => None,
                    VnCn::JustCn(_) => return Err(ParseError::ExpectedVn),
                    VnCn::VnCn(vn, cn) => Some((vn, cn)),
                };

                let mut slot_v_affixes = Vec::new();
                let mut slot_vii_affixes = Vec::new();

                loop {
                    match stream.next_any() {
                        None => break,
                        Some(Token::V(vx)) => {
                            let vx = *vx;
                            match stream.next_any() {
                                Some(Token::C(cs)) => {
                                    slot_vii_affixes.push((vx, cs.clone()));

                                    if cs.is_geminate() {
                                        return Err(ParseError::GeminatedCs);
                                    }

                                    if vx.has_glottal_stop {
                                        if slot_v_affixes.is_empty() {
                                            slot_v_affixes = slot_vii_affixes;
                                            slot_vii_affixes = Vec::new();
                                        } else {
                                            return Err(ParseError::MultipleEndOfSlotVMarkers);
                                        }
                                    }
                                }
                                _ => return Err(ParseError::ExpectedCs),
                            }
                        }
                        _ => return Err(ParseError::ExpectedVx),
                    }
                }

                MiddleSegments::Ca {
                    ca,
                    slot_v_affixes,
                    slot_vii_affixes,
                    vncn,
                }
            }

            Shortcut::None(affix_shortcut) => {
                if let VnCn::JustCn(cn) = vncn {
                    MiddleSegments::Cn {
                        cn,
                        slot_vii_affixes: Vec::new(),
                        affix_shortcut,
                    }
                } else {
                    // Quick reminder of possible forms at this point:
                    // 1. C(VC...)            (an ungeminated Ca, then slot VII affixes)
                    // 2. (CV...)CC(VC...)    (slot V affixes, then a geminated Ca)
                    // 3. H(VC...)            (a Cn shortcut, then slot VII affixes)

                    if let Some(HForm { sequence, degree }) = stream.next() {
                        if sequence != HFormSequence::S0 {
                            return Err(ParseError::AspectualCnShortcut);
                        }

                        let cn = match degree {
                            HFormDegree::D1 => return Err(ParseError::DefaultCnShortcut),
                            HFormDegree::D2 => ArbitraryMoodOrCaseScope::SUB_CCA,
                            HFormDegree::D3 => ArbitraryMoodOrCaseScope::ASM_CCS,
                            HFormDegree::D4 => ArbitraryMoodOrCaseScope::SPC_CCQ,
                            HFormDegree::D5 => ArbitraryMoodOrCaseScope::COU_CCP,
                            HFormDegree::D6 => ArbitraryMoodOrCaseScope::HYP_CCV,
                        };

                        let mut slot_vii_affixes = Vec::new();

                        loop {
                            match stream.next_any() {
                                None => break,
                                Some(Token::V(vx)) => {
                                    let vx = *vx;
                                    match stream.next_any() {
                                        Some(Token::C(cs)) => {
                                            slot_vii_affixes.push((vx, cs.clone()));

                                            if cs.is_geminate() {
                                                return Err(ParseError::GeminatedCs);
                                            }

                                            if vx.has_glottal_stop {
                                                if does_vx_have_glottal_stop {
                                                    return Err(ParseError::DoublyGlottalizedVx);
                                                } else {
                                                    does_vx_have_glottal_stop = true;
                                                }
                                            }
                                        }
                                        _ => return Err(ParseError::ExpectedCs),
                                    }
                                }
                                _ => return Err(ParseError::ExpectedVx),
                            }
                        }

                        MiddleSegments::Cn {
                            cn,
                            slot_vii_affixes,
                            affix_shortcut,
                        }
                    } else {
                        if stream
                            .remaining_tokens()
                            .iter()
                            .any(|x| matches!(x, Token::C(cs) if cs.is_geminate()))
                        {
                            let mut slot_v_affixes = Vec::new();

                            'outer: loop {
                                match stream.next_any() {
                                    Some(Token::C(cs)) => {
                                        let cs = cs.clone();

                                        if cs.is_geminate() {
                                            let ca = Ca::from_geminated_string(&cs)
                                                .ok_or(ParseError::ExpectedCa)?;

                                            let mut slot_vii_affixes = Vec::new();

                                            // Do we really have 11 levels of nesting? So proud /hj

                                            loop {
                                                match stream.next_any() {
                                                    None => {
                                                        break 'outer MiddleSegments::Normal {
                                                            slot_v_affixes,
                                                            ca,
                                                            slot_vii_affixes,
                                                            vncn: match vncn {
                                                                VnCn::VnCn(vn, cn) => {
                                                                    Some((vn, cn))
                                                                }
                                                                VnCn::JustCn(_) => unreachable!(
                                                                    // 68 indent characters?
                                                                    // New world record!
                                                                    "we caught this already"
                                                                ),
                                                                VnCn::Nothing => None,
                                                            },
                                                            affix_shortcut,
                                                        };
                                                    }

                                                    Some(Token::V(vx)) => {
                                                        let vx = *vx;

                                                        let cs: OwnedConsonantForm = stream
                                                            .next::<OwnedConsonantForm>()
                                                            .ok_or(ParseError::ExpectedCs)?
                                                            .clone();

                                                        if cs.is_geminate() {
                                                            return Err(ParseError::GeminatedCs);
                                                        }

                                                        if vx.has_glottal_stop {
                                                            if does_vx_have_glottal_stop {
                                                                return Err(
                                                                    ParseError::DoublyGlottalizedVx,
                                                                );
                                                            } else {
                                                                does_vx_have_glottal_stop = true;
                                                            }
                                                        }

                                                        slot_vii_affixes.push((vx, cs));
                                                    }

                                                    _ => return Err(ParseError::ExpectedVx),
                                                }
                                            }
                                        } else {
                                            let vx: VowelForm =
                                                stream.next().ok_or(ParseError::ExpectedVx)?;

                                            if vx.has_glottal_stop {
                                                if does_vx_have_glottal_stop {
                                                    return Err(ParseError::DoublyGlottalizedVx);
                                                } else {
                                                    does_vx_have_glottal_stop = true;
                                                }
                                            }

                                            slot_v_affixes.push((vx, cs));
                                        }
                                    }
                                    _ => return Err(ParseError::ExpectedCs),
                                }
                            }
                        } else {
                            let ca: OwnedConsonantForm =
                                stream.next().ok_or(ParseError::ExpectedCa)?;

                            let ca =
                                Ca::from_ungeminated_string(&ca).ok_or(ParseError::ExpectedCa)?;

                            let mut slot_vii_affixes = Vec::new();

                            loop {
                                match stream.next_any() {
                                    None => break,
                                    Some(Token::V(vx)) => {
                                        let vx = *vx;
                                        match stream.next_any() {
                                            Some(Token::C(cs)) => {
                                                slot_vii_affixes.push((vx, cs.clone()));

                                                if cs.is_geminate() {
                                                    return Err(ParseError::GeminatedCs);
                                                }

                                                if vx.has_glottal_stop {
                                                    if does_vx_have_glottal_stop {
                                                        return Err(
                                                            ParseError::DoublyGlottalizedVx,
                                                        );
                                                    } else {
                                                        does_vx_have_glottal_stop = true;
                                                    }
                                                }
                                            }
                                            _ => return Err(ParseError::ExpectedCs),
                                        }
                                    }
                                    _ => return Err(ParseError::ExpectedVx),
                                }
                            }

                            MiddleSegments::Normal {
                                slot_v_affixes: Vec::new(),
                                ca,
                                slot_vii_affixes,
                                vncn: match vncn {
                                    VnCn::VnCn(vn, cn) => Some((vn, cn)),
                                    VnCn::JustCn(_) => unreachable!("we caught this already"),
                                    VnCn::Nothing => None,
                                },
                                affix_shortcut,
                            }
                        }
                    }
                }
            }
        };

        // Time to parse affixes.

        type ParsedMiddleSegments = MiddleSegments<AffixList>;

        let middle_parsed: ParsedMiddleSegments = match middle {
            MiddleSegments::Normal {
                slot_v_affixes,
                ca,
                slot_vii_affixes,
                vncn,
                affix_shortcut,
            } => MiddleSegments::Normal {
                slot_v_affixes: {
                    let slot_v_affixes = AffixList::from_vxcs_slice(&slot_v_affixes)?;

                    if !flags.matches(FromTokenFlags::PERMISSIVE) {
                        if does_vv_have_glottal_stop {
                            if slot_v_affixes.len() <= 1 {
                                return Err(ParseError::TooFewSlotVAffixes);
                            }
                        } else {
                            if slot_v_affixes.len() > 1 {
                                return Err(ParseError::TooManySlotVAffixes);
                            }
                        }
                    }

                    slot_v_affixes
                },
                ca,
                slot_vii_affixes: AffixList::from_vxcs_slice(&slot_vii_affixes)?,
                vncn,
                affix_shortcut,
            },

            MiddleSegments::Ca {
                ca,
                slot_v_affixes,
                slot_vii_affixes,
                vncn,
            } => MiddleSegments::Ca {
                ca,
                slot_v_affixes: {
                    let slot_v_affixes = AffixList::from_vxcs_slice(&slot_v_affixes)?;

                    if !flags.matches(FromTokenFlags::PERMISSIVE) {
                        if does_vv_have_glottal_stop {
                            if slot_v_affixes.len() <= 1 {
                                return Err(ParseError::TooFewSlotVAffixes);
                            }
                        } else {
                            if slot_v_affixes.len() > 1 {
                                return Err(ParseError::TooManySlotVAffixes);
                            }
                        }
                    }

                    slot_v_affixes
                },
                slot_vii_affixes: AffixList::from_vxcs_slice(&slot_vii_affixes)?,
                vncn,
            },

            MiddleSegments::Cn {
                cn,
                slot_vii_affixes,
                affix_shortcut,
            } => MiddleSegments::Cn {
                cn,
                slot_vii_affixes: AffixList::from_vxcs_slice(&slot_vii_affixes)?,
                affix_shortcut,
            },
        };

        // We're free! Now we just have to put everything together

        let (additions, slot_vii_affixes) = match middle_parsed {
            MiddleSegments::Normal {
                slot_v_affixes,
                ca,
                slot_vii_affixes,
                vncn,
                affix_shortcut,
            } => (
                GeneralFormativeAdditions::Normal(GeneralNonShortcutAdditions {
                    relation: match relation_type {
                        RelationType::Verbal => Relation::Verbal {
                            mood: match vncn {
                                Some(vncn) => vncn.1.as_specific(),
                                _ => Mood::FAC,
                            },
                            ivl: {
                                let mut vk = vc_or_vk.unwrap_or_default();
                                vk.merge_vcvk_glottal_stop(does_vr_have_glottal_stop)?;
                                vk.merge_vcvk_glottal_stop(does_vx_have_glottal_stop)?;
                                vk.merge_vcvk_glottal_stop(does_vn_have_glottal_stop)?;
                                IllocutionOrValidation::from_vk(vk)?
                            },
                        },
                        _ => Relation::Nominal {
                            mode: match relation_type {
                                RelationType::Nominal => NominalMode::NOM,
                                RelationType::T1(_) => NominalMode::T1,
                                RelationType::T2(_) => NominalMode::T2,
                                RelationType::Framed => NominalMode::FRM,
                                RelationType::Verbal => unreachable!(),
                            },
                            case_scope: vncn.map(|x| x.1.as_specific()).unwrap_or_default(),
                            case: match relation_type {
                                RelationType::Nominal | RelationType::Framed => {
                                    let mut vc = vc_or_vk.unwrap_or_default();
                                    vc.merge_vcvk_glottal_stop(does_vr_have_glottal_stop)?;
                                    vc.merge_vcvk_glottal_stop(does_vx_have_glottal_stop)?;
                                    vc.merge_vcvk_glottal_stop(does_vn_have_glottal_stop)?;
                                    Case::from_vc(vc)?
                                }
                                RelationType::T1(is_high) | RelationType::T2(is_high) => {
                                    let mut vc = vc_or_vk.unwrap_or_default();
                                    vc.merge_vcvk_glottal_stop(does_vr_have_glottal_stop)?;
                                    vc.merge_vcvk_glottal_stop(does_vx_have_glottal_stop)?;
                                    vc.merge_vcvk_glottal_stop(does_vn_have_glottal_stop)?;
                                    if vc.has_glottal_stop {
                                        return Err(ParseError::GlottalizedVc);
                                    }
                                    vc.has_glottal_stop = is_high;
                                    Case::from_vc(vc)?
                                }
                                RelationType::Verbal => unreachable!(),
                            },
                        },
                    },
                    affix_shortcut,
                    function,
                    specification,
                    context,
                    slot_v_affixes,
                    ca,
                    vn: match vncn {
                        Some(vncn) => vncn.0,
                        _ => Vn::Valence(Valence::MNO),
                    },
                }),
                slot_vii_affixes,
            ),

            MiddleSegments::Cn {
                cn,
                slot_vii_affixes,
                affix_shortcut,
            } => (
                GeneralFormativeAdditions::CnShortcut(GeneralCnShortcutAdditions {
                    relation: match relation_type {
                        RelationType::Verbal => Relation::Verbal {
                            mood: cn
                                .try_as_specific()
                                .ok_or(ParseError::ExpectedNonDefaultCn)?,
                            ivl: {
                                let mut vk = vc_or_vk.unwrap_or_default();
                                vk.merge_vcvk_glottal_stop(does_vr_have_glottal_stop)?;
                                vk.merge_vcvk_glottal_stop(does_vx_have_glottal_stop)?;
                                vk.merge_vcvk_glottal_stop(does_vn_have_glottal_stop)?;
                                IllocutionOrValidation::from_vk(vk)?
                            },
                        },
                        _ => Relation::Nominal {
                            mode: match relation_type {
                                RelationType::Nominal => NominalMode::NOM,
                                RelationType::T1(_) => NominalMode::T1,
                                RelationType::T2(_) => NominalMode::T2,
                                RelationType::Framed => NominalMode::FRM,
                                RelationType::Verbal => unreachable!(),
                            },
                            case_scope: cn
                                .try_as_specific()
                                .ok_or(ParseError::ExpectedNonDefaultCn)?,
                            case: match relation_type {
                                RelationType::Nominal | RelationType::Framed => {
                                    let mut vc = vc_or_vk.unwrap_or_default();
                                    vc.merge_vcvk_glottal_stop(does_vr_have_glottal_stop)?;
                                    vc.merge_vcvk_glottal_stop(does_vx_have_glottal_stop)?;
                                    vc.merge_vcvk_glottal_stop(does_vn_have_glottal_stop)?;
                                    Case::from_vc(vc)?
                                }
                                RelationType::T1(is_high) | RelationType::T2(is_high) => {
                                    let mut vc = vc_or_vk.unwrap_or_default();
                                    vc.merge_vcvk_glottal_stop(does_vr_have_glottal_stop)?;
                                    vc.merge_vcvk_glottal_stop(does_vx_have_glottal_stop)?;
                                    vc.merge_vcvk_glottal_stop(does_vn_have_glottal_stop)?;
                                    if vc.has_glottal_stop {
                                        return Err(ParseError::GlottalizedVc);
                                    }
                                    vc.has_glottal_stop = is_high;
                                    Case::from_vc(vc)?
                                }
                                RelationType::Verbal => unreachable!(),
                            },
                        },
                    },
                    affix_shortcut,
                    function,
                    specification,
                    context,
                }),
                slot_vii_affixes,
            ),

            MiddleSegments::Ca {
                ca,
                slot_v_affixes,
                slot_vii_affixes,
                vncn,
            } => (
                GeneralFormativeAdditions::CaShortcut(NormalCaShortcutAdditions {
                    relation: match relation_type {
                        RelationType::Verbal => Relation::Verbal {
                            mood: match vncn {
                                Some(vncn) => vncn.1.as_specific(),
                                _ => Mood::FAC,
                            },
                            ivl: {
                                let mut vk = vc_or_vk.unwrap_or_default();
                                vk.merge_vcvk_glottal_stop(does_vr_have_glottal_stop)?;
                                vk.merge_vcvk_glottal_stop(does_vx_have_glottal_stop)?;
                                vk.merge_vcvk_glottal_stop(does_vn_have_glottal_stop)?;
                                IllocutionOrValidation::from_vk(vk)?
                            },
                        },
                        _ => Relation::Nominal {
                            mode: match relation_type {
                                RelationType::Nominal => NominalMode::NOM,
                                RelationType::T1(_) => NominalMode::T1,
                                RelationType::T2(_) => NominalMode::T2,
                                RelationType::Framed => NominalMode::FRM,
                                RelationType::Verbal => unreachable!(),
                            },
                            case_scope: vncn.map(|x| x.1.as_specific()).unwrap_or_default(),
                            case: match relation_type {
                                RelationType::Nominal | RelationType::Framed => {
                                    let mut vc = vc_or_vk.unwrap_or_default();
                                    vc.merge_vcvk_glottal_stop(does_vr_have_glottal_stop)?;
                                    vc.merge_vcvk_glottal_stop(does_vx_have_glottal_stop)?;
                                    vc.merge_vcvk_glottal_stop(does_vn_have_glottal_stop)?;
                                    Case::from_vc(vc)?
                                }
                                RelationType::T1(is_high) | RelationType::T2(is_high) => {
                                    let mut vc = vc_or_vk.unwrap_or_default();
                                    vc.merge_vcvk_glottal_stop(does_vr_have_glottal_stop)?;
                                    vc.merge_vcvk_glottal_stop(does_vx_have_glottal_stop)?;
                                    vc.merge_vcvk_glottal_stop(does_vn_have_glottal_stop)?;
                                    if vc.has_glottal_stop {
                                        return Err(ParseError::GlottalizedVc);
                                    }
                                    vc.has_glottal_stop = is_high;
                                    Case::from_vc(vc)?
                                }
                                RelationType::Verbal => unreachable!(),
                            },
                        },
                    },
                    slot_v_affixes,
                    ca,
                    vn: vncn.map(|x| x.0).unwrap_or_default(),
                }),
                slot_vii_affixes,
            ),
        };

        Ok(GeneralFormative(
            GeneralFormativeCore {
                root,
                slot_vii_affixes,
                stem,
                version,
            },
            additions,
        ))
    }
}

impl FromTokenStream for Formative {
    fn parse_volatile(stream: &mut TokenStream, flags: FromTokenFlags) -> Result<Self, ParseError> {
        let general: GeneralFormative = stream.parse(flags)?;

        general
            .try_as_specific()
            .ok_or(ParseError::InvalidFormative)
    }
}
