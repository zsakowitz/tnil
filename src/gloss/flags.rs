use crate::macros::bitflags;

/// A bit flag of options passed to glossing functions.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GlossFlags(u8);

impl GlossFlags {
    /// A [`GlossFlags`] instance with no flags active.
    pub const NONE: Self = Self(0);

    /// A [`GlossFlags`] instance with only the `long` flag enabled.
    ///
    /// If passed to `.gloss()`, objects will return their long representations. For example,
    /// glossing the THM case with this flag returns `"thematic"`, whereas without it returns
    /// `"THM"`.
    pub const LONG: Self = Self(1 << 0);

    /// A [`GlossFlags`] instance with only the `show_defaults` flag enabled.
    ///
    /// If passed to `.gloss()`, objects will include default values in their representations. For
    /// example, glossing the word `la` without this flag returns `"1m"`, whereas with it returns
    /// `"1m.NEU-THM\NRM"`.
    pub const SHOW_DEFAULTS: Self = Self(1 << 1);

    /// A [`GlossFlags`] instance with only the `format_markdown` flag enabled.
    ///
    /// If passed to `.gloss()`, this will format using the Markdown specification for basic
    /// formatting. Specifically, `**...**` will be used for bold text and `_..._` for italics.
    pub const FORMAT_MARKDOWN: Self = Self(1 << 2);

    /// A [`GlossFlags`] instance with only the `format_discord_markdown` flag enabled.
    ///
    /// If passed to `.gloss()`, this will format using the Markdown specification as used by
    /// Discord for basic formatting. Specifically, `**...**` will be used for bold text, `_..._`
    /// for italics, and `__...__` for underlines.
    ///
    /// If the `FORMAT_MARKDOWN` flag is not specified, this flag does nothing.
    pub const FORMAT_DISCORD_MARKDOWN: Self = Self(1 << 3);

    #[cfg(feature = "sheet-affix-data")]
    /// A [`GlossFlags`] instance with only the `use_sheet_affix_data` flag enabled.
    ///
    /// If passed to `.gloss()`, this will replace affixes with their values as specified in the collaborative Ithkuil IV Roots and Affixes spreadsheet, found at https://docs.google.com/spreadsheets/d/1JdaG1PaSQJRE2LpILvdzthbzz1k_a0VT86XSXouwGy8/edit.
    pub const USE_SHEET_AFFIX_DATA: Self = Self(1 << 4);

    #[cfg(feature = "sheet-root-data")]
    /// A [`GlossFlags`] instance with only the `use_sheet_root_data` flag enabled.
    ///
    /// If passed to `.gloss()`, this will replace roots with their values as specified in the collaborative Ithkuil IV Roots and Affixes spreadsheet, found at https://docs.google.com/spreadsheets/d/1JdaG1PaSQJRE2LpILvdzthbzz1k_a0VT86XSXouwGy8/edit.
    pub const USE_SHEET_ROOT_DATA: Self = Self(1 << 5);

    #[cfg(feature = "lexicon-json-root-data")]
    /// A [`GlossFlags`] instance with only the `use_lexicon_json_root_data` flag enabled.
    ///
    /// If passed to `.gloss()`, this will replace roots with their values as specified in the lexicon-json repository, found at https://github.com/yuorb/lexicon-json.
    pub const USE_ITHKUIL_JSON_ROOT_DATA: Self = Self(1 << 6);

    #[cfg(all(feature = "sheet-root-data", feature = "lexicon-json-root-data"))]
    /// A [`GlossFlags`] instance with both the `use_sheet_root_data` and
    /// `use_lexicon_json_root_data` flags enabled.
    ///
    /// If passed to `.gloss()`, this will replace roots with their values as specified in the lexicon-json repository, found at https://github.com/yuorb/lexicon-json.
    pub const USE_COMBINED_ROOT_DATA: Self = Self(1 << 5 | 1 << 6);
}

bitflags!(GlossFlags);
