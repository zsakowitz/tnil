//! Defines complexes (groups of core categories) and their glossing methods.

use std::str::FromStr;

use vec1::Vec1;

use super::{
    Affiliation, Aspect, CaseScope, Category, Configuration, Effect, Essence, Extension, Level,
    Mood, Perspective, Phase, Plexity, ReferentEffect, ReferentTarget, ReferentialAffixPerspective,
    Separability, Similarity, Valence,
};
use crate::{
    gloss::{Gloss, GlossFlags, GlossHelpers, GlossStatic},
    romanize::stream::ParseError,
};

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
    /// Glosses this value with a set of flags, returning the gloss as a `&'static str`.
    /// If `flags` does not include `GlossFlags::SHOW_DEFAULTS` and `self` is FAC or CCN, the empty
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

    /// Glosses this value with a set of flags, returning the gloss as an allocated [`String`].
    /// If `flags` does not include `GlossFlags::SHOW_DEFAULTS` and `self` is FAC or CCN, the empty
    /// string is returned.
    pub fn gloss_non_fac_ccn(&self, flags: GlossFlags) -> String {
        self.gloss_static_non_fac_ccn(flags).to_owned()
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// A referent with a target and effect.
pub struct Referent {
    /// The target of this referent.
    target: ReferentTarget,

    /// The effect of this referent.
    effect: ReferentEffect,
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
    referents: Vec1<Referent>,

    /// The perspective of this referent list.
    perspective: PerspectiveType,
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

/// A list of referents used in referentials.
pub type NormalReferentList = ReferentList<Perspective>;

/// A list of referents used in referential affixes.
pub type AffixualReferentList = ReferentList<ReferentialAffixPerspective>;

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

        macro_rules! referent {
            ($effect:ident, $target:ident) => {
                Referent {
                    effect: ReferentEffect::$effect,
                    target: ReferentTarget::$target,
                }
            };
        }

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

                Some('l') => alternate_referent!('l', referent!(NEU, M1), referent!(NEU, Obv)),
                Some('r') => alternate_referent!('r', referent!(BEN, M1), referent!(BEN, Obv)),
                Some('ř') => alternate_referent!('ř', referent!(DET, M1), referent!(DET, Obv)),

                Some('s') => referent!(NEU, M2),
                Some('š') => referent!(BEN, M2),
                Some('ž') => referent!(DET, M2),

                Some('n') => alternate_referent!('n', referent!(NEU, P2), referent!(BEN, PVS)),
                Some('t') => alternate_referent!('h', referent!(BEN, P2), referent!(NEU, Rdp)),
                Some('d') => referent!(DET, P2),

                Some('m') => alternate_referent!('m', referent!(NEU, MA), referent!(NEU, PVS)),
                Some('p') => alternate_referent!('h', referent!(BEN, MA), referent!(BEN, Rdp)),
                Some('b') => referent!(DET, MA),

                Some('ň') => alternate_referent!('ň', referent!(NEU, PA), referent!(DET, PVS)),
                Some('k') => alternate_referent!('h', referent!(BEN, PA), referent!(DET, Rdp)),
                Some('g') => referent!(DET, PA),

                Some('z') => referent!(NEU, MI),
                Some('ţ') => referent!(BEN, MI),
                Some('ḑ') => referent!(DET, MI),

                Some('ż') => referent!(NEU, PI),
                Some('f') => referent!(BEN, PI),
                Some('v') => referent!(DET, PI),

                Some('c') => referent!(NEU, Mx),
                Some('č') => referent!(BEN, Mx),
                Some('j') => referent!(DET, Mx),

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

        /// A shortcut for a full referent.
        macro_rules! referent {
            ($effect:ident, $target:ident) => {
                Referent {
                    effect: ReferentEffect::$effect,
                    target: ReferentTarget::$target,
                }
            };
        }

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

                Some('l') => alternate_referent!('ç', referent!(NEU, M1), referent!(NEU, Obv)),
                Some('r') => alternate_referent!('ç', referent!(BEN, M1), referent!(BEN, Obv)),
                Some('ř') => alternate_referent!('ç', referent!(DET, M1), referent!(DET, Obv)),

                Some('s') => referent!(NEU, M2),
                Some('š') => referent!(BEN, M2),
                Some('ž') => referent!(DET, M2),

                Some('n') => alternate_referent!('ç', referent!(NEU, P2), referent!(BEN, PVS)),
                Some('t') => alternate_referent!('h', referent!(BEN, P2), referent!(NEU, Rdp)),
                Some('d') => referent!(DET, P2),

                Some('m') => alternate_referent!('ç', referent!(NEU, MA), referent!(NEU, PVS)),
                Some('p') => alternate_referent!('h', referent!(BEN, MA), referent!(BEN, Rdp)),
                Some('b') => referent!(DET, MA),

                Some('ň') => alternate_referent!('ç', referent!(NEU, PA), referent!(DET, PVS)),
                Some('k') => alternate_referent!('h', referent!(BEN, PA), referent!(DET, Rdp)),
                Some('g') => referent!(DET, PA),

                Some('z') => referent!(NEU, MI),
                Some('ţ') => referent!(BEN, MI),
                Some('ḑ') => referent!(DET, MI),

                Some('ż') => referent!(NEU, PI),
                Some('f') => referent!(BEN, PI),
                Some('v') => referent!(DET, PI),

                Some('c') => referent!(NEU, Mx),
                Some('č') => referent!(BEN, Mx),
                Some('j') => referent!(DET, Mx),

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
