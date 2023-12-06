A crate to parse, generate, transform, and operate on text in the language of
New Ithkuil.

## Completed Goals

- **To model all words and grammatical categories in New Ithkuil.** Structs and
  enums are provided for all parts of the Ithkuil language and model it
  precisely, even if this leads to types that end up being difficult to work
  with. See the [`CheckedFormative`][crate::word::formative::Formative] type for
  an example of this complexity in action.

- **To provide glossing functionality.** Ithkuil text is often shown as glosses,
  which show the grammatical categories and roots of a word in a form easy to
  parse for humans. This crate provides the [`Gloss`][crate::gloss::Gloss] and
  [`GlossStatic`][crate::gloss::GlossStatic] traits to allow all types in the
  crate to be glossed.

## Incomplete Goals

- **To easily convert to and from romanized text.** The most prominent form of
  Ithkuil text is in its romanization system, and this crate aims to have
  first-class support for it with parsing and generation functions.

- **To convert into Ithkuil script.** Natively, Ithkuil is written using a
  morphophonemic script. This crate aims to be able to generate representations
  of Ithkuil text as script characters to allow for generic script conversions.
  It also aims to provide conversions into SVGs for all types using the glyphs
  provided by members of the Ithkuil Discord server.

- **To convert into Unicode representations.** "Generic script conversions"
  includes the two formats of writing Ithkuil text using Unicode: one developed
  by [@shankarsivarajan](https://github.com/shankarsivarajan), which primarily
  uses ASCII characters with diacritics for easy computer parsing, and one
  developed by [@zsakowitz](https://github.com/zsakowitz), which uses a section
  of the Unicode block designated for private use, to allow for interpolating
  Ithkuil text with other text.

- **To provide a list of roots and affixes.** Two main projects have been
  started to be computer-readable representations of the Ithkuil lexicon and
  affix list: the
  [Collaborative Ithkuil IV Roots and Affixes Spreadsheet](https://docs.google.com/spreadsheets/d/1JdaG1PaSQJRE2LpILvdzthbzz1k_a0VT86XSXouwGy8/edit#gid=378613623)
  and [lexicon-json](https://github.com/yuorb/lexicon-json). This crate aims to
  allow its users to choose which representation to use in glosses and aims to
  provide type definitions for each.

## Non-Goals

- **To preserve all data when parsing text.** `malëuţřait` and `malëuţřaita`
  should and will resolve to the same value; the decision to include a final
  vowel has no effect on the meaning and thus has no effect on the parsed value.
  Ca and Cn shortcuts are still represented accurately in the type system.

- **To be the definitive source of Ithkuil syntax.** If this crate's behavior
  differs from the official morphology document (currently v1.3.2) in any way,
  it is an issue in this crate, not the morphology document.

- **To achieve exact compatiability with @ırburučpaızya.** This crate has
  similar, but not identical behavior to
  [IthkuilGloss](https://github.com/ngoriyasjil/IthkuilGloss). This is
  intentional; they are not the same crate and will have different behavior.
  Note however that both adhere to the same general syntax ([] for referentials,
  . for segments in a slot, - for different slots, double apostrophes for roots
  and single apostrophes for affixes, and so on).
