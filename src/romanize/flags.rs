//! Contains flags used to modify how words are parsed.

use crate::macros::bitflags;

/// A bit flag of options passed to parsing functions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FromTokenFlags(u8);

impl FromTokenFlags {
    /// A [`FromTokenFlags`] instance with no flags active.
    pub const NONE: Self = Self(0);

    /// A [`FromTokenFlags`] instance with only the `permissive` flag enabled.
    ///
    /// If passed to `.parse()`, objects will let through many errors and parse words to the best of
    /// their intent, even if the word isn't actually valid.
    pub const PERMISSIVE: Self = Self(1 << 0);
}

bitflags!(FromTokenFlags);

/// A bit flag of options passed to generation functions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct IntoTokensFlags(u8);

impl IntoTokensFlags {
    /// A [`IntoTokensFlags`] instance with no flags active.
    pub const NONE: Self = Self(0);

    /// A [`IntoTokensFlags`] instance with only the `word_initial_vowel` flag enabled.
    ///
    /// If passed to `.append_tokens_to()`, objects will attempt to add a word initial vowel to
    /// themselves.
    pub const WORD_INITIAL_VOWEL: Self = Self(1 << 0);

    /// A [`IntoTokensFlags`] instance with only the `word_final_vowel` flag enabled.
    ///
    /// If passed to `.append_tokens_to()`, objects will attempt to add a word final vowel to
    /// themselves.
    pub const WORD_FINAL_VOWEL: Self = Self(1 << 1);

    /// A [`IntoTokensFlags`] instance with only the `move_vc_glottal_stop` flag enabled.
    ///
    /// If passed to `.append_tokens_to()`, formatives will try to move Vc glottal stops into Vr or
    /// Vn.
    pub const MOVE_VC_GLOTTAL_STOP: Self = Self(1 << 2);
}

bitflags!(IntoTokensFlags);
