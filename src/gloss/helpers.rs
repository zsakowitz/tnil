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
        if !self.is_empty() {
            *self += separator;
        }

        *self += value;
    }
}

// TODO: Do we still need this
// /// Pushes `"-$segment1.$segment2.$segment3...` to `output`, omitting empty segments and omitting
// /// the whole item if all segments are empty.
// #[macro_export]
// macro_rules! add_dotted_segments_with_dash {
//     ($output:expr) => {
//         compile_error!("`add_dotted_segments_with_dash!()` needs at least one segment");
//     };

//     ($output:expr, $($x:expr),+ $(,)?) => {{
//         let output: &mut String = &mut $output;

//         let mut has_added_segment = false;

//         $(match &$x {
//             "" => {}
//             value => {
//                 if has_added_segment {
//                     output += ".";
//                 } else {
//                     output += "-";
//                     has_added_segment = true;
//                 }

//                 output += value;
//             }
//         };)+
//     }};
// }

// pub use add_dotted_segments_with_dash;
