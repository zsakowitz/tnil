//! Defines traits used to convert words into native Ithkuil script.

use super::{
    buf::CharacterBuf,
    character::{Character, Secondary},
    flags::IntoScriptFlags,
};

/// Allows types to be converted into a secondary character.
pub trait IntoSecondary {
    /// Converts this item into a secondary character.
    fn into_secondary(self) -> Secondary;
}

/// Allows types to be converted into a single character.
pub trait IntoCharacter {
    /// Converts this item into a character.
    fn into_character(self) -> Character;
}

/// Allows types to be converted into script characters.
pub trait IntoScript {
    /// Appends this item as script characters to a [`CharacterBuf`].
    fn append_script_to(&self, list: &mut CharacterBuf, flags: IntoScriptFlags);

    /// Converts this item into a [`CharacterBuf`].
    fn into_script(&self, flags: IntoScriptFlags) -> CharacterBuf {
        let mut list = CharacterBuf::new();
        self.append_script_to(&mut list, flags);
        list
    }
}

/// Allows script characters to be converted into a type.
pub trait ScriptRepr: Sized {
    /// Constructs an empty representation.
    fn new() -> Self;

    /// Pushes a character as script into this representation.
    fn push(&mut self, char: Character);

    /// Represents a character in this script format.
    fn from_char(char: Character) -> Self {
        let mut this = Self::new();
        this.push(char);
        this
    }

    /// Represents a set of characters in this script format.
    fn from_chars(chars: &[Character]) -> Self {
        let mut this = Self::new();
        for char in chars {
            this.push(*char);
        }
        this
    }

    /// Pushes an item as script characters into this representation.
    fn append<T: IntoScript>(&mut self, item: &T, flags: IntoScriptFlags) {
        for char in item.into_script(flags).vec {
            self.push(char);
        }
    }

    /// Encodes an item in this script representation.
    fn encode<T: IntoScript>(item: &T, flags: IntoScriptFlags) -> Self {
        let buf = item.into_script(flags);
        Self::from_chars(&buf.vec[..])
    }
}

impl<T: IntoSecondary> IntoCharacter for T {
    fn into_character(self) -> Character {
        Character::Secondary(self.into_secondary())
    }
}

impl<T: Clone + IntoCharacter> IntoScript for T {
    fn append_script_to(&self, list: &mut CharacterBuf, _flags: IntoScriptFlags) {
        list.push(self.clone());
    }
}
