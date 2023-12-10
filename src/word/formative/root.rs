//! Provides different formative root types.

use std::str::FromStr;

use crate::{
    category::{AffixDegree, PerspectivelessReferentList},
    gloss::{Gloss, GlossFlags, GlossStatic},
    romanize::{stream::ParseError, token::OwnedConsonantForm},
    specificity::{AsGeneral, TryAsSpecific},
};

/// A normal formative root.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NormalFormativeRoot {
    /// The Cr form of this root.
    pub cr: OwnedConsonantForm,
}

impl NormalFormativeRoot {
    /// Creates a new [`NormalFormativeRoot`] from a Cr form.
    pub fn new(cr: impl Into<String>) -> Self {
        Self {
            cr: OwnedConsonantForm(cr.into()),
        }
    }
}

impl Gloss for NormalFormativeRoot {
    // TODO: Use root data
    fn gloss(&self, flags: GlossFlags) -> String {
        if flags.matches(GlossFlags::FORMAT_MARKDOWN) {
            let mut output = "**".to_owned();
            output += &self.cr; // We assume the Cr form doesn't need Markdown escaping.
            output += "**";
            output
        } else {
            self.cr.0.to_owned()
        }
    }
}

/// A numeric formative root.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive] // TODO: Remove non_exhaustive once we decide how to work with decimals.
pub struct NumericFormativeRoot {
    /// The integer part of this root.
    pub integer_part: u64,
}

impl NumericFormativeRoot {
    /// Creates a new [`NumericFormativeRoot`] from an integer part.
    pub const fn new(integer_part: u64) -> Self {
        NumericFormativeRoot { integer_part }
    }
}

impl Gloss for NumericFormativeRoot {
    fn gloss(&self, _flags: GlossFlags) -> String {
        let mut output = "“".to_owned();
        output += &self.integer_part.to_string();
        output += "”";
        output
    }
}

/// A referential formative root.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReferentialFormativeRoot {
    /// The referents of this formative.
    pub referents: PerspectivelessReferentList,
}

impl FromStr for ReferentialFormativeRoot {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ReferentialFormativeRoot {
            referents: s.parse()?,
        })
    }
}

impl Gloss for ReferentialFormativeRoot {
    fn gloss(&self, flags: GlossFlags) -> String {
        let referents = &self.referents.referents;

        if referents.len() == 1 {
            referents.first().gloss(flags)
        } else {
            let mut output = "[".to_owned();
            output += &referents.first().gloss(flags);
            for el in &referents[1..] {
                output += "+";
                output += &el.gloss(flags);
            }
            output += "]";
            output
        }
    }
}

/// An affixual formative root.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AffixualFormativeRoot {
    /// The Cs form of this affix.
    pub cs: String,

    /// The degree of this affix.
    pub degree: AffixDegree,
}

impl AffixualFormativeRoot {
    /// Creates a new [`AffixualFormativeRoot`] from a Cs form and affix degree.
    pub fn new(cs: impl Into<String>, degree: AffixDegree) -> Self {
        Self {
            cs: cs.try_into().unwrap(),
            degree,
        }
    }
}

impl Gloss for AffixualFormativeRoot {
    // TODO: Use affix data
    fn gloss(&self, flags: GlossFlags) -> String {
        if flags.matches(GlossFlags::FORMAT_MARKDOWN) {
            let mut output = "**".to_owned();
            output += &self.cs; // We assume the Cs form doesn't need Markdown escaping.
            output += "**/";
            output += self.degree.gloss_static(GlossFlags::NONE);
            output += "-D";
            output += self.degree.gloss_static(GlossFlags::NONE);
            output
        } else {
            let mut output = self.cs.clone();
            output += "/";
            output += self.degree.gloss_static(GlossFlags::NONE);
            output += "-D";
            output += self.degree.gloss_static(GlossFlags::NONE);
            return output;
        }
    }
}

/// A general formative root.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ShortcutCheckedFormativeRoot {
    /// The root of a normal formative.
    Normal(NormalFormativeRoot),

    /// The root of a numeric formative.
    Numeric(NumericFormativeRoot),

    /// The root of a referential formative.
    Referential(ReferentialFormativeRoot),

    /// The root of an affixual formative.
    Affixual(AffixualFormativeRoot),
}

impl ShortcutCheckedFormativeRoot {
    /// Creates a new [`ShortcutCheckedFormativeRoot::Normal`].
    pub fn new_normal(source: impl Into<String>) -> ShortcutCheckedFormativeRoot {
        ShortcutCheckedFormativeRoot::Normal(NormalFormativeRoot {
            cr: OwnedConsonantForm(source.into()),
        })
    }
}

impl Gloss for ShortcutCheckedFormativeRoot {
    fn gloss(&self, flags: GlossFlags) -> String {
        match self {
            Self::Normal(value) => value.gloss(flags),
            Self::Numeric(value) => value.gloss(flags),
            Self::Referential(value) => value.gloss(flags),
            Self::Affixual(value) => value.gloss(flags),
        }
    }
}

macro_rules! as_general_impl {
    ($specific:ident, $variant:ident) => {
        impl AsGeneral<ShortcutCheckedFormativeRoot> for $specific {
            fn as_general(self) -> ShortcutCheckedFormativeRoot {
                ShortcutCheckedFormativeRoot::$variant(self)
            }
        }

        impl From<$specific> for ShortcutCheckedFormativeRoot {
            fn from(value: $specific) -> Self {
                value.as_general()
            }
        }

        impl TryAsSpecific<$specific> for ShortcutCheckedFormativeRoot {
            fn try_as_specific(self) -> Option<$specific> {
                match self {
                    Self::$variant(value) => Some(value),
                    _ => None,
                }
            }
        }
    };
}

as_general_impl!(NormalFormativeRoot, Normal);
as_general_impl!(NumericFormativeRoot, Numeric);
as_general_impl!(ReferentialFormativeRoot, Referential);
as_general_impl!(AffixualFormativeRoot, Affixual);
