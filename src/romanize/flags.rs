//! Contains flags used to modify how words are parsed.

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

/// A bit flag of options passed to parsing functions.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FromTokenFlags(u8);

impl FromTokenFlags {
    /// A [`FromStreamFlags`] instance with no flags active.
    pub const NONE: Self = Self(0);

    /// A [`FromStreamFlags`] instance with only the `permissive` flag enabled.
    ///
    /// If passed to `.parse()`, objects will let through many errors and parse words to the best of
    /// their intent, even if the word isn't actually valid.
    pub const PERMISSIVE: Self = Self(1 << 0);

    /// Checks if `self` contains the flags specified in `other`.
    pub const fn matches(self, other: FromTokenFlags) -> bool {
        self.0 & other.0 == other.0
    }
}

impl BitAnd for FromTokenFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for FromTokenFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitXor for FromTokenFlags {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitAndAssign for FromTokenFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOrAssign for FromTokenFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXorAssign for FromTokenFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}
