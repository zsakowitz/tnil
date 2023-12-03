use std::{
    mem::transmute,
    ops::{Deref, DerefMut},
};

/// A consonant form wrapping an internal `str`.
///
/// This type is unsized, and thus can only be used behind some kind of pointer.
#[repr(transparent)]
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConsonantForm(str);

impl From<&str> for &ConsonantForm {
    fn from(value: &str) -> Self {
        // SAFETY: `&str` and `&ConsonantForm` have the same layout
        #[allow(unsafe_code)]
        unsafe {
            transmute(value)
        }
    }
}

impl From<&mut str> for &mut ConsonantForm {
    fn from(value: &mut str) -> Self {
        // SAFETY: `&mut str` and `&mut ConsonantForm` have the same layout
        #[allow(unsafe_code)]
        unsafe {
            transmute(value)
        }
    }
}

impl ConsonantForm {
    /// Creates a reference to a `ConsonantForm` from a `&str`.
    pub fn new(value: &str) -> &Self {
        value.into()
    }

    /// Checks if this consonant form contains a geminate.
    ///
    /// # Examples
    ///
    /// ```
    /// use tnil::romanize::token::ConsonantForm;
    ///
    /// let consonant_form = ConsonantForm::new("rrw");
    /// assert!(consonant_form.is_geminate());
    ///
    /// let consonant_form = ConsonantForm::new("zv");
    /// assert!(!consonant_form.is_geminate());
    /// ```
    pub fn is_geminate(&self) -> bool {
        let mut chars = self.0.chars();

        let Some(mut last_char) = chars.next() else {
            return false;
        };

        for char in chars {
            if char == last_char {
                return true;
            }

            last_char = char;
        }

        false
    }
}

impl Deref for ConsonantForm {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ConsonantForm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
