//! Defines traits used to convert words into Ithkuil.

use super::character::Character;

/// Allows types to be converted into script characters.
pub trait IntoScript {
    /// Appends this item as script characters to a [`Vec<Character>`].
    fn append_script_to(&self, list: &mut Vec<Character>);
}
