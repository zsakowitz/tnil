use super::GlossFlags;

/// Allows types to be glossed.
pub trait Gloss {
    /// Glosses this value with a set of flags, returning the gloss as an allocated [`String`].
    ///
    /// Prefer using [`GlossStatic::gloss_static`] over [`Gloss::gloss`] when possible, as it avoids
    /// allocating new memory. However, this method is often required when glossing complex types.
    fn gloss(&self, flags: GlossFlags) -> String;

    /// Glosses this value with a set of flags, returning the gloss as an allocated [`String`].
    /// If `flags` does not include `GlossFlags::SHOW_DEFAULTS` and `self` is the default value of
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

    /// Glosses this value with a set of flags, returning the gloss as an allocated [`String`].
    /// If `flags` does not include `GlossFlags::SHOW_DEFAULTS` and `self` is the default value of
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
        if flags.matches(GlossFlags::SHOW_DEFAULTS) || *self != Self::default() {
            self.gloss_static(flags).to_owned()
        } else {
            String::new()
        }
    }
}
