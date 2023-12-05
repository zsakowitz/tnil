//! Provides types for working with formatives.
//!
//! ## Representation
//!
//! Even though formatives have regular structures, they are still very complex, and modeling them
//! with Rust's "make invalid states unrepresentable" mantra is quite difficult. Here are the main
//! points:
//!
//! Formatives come in five relations: nominal, unframed verbal, framed verbal, type-1 concatenated,
//! and type-2 concatenated. Most of them behave identically (programatically speaking), except for
//! unframed verbal formatives. Unframed verbal formatives take Mood and Illocution/Validation
//! slots, whereas other formatives take Case-Scope and Case slots. So that's one thing we have to
//! manage.
//!
//! Formatives also have four different types of roots: normal, numeric, referential, and affixual,
//! each which have slightly different fields.
//!
//! - Normal and numeric formatives have [`Stem`][crate::category::Stem] and
//!   [`Specification`][crate::category::Specification] fields.
//! - Referential formatives only have a [`Specification`][crate::category::Specification] field.
//! - Affixual formatives have neither.
//!
//! To theoretically model this complexity, we would split a formative into three parts: the
//! relation, the root, and everything else. However, there's a subtle problem with this model,
//! which is revealed as such:
//!
//! 1. The available shortcut types are limited by the type of a formative's root.
//! 2. Thus, the formative root must encode the shortcut.
//! 3. But one type of shortcut is a Cn shortcut.
//! 4. Cn shortcuts cannot shortcut a default Cn value.
//! 5. Thus, the Cn value must be represented as a `NonDefaultCn` when a Cn shortcut is active.
//! 6. The only way to do this is if the value is Cn value is included in the root.
//! 7. But the type of Cn value changes based on the relation.
//! 8. Thus, we need to somehow link the relation and root.
//! 9. The easiest way to do this is to have either relation types include roots or have root types
//!    include relations.
//!
//! To mitigate this nesting problem, this crate uses generics extensively to avoid code duplication
//! and an explosion of types, and includes an enum named [`Formative`] which represents all
//! possible formative types (normal, numeric, referential, and affixual).

pub mod additions;
pub mod core;
pub(super) mod formative;
pub mod relation;
pub mod root;

#[cfg(test)]
mod tests;
