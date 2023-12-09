//! Defines complexes (groups of core categories) and their glossing methods.

use super::{
    Affiliation, Aspect, CaseScope, Category, Configuration, Effect, Essence, Extension, Level,
    Mood, Perspective, Phase, Plexity, ReferentEffect, ReferentTarget, ReferentialAffixPerspective,
    Separability, Similarity, Valence,
};
use crate::{
    gloss::{Gloss, GlossFlags, GlossHelpers, GlossStatic},
    referent,
    romanize::stream::ParseError,
};
use std::str::FromStr;
use vec1::Vec1;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// A pair containing a Similarity and a Separability.
pub struct SimilarityAndSeparability {
    /// The Similarity component of self.
    pub similarity: Similarity,

    /// The Separability component of self.
    pub separability: Separability,
}

impl SimilarityAndSeparability {
    /// Constructs a new [`SimilarityAndSeparability`] from a [`Similarity`] and [`Separability`].
    pub fn new(similarity: Similarity, separability: Separability) -> Self {
        Self {
            similarity,
            separability,
        }
    }
}

impl GlossStatic for SimilarityAndSeparability {
    fn gloss_static(&self, flags: GlossFlags) -> &'static str {
        if flags.matches(GlossFlags::LONG) {
            match self.similarity {
                Similarity::S => match self.separability {
                    Separability::S => "similar.separate",
                    Separability::C => "similar.connected",
                    Separability::F => "similar.fused",
                },
                Similarity::D => match self.separability {
                    Separability::S => "dissimilar.separate",
                    Separability::C => "dissimilar.connected",
                    Separability::F => "dissimilar.fused",
                },
                Similarity::F => match self.separability {
                    Separability::S => "fused.separate",
                    Separability::C => "fused.connected",
                    Separability::F => "fused.fused",
                },
            }
        } else {
            match self.similarity {
                Similarity::S => match self.separability {
                    Separability::S => "SS",
                    Separability::C => "SC",
                    Separability::F => "SF",
                },
                Similarity::D => match self.separability {
                    Separability::S => "DS",
                    Separability::C => "DC",
                    Separability::F => "DF",
                },
                Similarity::F => match self.separability {
                    Separability::S => "FS",
                    Separability::C => "FC",
                    Separability::F => "FF",
                },
            }
        }
    }
}

impl Category for SimilarityAndSeparability {
    fn abbr(self) -> &'static str {
        match self.similarity {
            Similarity::S => match self.separability {
                Separability::S => "SS",
                Separability::C => "SC",
                Separability::F => "SF",
            },
            Similarity::D => match self.separability {
                Separability::S => "DS",
                Separability::C => "DC",
                Separability::F => "DF",
            },
            Similarity::F => match self.separability {
                Separability::S => "FS",
                Separability::C => "FC",
                Separability::F => "FF",
            },
        }
    }

    fn name(self) -> &'static str {
        match self.similarity {
            Similarity::S => match self.separability {
                Separability::S => "Similar Separate",
                Separability::C => "Similar Connected",
                Separability::F => "Similar Fused",
            },
            Similarity::D => match self.separability {
                Separability::S => "Dissimilar Separate",
                Separability::C => "Dissimilar Connected",
                Separability::F => "Dissimilar Fused",
            },
            Similarity::F => match self.separability {
                Separability::S => "Fused Separate",
                Separability::C => "Fused Connected",
                Separability::F => "Fused Fused",
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// A destructed version of [`Configuration`].
pub struct DestructuredConfiguration {
    /// The Plexity of this [`DestructuredConfiguration`].
    pub plexity: Plexity,

    /// The [`Similarity`] and [`Separability`] of this [`DestructuredConfiguration`], or `None` if
    /// it is pure UPX or DPX.
    pub similarity_and_separability: Option<SimilarityAndSeparability>,
}

impl DestructuredConfiguration {
    /// Constructs a new [`DestructuredConfiguration`] from a [`Plexity`] and
    /// [`Option<SimilarityAndSeparability>`].
    pub fn new(
        plexity: Plexity,
        similarity_and_separability: Option<SimilarityAndSeparability>,
    ) -> Self {
        Self {
            plexity,
            similarity_and_separability,
        }
    }
}

impl GlossStatic for DestructuredConfiguration {
    fn gloss_static(&self, flags: GlossFlags) -> &'static str {
        Configuration::from(*self).gloss_static(flags)
    }
}

impl Category for DestructuredConfiguration {
    fn abbr(self) -> &'static str {
        Configuration::from(self).abbr()
    }

    fn name(self) -> &'static str {
        Configuration::from(self).name()
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// A Ca value.
pub struct Ca {
    /// The [`Affiliation`] of this Ca.
    pub affiliation: Affiliation,

    /// The [`Configuration`] of this Ca.
    pub configuration: Configuration,

    /// The [`Extension`] of this Ca.
    pub extension: Extension,

    /// The [`Perspective`] of this Ca.
    pub perspective: Perspective,

    /// The [`Essence`] of this Ca.
    pub essence: Essence,
}

impl Gloss for Ca {
    fn gloss(&self, flags: GlossFlags) -> String {
        if flags.matches(GlossFlags::SHOW_DEFAULTS) {
            let mut output = self.affiliation.gloss(flags);
            output += ".";
            output += self.configuration.gloss_static(flags);
            output += ".";
            output += self.extension.gloss_static(flags);
            output += ".";
            output += self.perspective.gloss_static(flags);
            output += ".";
            output += self.essence.gloss_static(flags);
            output
        } else {
            let mut output = String::new();

            for item in [
                self.affiliation.gloss_static_non_default(flags),
                self.configuration.gloss_static_non_default(flags),
                self.extension.gloss_static_non_default(flags),
                self.perspective.gloss_static_non_default(flags),
                self.essence.gloss_static_non_default(flags),
            ] {
                output.add_dotted(item);
            }

            output
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// A non-aspectual Vn value.
pub enum NonAspectualVn {
    /// A variant containing a [`Valence`].
    Valence(Valence),

    /// A variant containing a [`Phase`].
    Phase(Phase),

    /// A variant containing an [`Effect`].
    Effect(Effect),

    /// A variant containing a [`Level`].
    Level(Level),
}

impl Default for NonAspectualVn {
    fn default() -> Self {
        Self::Valence(Valence::MNO)
    }
}

impl GlossStatic for NonAspectualVn {
    fn gloss_static(&self, flags: GlossFlags) -> &'static str {
        match self {
            Self::Valence(value) => value.gloss_static(flags),
            Self::Phase(value) => value.gloss_static(flags),
            Self::Effect(value) => value.gloss_static(flags),
            Self::Level(value) => value.gloss_static(flags),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// A Vn value.
pub enum Vn {
    /// A variant containing a [`Valence`].
    Valence(Valence),

    /// A variant containing a [`Phase`].
    Phase(Phase),

    /// A variant containing an [`Effect`].
    Effect(Effect),

    /// A variant containing a [`Level`].
    Level(Level),

    /// A variant containing an [`Aspect`].
    Aspect(Aspect),
}

impl Vn {
    /// Converts this [`Vn`] into a [`NonAspectualVn`], returning an [`Err<Aspect>`] if it fails.
    pub const fn as_non_aspectual_vn(self) -> Result<NonAspectualVn, Aspect> {
        match self {
            Self::Valence(value) => Ok(NonAspectualVn::Valence(value)),
            Self::Phase(value) => Ok(NonAspectualVn::Phase(value)),
            Self::Effect(value) => Ok(NonAspectualVn::Effect(value)),
            Self::Level(value) => Ok(NonAspectualVn::Level(value)),
            Self::Aspect(value) => Err(value),
        }
    }
}

impl Default for Vn {
    fn default() -> Self {
        Self::Valence(Valence::MNO)
    }
}

impl GlossStatic for Vn {
    fn gloss_static(&self, flags: GlossFlags) -> &'static str {
        match self {
            Self::Valence(value) => value.gloss_static(flags),
            Self::Phase(value) => value.gloss_static(flags),
            Self::Effect(value) => value.gloss_static(flags),
            Self::Level(value) => value.gloss_static(flags),
            Self::Aspect(value) => value.gloss_static(flags),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// A mood or case-scope value. This contains _either_ a mood _or_ a case-scope, not an arbitrary
/// thing which could be either depending on its corresponding formative's type.
///
/// If you need something which could become a [`Mood`] or a [`CaseScope`] depending on the relation
/// of a nearby formative, consider
/// [`ArbitraryMoodOrCaseScope`][crate::category::ArbitraryMoodOrCaseScope] instead.
pub enum MoodOrCaseScope {
    /// A mood.
    Mood(Mood),

    /// A case-scope.
    CaseScope(CaseScope),
}

impl GlossStatic for MoodOrCaseScope {
    fn gloss_static(&self, flags: GlossFlags) -> &'static str {
        match self {
            Self::Mood(mood) => mood.gloss_static(flags),
            Self::CaseScope(case_scope) => case_scope.gloss_static(flags),
        }
    }
}

impl MoodOrCaseScope {
    /// Glosses this value with a set of flags, returning the gloss as a `&'static str`. If `flags`
    /// does not include `GlossFlags::SHOW_DEFAULTS` and `self` is FAC or CCN, the empty
    /// string is returned.
    pub fn gloss_static_non_fac_ccn(&self, flags: GlossFlags) -> &'static str {
        if flags.matches(GlossFlags::SHOW_DEFAULTS)
            || !matches!(
                self,
                Self::Mood(Mood::FAC) | Self::CaseScope(CaseScope::CCN)
            )
        {
            self.gloss_static(flags)
        } else {
            ""
        }
    }

    /// Glosses this value with a set of flags, returning the gloss as an allocated [`String`]. If
    /// `flags` does not include `GlossFlags::SHOW_DEFAULTS` and `self` is FAC or CCN, the empty
    /// string is returned.
    pub fn gloss_non_fac_ccn(&self, flags: GlossFlags) -> String {
        self.gloss_static_non_fac_ccn(flags).to_owned()
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// A referent with a target and effect.
pub struct Referent {
    /// The target of this referent.
    pub target: ReferentTarget,

    /// The effect of this referent.
    pub effect: ReferentEffect,
}

impl Referent {
    /// Gets the string representing this referent in a referential affix.
    pub const fn to_referential_affix_str(self) -> &'static str {
        match self {
            referent!(M1.NEU) => "l",
            referent!(M1.BEN) => "r",
            referent!(M1.DET) => "ř",

            referent!(M2.NEU) => "s",
            referent!(M2.BEN) => "š",
            referent!(M2.DET) => "ž",

            referent!(P2.NEU) => "n",
            referent!(P2.BEN) => "t",
            referent!(P2.DET) => "d",

            referent!(MA.NEU) => "m",
            referent!(MA.BEN) => "p",
            referent!(MA.DET) => "b",

            referent!(PA.NEU) => "ň",
            referent!(PA.BEN) => "k",
            referent!(PA.DET) => "g",

            referent!(MI.NEU) => "z",
            referent!(MI.BEN) => "ţ",
            referent!(MI.DET) => "ḑ",

            referent!(PI.NEU) => "ẓ",
            referent!(PI.BEN) => "f",
            referent!(PI.DET) => "v",

            referent!(Mx.NEU) => "c",
            referent!(Mx.BEN) => "č",
            referent!(Mx.DET) => "j",

            referent!(Rdp.NEU) => "th",
            referent!(Rdp.BEN) => "kh",
            referent!(Rdp.DET) => "ph",

            referent!(Obv.NEU) => "lç",
            referent!(Obv.BEN) => "rç",
            referent!(Obv.DET) => "řç",

            referent!(PVS.NEU) => "mç",
            referent!(PVS.BEN) => "nç",
            referent!(PVS.DET) => "ňç",
        }
    }

    /// Gets the string representing this referent in a non-referential affix.
    pub const fn to_normal_str(self) -> &'static str {
        match self {
            referent!(M1.NEU) => "l",
            referent!(M1.BEN) => "r",
            referent!(M1.DET) => "ř",

            referent!(M2.NEU) => "s",
            referent!(M2.BEN) => "š",
            referent!(M2.DET) => "ž",

            referent!(P2.NEU) => "n",
            referent!(P2.BEN) => "t",
            referent!(P2.DET) => "d",

            referent!(MA.NEU) => "m",
            referent!(MA.BEN) => "p",
            referent!(MA.DET) => "b",

            referent!(PA.NEU) => "ň",
            referent!(PA.BEN) => "k",
            referent!(PA.DET) => "g",

            referent!(MI.NEU) => "z",
            referent!(MI.BEN) => "ţ",
            referent!(MI.DET) => "ḑ",

            referent!(PI.NEU) => "ẓ",
            referent!(PI.BEN) => "f",
            referent!(PI.DET) => "v",

            referent!(Mx.NEU) => "c",
            referent!(Mx.BEN) => "č",
            referent!(Mx.DET) => "j",

            referent!(Rdp.NEU) => "th",
            referent!(Rdp.BEN) => "kh",
            referent!(Rdp.DET) => "ph",

            referent!(Obv.NEU) => "ll",
            referent!(Obv.BEN) => "rr",
            referent!(Obv.DET) => "řř",

            referent!(PVS.NEU) => "mm",
            referent!(PVS.BEN) => "nn",
            referent!(PVS.DET) => "ňň",
        }
    }
}

impl Gloss for Referent {
    fn gloss(&self, flags: GlossFlags) -> String {
        let mut output = self.target.gloss(flags);

        if self.effect != ReferentEffect::NEU || flags.matches(GlossFlags::SHOW_DEFAULTS) {
            output += ".";
            output += self.effect.gloss_static(flags);
        }

        output
    }
}

/// A list of referents with a perspective.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReferentList<PerspectiveType> {
    /// The referents of this referent list.
    pub referents: Vec1<Referent>,

    /// The perspective of this referent list.
    pub perspective: PerspectiveType,
}

impl<PerspectiveType> Gloss for ReferentList<PerspectiveType>
where
    PerspectiveType: Default + GlossStatic + PartialEq,
{
    fn gloss(&self, flags: GlossFlags) -> String {
        let needs_brackets = self.referents.len() != 1
            || self.perspective != PerspectiveType::default()
            || flags.matches(GlossFlags::SHOW_DEFAULTS);

        let mut output = String::new();

        if needs_brackets {
            output += "[";
        }

        let mut is_first_segment = true;

        for referent in &self.referents {
            if !is_first_segment {
                output += "+";
            }

            is_first_segment = false;

            output += &referent.gloss(flags);
        }

        if self.perspective != PerspectiveType::default()
            || flags.matches(GlossFlags::SHOW_DEFAULTS)
        {
            if !is_first_segment {
                output += "+";
            }

            output += self.perspective.gloss_static(flags);
        }

        if needs_brackets {
            output += "]";
        }

        output
    }
}

/// A list of referents used in referential formatives.
pub type PerspectivelessReferentList = ReferentList<()>;

/// A list of referents used in referentials.
pub type NormalReferentList = ReferentList<Perspective>;

/// A list of referents used in referential affixes.
pub type AffixualReferentList = ReferentList<ReferentialAffixPerspective>;

impl FromStr for PerspectivelessReferentList {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseError::ReferentEmpty);
        }

        let mut referents = vec![];
        let mut chars: Vec<_> = s.chars().collect();
        chars.reverse();

        macro_rules! alternate_referent {
            ($char:pat, $default:expr, $alt:expr) => {
                match chars.last() {
                    None => $default,

                    Some($char) => {
                        chars.pop();
                        $alt
                    }

                    Some(_) => $default,
                }
            };
        }

        loop {
            let referent = match chars.pop() {
                None => break,

                Some('l') => alternate_referent!('l', referent!(M1.NEU), referent!(Obv.NEU)),
                Some('r') => alternate_referent!('r', referent!(M1.BEN), referent!(Obv.BEN)),
                Some('ř') => alternate_referent!('ř', referent!(M1.DET), referent!(Obv.DET)),

                Some('s') => referent!(M2.NEU),
                Some('š') => referent!(M2.BEN),
                Some('ž') => referent!(M2.DET),

                Some('n') => alternate_referent!('n', referent!(P2.NEU), referent!(PVS.BEN)),
                Some('t') => alternate_referent!('h', referent!(P2.BEN), referent!(Rdp.NEU)),
                Some('d') => referent!(P2.DET),

                Some('m') => alternate_referent!('m', referent!(MA.NEU), referent!(PVS.NEU)),
                Some('p') => alternate_referent!('h', referent!(MA.BEN), referent!(Rdp.BEN)),
                Some('b') => referent!(MA.DET),

                Some('ň') => alternate_referent!('ň', referent!(PA.NEU), referent!(PVS.DET)),
                Some('k') => alternate_referent!('h', referent!(PA.BEN), referent!(Rdp.DET)),
                Some('g') => referent!(PA.DET),

                Some('z') => referent!(MI.NEU),
                Some('ţ') => referent!(MI.BEN),
                Some('ḑ') => referent!(MI.DET),

                Some('ẓ') => referent!(PI.NEU),
                Some('f') => referent!(PI.BEN),
                Some('v') => referent!(PI.DET),

                Some('c') => referent!(Mx.NEU),
                Some('č') => referent!(Mx.BEN),
                Some('j') => referent!(Mx.DET),

                Some(_) => return Err(ParseError::ReferentInvalid),
            };

            referents.push(referent);
        }

        match Vec1::try_from_vec(referents) {
            Ok(referents) => Ok(Self {
                referents,
                perspective: (),
            }),
            Err(_) => Err(ParseError::ReferentEmpty),
        }
    }
}

impl FromStr for NormalReferentList {
    type Err = ParseError;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let mut perspective = Perspective::M;

        for (form, form_perspective) in [
            ("tļ", Perspective::G),
            ("ļ", Perspective::G),
            ("ç", Perspective::N),
            ("x", Perspective::N),
            ("w", Perspective::A),
            ("y", Perspective::A),
        ] {
            if s.starts_with(form) {
                s = &s[form.len()..];
                perspective = form_perspective;
                break;
            }

            if s.ends_with(form) {
                s = &s[..s.len() - form.len()];
                perspective = form_perspective;
                break;
            }
        }

        if s.is_empty() {
            return Err(ParseError::ReferentEmpty);
        }

        let mut referents = vec![];
        let mut chars: Vec<_> = s.chars().collect();
        chars.reverse();

        macro_rules! alternate_referent {
            ($char:pat, $default:expr, $alt:expr) => {
                match chars.last() {
                    None => $default,

                    Some($char) => {
                        chars.pop();
                        $alt
                    }

                    Some(_) => $default,
                }
            };
        }

        loop {
            let referent = match chars.pop() {
                None => break,

                Some('l') => alternate_referent!('l', referent!(M1.NEU), referent!(Obv.NEU)),
                Some('r') => alternate_referent!('r', referent!(M1.BEN), referent!(Obv.BEN)),
                Some('ř') => alternate_referent!('ř', referent!(M1.DET), referent!(Obv.DET)),

                Some('s') => referent!(M2.NEU),
                Some('š') => referent!(M2.BEN),
                Some('ž') => referent!(M2.DET),

                Some('n') => alternate_referent!('n', referent!(P2.NEU), referent!(PVS.BEN)),
                Some('t') => alternate_referent!('h', referent!(P2.BEN), referent!(Rdp.NEU)),
                Some('d') => referent!(P2.DET),

                Some('m') => alternate_referent!('m', referent!(MA.NEU), referent!(PVS.NEU)),
                Some('p') => alternate_referent!('h', referent!(MA.BEN), referent!(Rdp.BEN)),
                Some('b') => referent!(MA.DET),

                Some('ň') => alternate_referent!('ň', referent!(PA.NEU), referent!(PVS.DET)),
                Some('k') => alternate_referent!('h', referent!(PA.BEN), referent!(Rdp.DET)),
                Some('g') => referent!(PA.DET),

                Some('z') => referent!(MI.NEU),
                Some('ţ') => referent!(MI.BEN),
                Some('ḑ') => referent!(MI.DET),

                Some('ẓ') => referent!(PI.NEU),
                Some('f') => referent!(PI.BEN),
                Some('v') => referent!(PI.DET),

                Some('c') => referent!(Mx.NEU),
                Some('č') => referent!(Mx.BEN),
                Some('j') => referent!(Mx.DET),

                Some(_) => return Err(ParseError::ReferentInvalid),
            };

            referents.push(referent);
        }

        match Vec1::try_from_vec(referents) {
            Ok(referents) => Ok(Self {
                referents,
                perspective,
            }),
            Err(_) => Err(ParseError::ReferentEmpty),
        }
    }
}

impl FromStr for AffixualReferentList {
    type Err = ParseError;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let mut perspective = ReferentialAffixPerspective::M;

        for (form, form_perspective) in [
            ("tļ", ReferentialAffixPerspective::G),
            ("ļ", ReferentialAffixPerspective::G),
            ("ç", ReferentialAffixPerspective::N),
            ("x", ReferentialAffixPerspective::N),
        ] {
            if s.starts_with(form) {
                s = &s[form.len()..];
                perspective = form_perspective;
                break;
            }
        }

        if matches!(perspective, ReferentialAffixPerspective::M) {
            for (form, form_perspective, condition) in [
                ("tļ", ReferentialAffixPerspective::G, true),
                ("ļ", ReferentialAffixPerspective::G, !s.ends_with("tļ")),
                (
                    "ç",
                    ReferentialAffixPerspective::N,
                    match s.chars().nth_back(1) {
                        None => false,
                        Some('l' | 'r' | 'ř' | 'm' | 'n' | 'ň') => false,
                        Some(_) => true,
                    },
                ),
                ("x", ReferentialAffixPerspective::N, true),
            ] {
                if condition && s.ends_with(form) {
                    s = &s[..s.len() - form.len()];
                    perspective = form_perspective;
                    break;
                }
            }
        }

        if s.is_empty() {
            return Err(ParseError::ReferentEmpty);
        }

        let mut referents = Vec::new();
        let mut chars: Vec<_> = s.chars().collect();
        chars.reverse();

        /// Matches one of two referents depending on whether the next character of the input string
        /// matches a given pattern.
        macro_rules! alternate_referent {
            ($char:pat, $default:expr, $alt:expr) => {
                match chars.last() {
                    None => $default,

                    Some($char) => {
                        chars.pop();
                        $alt
                    }

                    Some(_) => $default,
                }
            };
        }

        loop {
            let referent = match chars.pop() {
                None => break,

                Some('l') => alternate_referent!('ç', referent!(M1.NEU), referent!(Obv.NEU)),
                Some('r') => alternate_referent!('ç', referent!(M1.BEN), referent!(Obv.BEN)),
                Some('ř') => alternate_referent!('ç', referent!(M1.DET), referent!(Obv.DET)),

                Some('s') => referent!(M2.NEU),
                Some('š') => referent!(M2.BEN),
                Some('ž') => referent!(M2.DET),

                Some('n') => alternate_referent!('ç', referent!(P2.NEU), referent!(PVS.BEN)),
                Some('t') => alternate_referent!('h', referent!(P2.BEN), referent!(Rdp.NEU)),
                Some('d') => referent!(P2.DET),

                Some('m') => alternate_referent!('ç', referent!(MA.NEU), referent!(PVS.NEU)),
                Some('p') => alternate_referent!('h', referent!(MA.BEN), referent!(Rdp.BEN)),
                Some('b') => referent!(MA.DET),

                Some('ň') => alternate_referent!('ç', referent!(PA.NEU), referent!(PVS.DET)),
                Some('k') => alternate_referent!('h', referent!(PA.BEN), referent!(Rdp.DET)),
                Some('g') => referent!(PA.DET),

                Some('z') => referent!(MI.NEU),
                Some('ţ') => referent!(MI.BEN),
                Some('ḑ') => referent!(MI.DET),

                Some('ẓ') => referent!(PI.NEU),
                Some('f') => referent!(PI.BEN),
                Some('v') => referent!(PI.DET),

                Some('c') => referent!(Mx.NEU),
                Some('č') => referent!(Mx.BEN),
                Some('j') => referent!(Mx.DET),

                Some(_) => return Err(ParseError::ReferentInvalid),
            };

            referents.push(referent);
        }

        match Vec1::try_from_vec(referents) {
            Ok(referents) => Ok(Self {
                referents,
                perspective,
            }),
            Err(_) => Err(ParseError::ReferentEmpty),
        }
    }
}

impl ToString for PerspectivelessReferentList {
    fn to_string(&self) -> String {
        // HACK: This isn't exact, but it's better than just reallocating constantly.
        let mut output = String::with_capacity(self.referents.len() * 3 / 2);

        for referent in &self.referents {
            output += referent.to_normal_str();
        }

        output
    }
}

impl ToString for NormalReferentList {
    fn to_string(&self) -> String {
        // HACK: This isn't exact, but it's better than just reallocating constantly.
        let mut output = String::with_capacity(self.referents.len() * 3 / 2);

        for referent in &self.referents {
            output += referent.to_normal_str();
        }

        // TODO: Use -ç- when possible.
        match self.perspective {
            Perspective::M => {}
            Perspective::G => output += "ļ",
            Perspective::N => output += "x",
            Perspective::A => output += "w",
        }

        output
    }
}

impl ToString for AffixualReferentList {
    fn to_string(&self) -> String {
        // HACK: This isn't exact, but it's better than just reallocating constantly.
        let mut output = String::with_capacity(self.referents.len() * 3 / 2);

        for referent in &self.referents {
            output += referent.to_referential_affix_str();
        }

        // TODO: Use -ç- when possible.
        match self.perspective {
            ReferentialAffixPerspective::M => {}
            ReferentialAffixPerspective::G => output += "ļ",
            ReferentialAffixPerspective::N => output += "x",
        }

        output
    }
}
