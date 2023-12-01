//! Provides several grammatical categories, as well as conversions between them where appropriate.

mod complexes;
mod conversions;
mod items;

pub use complexes::*;
pub use conversions::*;
pub use items::*;

/// Marks types as grammatical categories.
pub trait Category {
    /// Gets the abbreviation representing this item.
    fn abbr(self) -> &'static str;

    /// Gets the full name of this item.
    fn name(self) -> &'static str;
}
