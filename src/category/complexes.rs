/// Defines complexes (groups of core categories) and their glossing methods.
use super::{
    Affiliation, Aspect, Category, Configuration, Effect, Essence, Extension, Level, Perspective,
    Phase, Plexity, ReferentEffect, ReferentTarget, Separability, Similarity, Valence,
};
use crate::gloss::{Gloss, GlossFlags, GlossStatic};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
                self.affiliation.gloss_static(flags),
                self.configuration.gloss_static(flags),
                self.extension.gloss_static(flags),
                self.perspective.gloss_static(flags),
                self.essence.gloss_static(flags),
            ] {
                if item != "" {
                    if output != "" {
                        output += ".";
                    }

                    output += item;
                }
            }

            if output == "" {
                "{Ca}".to_owned()
            } else {
                output
            }
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
