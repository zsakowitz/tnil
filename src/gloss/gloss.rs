use super::GlossFlags;

/// Allows types to be glossed.
///
/// Glosses should be done in the style of @ırburučpaızya, or in a style consistent with the other
/// glossing functionality of this crate. In particular, these conventions are followed:
///
/// - If root meanings are known (e.g. "dog" for zv), they are enclosed in “rounded double quotes”.
///   Otherwise, roots are represented lowercase in bold or as plain Cr forms.
///
/// - If affix meanings are known (e.g. "large" for x/7), they are enclosed in ‘rounded single
///   quotes’ followed by one of `₁₂₃` indicating the affix type. If the value is not known, the
///   syntax cs₂/7 is used, where cs is replaced by the Cs form, the subscript is replaced with the
///   affix type, and /7 is replaced with the corresponding degree. The Cs form should be in bold if
///   possible.
///
/// - Category names are abbreviated UPPERCASE when glossed without the `GlossFlags::LONG` flag and
///   unabbreviated lowercase when glossed with the `GlossFlags::LONG` flag.
///
/// - Multiple categories represented within a single slot are separated by `.`. The exception is
///   affixes, which are separated by `-` when multiple are present.
///
/// - Multiple categories spanning multiple slots are separated by `-`.
///
/// - Affixes other than plain affixes (e.g. case-stackers, case-accessors, or Ca-stackers) are
///   shown in parentheses.
///
/// - Multiple referents in a single slot are separated with `+` and surrounded by `[...]`.
pub trait Gloss {
    /// Glosses this value with a set of flags, returning the gloss as an allocated [`String`].
    ///
    /// Prefer using [`GlossStatic::gloss_static`] over [`Gloss::gloss`] when possible, as it avoids
    /// allocating new memory. However, this method is often required when glossing complex types.
    fn gloss(&self, flags: GlossFlags) -> String;

    /// Glosses this value with a set of flags, returning the gloss as an allocated [`String`]. If
    /// `flags` does not include `GlossFlags::SHOW_DEFAULTS` and `self` is the default value of
    /// its type, an empty string is returned.
    ///
    /// Prefer using [`GlossStatic::gloss_static_non_default`] over [`Gloss::gloss`] when possible,
    /// as it avoids allocating new memory. However, this method is often required when glossing
    /// complex types.
    fn gloss_non_default(&self, flags: GlossFlags) -> String
    where
        Self: Default + PartialEq,
    {
        if flags.matches(GlossFlags::SHOW_DEFAULTS) || *self != Self::default() {
            self.gloss(flags)
        } else {
            String::new()
        }
    }
}

/// Allows types to be glossed without allocating.
pub trait GlossStatic {
    /// Glosses this value with a set of flags, returning the gloss as a `&'static str`.
    ///
    /// Prefer using this method over [`Gloss::gloss`] where possible, as it avoids allocating new
    /// memory.
    fn gloss_static(&self, flags: GlossFlags) -> &'static str;

    /// Glosses this value with a set of flags, returning the gloss as a `&'static str`. If `flags`
    /// does not include `GlossFlags::SHOW_DEFAULTS` and `self` is the default value of
    /// its type, an empty string is returned.
    ///
    /// Prefer using this method over [`Gloss::gloss_non_default`] where possible, as it avoids
    /// allocating new memory.
    fn gloss_static_non_default(&self, flags: GlossFlags) -> &'static str
    where
        Self: Default + PartialEq,
    {
        if flags.matches(GlossFlags::SHOW_DEFAULTS) || *self != Self::default() {
            self.gloss_static(flags)
        } else {
            ""
        }
    }
}

impl<T: GlossStatic> Gloss for T {
    fn gloss(&self, flags: GlossFlags) -> String {
        self.gloss_static(flags).to_owned()
    }

    fn gloss_non_default(&self, flags: GlossFlags) -> String
    where
        Self: Default + PartialEq,
    {
        self.gloss_static_non_default(flags).to_owned()
    }
}
