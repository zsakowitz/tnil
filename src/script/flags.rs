//! Provides a flag types used for modifying script output.

use crate::macros::bitflags;

/// A bit flag of options passed to script generation functions.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntoScriptFlags(u8);

impl IntoScriptFlags {
    /// A [`IntoScriptFlags`] instance with no flags active.
    pub const NONE: Self = Self(0);

    /// A [`IntoScriptFlags`] instance with only the `elide_primaries` flag enabled.
    ///
    /// If passed to `.append_script_to()`, formatives will elide their primary characters when
    /// possible.
    pub const ELIDE_PRIMARIES: Self = Self(1 << 0);

    /// A [`IntoScriptFlags`] instance with only the `keep_quaternaries` flag enabled.
    ///
    /// If passed to `.append_script_to()`, formatives will not elide quaternary characters.
    pub const KEEP_QUATERNARIES: Self = Self(1 << 1);

    /// A [`IntoScriptFlags`] instance with only the `tertiaries_as_affixes` flag enabled.
    ///
    /// If passed to `.append_script_to()`, formatives will write affixes representing tertiary
    /// segments as normal affixes.
    pub const TERTIARIES_AS_AFFIXES: Self = Self(1 << 2);

    /// A [`IntoScriptFlags`] instance with only the `prefer_top_extensions` flag enabled.
    ///
    /// If passed to `.append_script_to()`, secondaries with two characters will prefer using a core
    /// shape with top extensions instead of the default, which is a core shape with a bottom
    /// extension.
    pub const PREFER_TOP_EXTENSIONS: Self = Self(1 << 3);

    /// A [`IntoScriptFlags`] instance with only the `prefer_bottom_tertiaries` flag enabled.
    ///
    /// If passed to `.append_script_to()`, tertiaries with a single segment will place that segment
    /// on the bottom of the character.
    pub const PREFER_BOTTOM_TERTIARIES: Self = Self(1 << 4);
}

bitflags!(IntoScriptFlags);
