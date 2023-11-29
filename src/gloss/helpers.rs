use sealed::Sealed;

mod sealed {
    pub trait Sealed {}
}

/// Helpers for glossing text and creating complex glosses.
pub trait GlossHelpers: Sealed {
    /// Adds `value` to `self` if `value != ""`, inserting `separator` before `value` if `self` is
    /// non-empty.
    fn add_with_separator(&mut self, separator: &str, value: &str);

    /// Adds `value` to `self` if `value != ""`, inserting `.` before `value` if `self` is
    /// non-empty.
    fn add_dotted(&mut self, value: &str) {
        self.add_with_separator(".", value)
    }

    /// Adds `value` to `self` if `value != ""`, inserting `-` before `value` if `self` is
    /// non-empty.
    fn add_dashed(&mut self, value: &str) {
        self.add_with_separator("-", value)
    }
}

impl Sealed for String {}

impl GlossHelpers for String {
    #[inline(always)]
    fn add_with_separator(&mut self, separator: &str, value: &str) {
        if value.is_empty() {
            return;
        }

        if !self.is_empty() {
            *self += separator;
        }

        *self += value;
    }
}
