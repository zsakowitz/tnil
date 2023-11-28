//! Provides different formative root types.

use vec1::Vec1;

use crate::{
    affix::NumericAffix,
    category::{AffixDegree, Referent},
    gloss::{Gloss, GlossFlags, GlossStatic},
    AsGeneral, TryAsSpecific,
};

/// A normal formative root.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NormalFormativeRoot {
    /// The Cr form of this root.
    pub cr: String,
}

impl NormalFormativeRoot {
    /// Creates a new [`NormalRoot`] from a Cr form.
    pub fn new(cr: impl Into<String>) -> Self {
        Self { cr: cr.into() }
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
            self.cr.clone()
        }
    }
}

/// A numeric formative root.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumericFormativeRoot {
    /// The value of this root.
    value: NumericAffix,
}

impl NumericFormativeRoot {
    /// Creates a new [`NumericRoot`] from an integer part.
    pub const fn new(integer_part: u64) -> Self {
        NumericFormativeRoot {
            value: NumericAffix::new(integer_part),
        }
    }
}

impl Gloss for NumericFormativeRoot {
    fn gloss(&self, flags: GlossFlags) -> String {
        self.value.gloss(flags)
    }
}

/// A referential formative root.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReferentialFormativeRoot {
    /// The referents of this formative.
    pub referents: Vec1<Referent>,
}

impl ReferentialFormativeRoot {
    /// Constructs a new [`ReferentialRoot`] from a list of referents.
    pub fn new(referents: Vec1<Referent>) -> Self {
        Self { referents }
    }
}

impl Gloss for ReferentialFormativeRoot {
    fn gloss(&self, flags: GlossFlags) -> String {
        if self.referents.len() == 1 {
            self.referents.first().gloss(flags)
        } else {
            let mut output = "[".to_owned();
            output += &self.referents.first().gloss(flags);
            for el in &self.referents[1..] {
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
    /// Creates a new [`AffixualRoot`] from a Cs form and affix degree.
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
pub enum GeneralFormativeRoot {
    /// The root of a normal formative.
    Normal(NormalFormativeRoot),

    /// The root of a numeric formative.
    Numeric(NumericFormativeRoot),

    /// The root of a referential formative.
    Referential(ReferentialFormativeRoot),

    /// The root of an affixual formative.
    Affixual(AffixualFormativeRoot),
}

macro_rules! as_general_impl {
    ($specific:ident, $variant:ident) => {
        impl AsGeneral for $specific {
            type Output = GeneralFormativeRoot;

            fn as_general(self) -> Self::Output {
                GeneralFormativeRoot::$variant(self)
            }
        }

        impl From<$specific> for GeneralFormativeRoot {
            fn from(value: $specific) -> Self {
                value.as_general()
            }
        }

        impl TryAsSpecific<$specific> for GeneralFormativeRoot {
            fn try_as_specific(self) -> Option<$specific> {
                match self {
                    Self::$variant(value) => Some(value),
                    _ => None,
                }
            }
        }
    };
}

impl Gloss for GeneralFormativeRoot {
    fn gloss(&self, flags: GlossFlags) -> String {
        match self {
            Self::Normal(value) => value.gloss(flags),
            Self::Numeric(value) => value.gloss(flags),
            Self::Referential(value) => value.gloss(flags),
            Self::Affixual(value) => value.gloss(flags),
        }
    }
}

as_general_impl!(NormalFormativeRoot, Normal);
as_general_impl!(NumericFormativeRoot, Numeric);
as_general_impl!(ReferentialFormativeRoot, Referential);
as_general_impl!(AffixualFormativeRoot, Affixual);
