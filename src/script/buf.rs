//! Declares a type containing a buffer of script characters.

use super::{
    character::Character,
    flags::IntoScriptFlags,
    traits::{IntoCharacter, IntoScript},
};

/// A buffer of characters.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CharacterBuf {
    /// The inner vector of this buffer.
    pub vec: Vec<Character>,
}

impl CharacterBuf {
    /// Creates a new buffer.
    pub fn new() -> Self {
        CharacterBuf { vec: Vec::new() }
    }

    /// Creates a new buffer with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        CharacterBuf {
            vec: Vec::with_capacity(capacity),
        }
    }

    /// Pushes a character into this buffer.
    pub fn push<T: IntoCharacter>(&mut self, item: T) {
        self.vec.push(item.into_character());
    }

    /// Appends an item into this buffer as a list of characters.
    pub fn append<T: IntoScript>(&mut self, item: T, flags: IntoScriptFlags) {
        item.append_script_to(self, flags);
    }
}
