//! Defines traits used to convert words into Ithkuil.

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
pub trait ScriptRepr {
    /// Represents a character in this script format.
    fn from_char(char: Character) -> Self;

    /// Represents a set of characters in this script format.
    fn from_chars(chars: &[Character]) -> Self;
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
