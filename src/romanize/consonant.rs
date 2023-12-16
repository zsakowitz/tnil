use super::token::OwnedConsonantForm;
use once_cell::sync::Lazy;
use regex::Regex; // TODO: implement this ourselves
use std::{
    borrow::{Borrow, BorrowMut},
    mem::transmute,
    ops::{Deref, DerefMut},
};

macro_rules! regex {
    ($($pat:expr,)+) => {
        Lazy::new(|| {
            Regex::new(
                [
                    $($pat),+
                ]
                .join("|")
                .as_str()
            )
            .expect("valid regex")
        })
    }
}

macro_rules! matches_any {
    ($x:expr, $($pat:expr,)+) => {
        {
            static CONJUNCT_LIST: Lazy<Regex> = regex!($($pat,)+);
            CONJUNCT_LIST.is_match($x)
        }
    }
}

/// A consonant form wrapping an internal `str`.
///
/// This type is unsized, and thus can only be used behind some kind of pointer.
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq)]
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

impl ToOwned for ConsonantForm {
    type Owned = OwnedConsonantForm;

    fn to_owned(&self) -> Self::Owned {
        OwnedConsonantForm(self.0.to_owned())
    }
}

impl Borrow<ConsonantForm> for OwnedConsonantForm {
    fn borrow(&self) -> &ConsonantForm {
        &self
    }
}

impl BorrowMut<ConsonantForm> for OwnedConsonantForm {
    fn borrow_mut(&mut self) -> &mut ConsonantForm {
        &mut *self
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
    /// # use tnil::romanize::token::ConsonantForm;
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

    /// Removes the geminates from this [`ConsonantForm`].
    pub fn remove_geminates(&self) -> String {
        let mut chars: Vec<_> = self.0.chars().collect();

        chars.insert(0, ' ');

        chars
            .windows(2)
            .filter(|window| window[0] != window[1])
            .map(|x| x[1])
            .collect()
    }

    /// Checks for basic validity of this [`ConsonantForm`].
    pub fn is_valid(&self) -> bool {
        !matches_any!(
            &self.0,
            // Each entry below is disallowed.

            // 2.1.  [any consonant]'
            ".'",
            // 2.2.  [td][szšžcżčjţḑ]
            "[td][szšžcżčjţḑ]",
            // 2.3.  [kg][xň]
            "[kg][xň]",
            // 2.4.  kg|gk|td|dt|pb|bp
            "kg|gk|td|dt|pb|bp",
            // 2.5.  fv|vf|ţḑ|ḑţ|cż|żc|čj|jč|čc|jc|čż|jż
            "fv|vf|ţḑ|ḑţ|cż|żc|čj|jč|čc|jc|čż|jż",
            // 2.6.  [šž][cż]
            "[šž][cż]",
            // 2.7.  sż
            "sż",
            // 2.8.  [szšž][szšž] (but geminates are okay)
            "s[zšž]|z[sšž]|š[szž]|ž[szš]",
            // 2.9.  [cżčj][szšž]
            "[cżčj][szšž]",
            // 2.10. [szšž]ç
            "[szšž]ç",
            //       ç[szšž]
            "ç[szšž]",
            //       [cżčj]ç
            "[cżčj]ç",
            //       ç[żj]
            "ç[żj]",
            //       ļç|çļ|hç|çh|xç
            "ļç|çļ|hç|çh|xç",
            // 2.11. n[cżčj]
            "n[cżčj]",
            // 2.12. m[pb][fvtdţḑ]
            "m[pb][fvtdţḑ]",
            // 2.13. n(k[sš]|g[zž])
            "n(?:k[sš]|g[zž])",
            // 2.14. n[pb]
            "n[pb]",
            // 2.15. n[fv]C (that is, n[fv] cannot be followed by a consonant)
            "n[fv].",
            // 2.16. ň[kgxy]
            "ň[kgxy]",
            // 2.17. x[szšžçgļňyhř]
            "x[szšžçgļňyhř]",
            // 2.18. [bdghç]ļ
            "[bdghç]ļ",
            //       ļ[szšžhç]
            "ļ[szšžhç]",
            // 2.19. [ļxç]h$ (that is, [ļxç]h cannot be at the end of a word)
            "[ļxç]h$",
            // 2.20. [rh]ř
            "[rh]ř",
            // 2.21. řr
            "řr",
            // 2.22. [wy]C (that is, [wy] cannot be followed by a consonant)
            "[wy].",
        )
    }

    /// Checks if a [`ConsonantForm`] is allowed in word-initial position.
    pub fn is_valid_word_initial(&self) -> bool {
        match self.0.chars().fold(0, |a, _| a + 1) {
            // 3.1. any single consonant (including ') except for ļ is allowed
            1 => self.0.chars().next().expect("at least one character") != 'ļ',

            // 3.2. the following bi-consonantal conjuncts are okay:
            2 => matches_any!(
                &self.0,
                // 3.2.0. [pbtdkg][rlřwy]
                "[pbtdkg][rlřwy]",
                //        [pbkg][szšž of same voicing]
                "[pk][sš]",
                "[bg][zž]",
                //        [pbtdkg][non-sibilant fricative of =voicing and ≠articulation] but not kļ
                "p[fţxhļ]",
                "b[vḑ]",
                "t[fxhļ]", // tţ is disallowed elsewhere
                "d[v]",    // dḑ is disallowed elsewhere
                "k[fţh]",
                "g[vḑ]",
                //        [kg][mn]
                "[kg][mn]",
                // 3.2.1. [szšž][any C of =voicing but not [szšžļh]]
                "[sš][ptkfţxcčç]",
                "[zž][bdgvḑżjmnňrlwyř]",
                //        [szšž][mnňlrwyřv]
                "[sšzž][mnňlrwyřv]",
                // 3.2.2. [cżčj][lrmnňwv]
                "[cżčj][lrmnňwv]",
                //        [cżčj][fricative of =voicing but not [szšžļ]]
                "[cč][fţxh]",
                "[żj][vḑ]",
                //        [cč][ptk]
                "[cč][ptk]",
                //        [żj][bdg]
                "[żj][bdg]",
                // 3.2.3. x[ptcčmnlrw]
                "x[ptcčmnlrw]",
                // 3.2.4. ç[ptcčkmnňlrřw]
                "ç[ptcčkmnňlrřw]",
                // 3.2.5. [fvţḑ][lrwyřmnň]
                "[fvţḑ][lrwyřmnň]",
                //        [fţ][ptkcč]
                "[fţ][ptkcč]",
                //        [vḑ][bdgżj]
                "[vḑ][bdgżj]",
                // 3.2.6. ļ[ptkcčmnňwy]
                "ļ[ptkcčmnňwy]",
                // 3.2.7. h[lrmnw]
                "h[lrmnw]",
                // 3.2.8. [mn][lrwyř]
                "[mn][lrwyř]",
                //        ň[lrw]
                "ň[lrw]",
                // 3.2.9. l[wy]
                "l[wy]",
            ),

            // 3.3.9. If it [a tri-consonantal conjunct] contains a geminate, see Section 6.
            // 3.4.3. If it [a tetra-consonantal conjunct] contains a geminate, see Section 6.
            3 | 4 if self.is_geminate() => {
                // TODO: Mirror section 6 more closely.
                ConsonantForm::new(&self.remove_geminates()).is_valid_word_initial()
            }

            // 3.3. the following tri-consonantal conjuncts are okay:
            3 => matches_any!(
                &self.0,
                // 3.3.0. [pbkg][szšž of =voicing][any consonant that can follow szšž as part of a bi-consonantal conjunct as detailed above]
                "[pk][sš][ptkfţxcčçmnňlrwyřv]",
                "[bg][zž][bdgḑżjmnňlrwyřv]",
                //        [pbtdkg][fricative of =voicing but not [szšžx]][[wy] that can follow the fricative as part of a bi-consonantal conjunct as detailed above]
                "[ptk][fţļ][wy]",
                "[ptk]hw",
                "[bdg][vḑ][wy]",
                // 3.3.1. [pbtdkg][lr][wy]
                "[pbtdkg][lr][wy]",
                // 3.3.2. [ptk]ç[mnň]
                "[ptk]ç[mnň]",
                // 3.3.3. [pk][fţ][wy]
                "[pk][fţ][wy]",
                //        [pt]ļ[wy]
                "[pt]ļ[wy]",
                // 3.3.4. [szšžç][pbtdkg of =voicing][wyřlr]
                "[sšç][ptk][wyřlr]",
                "[zž][bdg][wyřlr]",
                //        [szšžç][mnň][[wy] that can follow the nasal as part of a bi-consonantal conjunct as detailed above]
                "[szšžç][mn][wy]",
                "[szšžç]ňw",
                // 3.3.5. h([lrmn]w|[mn]y|ll|rr|mm|nn)
                "h(?:[lrmn]w|[mn]y|ll|rr|mm|nn)",
                // 3.3.6. [cżčj][pbtdkg of =voicing][lrwyř]
                "[cč][ptk][lrwyř]",
                "[żj][bdg][lrwyř]",
                //        [cżčj][mnň][wy that can follow a nasal as part of a bi-consonantal conjunct as detailed above]
                "[cżčj][mn][wy]",
                "[cżčj]ňw",
                // 3.3.7. [fţ]l[wy]
                "[fţ]l[wy]",
                // 3.3.8. x[pt][lrwy]
                "x[pt][lrwy]",
                //        x[mn][wy]
                "x[mn][wy]",
                //        x[cč]w
                "x[cč]w",
            ),

            // 3.4. the following tetra-consonantal conjuncts are okay:
            4 => matches_any!(
                &self.0,
                // 3.4.1. [tri-consonantal conjunct ending in a stop][lrwyř]
                "[pk][sš][ptkfţxcčçmnňlrwyřv][lrwyř]",
                "[bg][zž][bdgḑżjmnňlrwyřv][lrwyř]",
                // 3.4.2. [tri-consonantal conjunct matching [szšžçcżčj][pbtdkg]ly
                "[sšçcč][ptk]ly",
                "[zžżj][bdg]ly",
            ),

            _ => false,
        }
    }

    /// Checks if a [`ConsonantForm`] is allowed in word-final position.
    pub fn is_valid_word_final(&self) -> bool {
        match self.0.chars().fold(0, |a, _| a + 1) {
            // 4.1. any single consonant except [hwy'] is okay
            1 => !matches_any!(&self.0, "[hwy']",),

            // 4.2.12. any single geminated consonant is okay
            2 if self.is_geminate() => true,

            // 4.2. the following bi-consonantal conjuncts are okay:
            2 => matches_any!(
                &self.0,
                // 4.2.1.  [pbtdkg][fricative of =voicing]
                "[ptk][fţsšçxhļ]",
                "[bdg][fḑzž]",
                // 4.2.2.  [pbkg][td of =voicing]
                "[pk]t",
                "[bg]d",
                // 4.2.3.  [szšžç][pbtdkg of =voicing]
                "[sšç][ptk]",
                "[zž][bdg]",
                // 4.2.4.  [cżčj][tdkg of =voicing]
                "[cč][tk]",
                "[żj][dg]",
                // 4.2.5.  [fv][tdkgszšž of =voicing]
                "f[tksš]",
                "v[dgzž]",
                // 4.2.6.  [ţḑ][tdkg of =voicing]
                "ţ[tk]",
                "ḑ[dg]",
                // 4.2.7.  [ļx][ptk]
                "[ļx][ptk]",
                // 4.2.8.  [mn][any stop or fricative]
                "[mn][pbtdkgfvţḑszšžçxhļ]",
                // 4.2.9.  ň[[td] or any fricative but not [xļ]]
                "ň[tdfvţḑszšžçh]",
                // 4.2.10. r[any consonant except [hwy']]
                "r[^hwy']",
                // 4.2.11. l[any consonant except [wyrň']]
                "l[^wyrň']",
            ),

            // 4.3. the following tri-consonantal conjuncts are okay:
            3 => matches_any!(
                &self.0,
                // [rřl]p[tkfţxsšhļç]
                "[rřl]p[tkfţxsšhļç]",
                // [mň]p[hļç]
                "[mň]p[hļç]",
                // [sšç]p[fţsšļç]
                "[sšç]p[fţsšļç]",
                // ————————————————
                // [lrř]t[kfxhļç]
                "[lrř]t[kfxhļç]",
                // [n]t[kfxh]
                "[n]t[kfxh]",
                // [mňsšç]t[hļç]
                "[mňsšç]t[hļç]",
                // ————————————————
                // [lrř]k[tfţsšhç]
                "[lrř]k[tfţsšhç]",
                // [nfţļ]k[hç]
                "[nfţļ]k[hç]",
                // [m]k[fţhç]
                "[m]k[fţhç]",
                // [sšç]k[fţsšhç]
                "[sšç]k[fţsšhç]",
                // ————————————————
                // [rř]b[dgvḑzž]
                "[rř]b[dgvḑzž]",
                // [l]b[vḑzž]
                "[l]b[vḑzž]",
                // ————————————————
                // [rř]d[bgv]
                "[rř]d[bgv]",
                // ————————————————
                // [rř]g[bdvḑzž]
                "[rř]g[bdvḑzž]",
                // [l]g[vḑzž]
                "[l]g[vḑzž]",
                // ————————————————
                // [lrřmň]f[tkfsš]
                "[lrřmň]f[tkfsš]",
                // [ptk]f[kf]
                "[ptk]f[kf]",
                // [f]f[tksš]
                "[f]f[tksš]",
                // ————————————————
                // [pkrlřmnň]ţ[tkţ]
                "[pkrlřmnň]ţ[tkţ]",
                // [ţ]ţ[tk]
                "[ţ]ţ[tk]",
                // ————————————————
                // [rlř]x[tx]
                "[rlř]x[tx]",
                // [ptfsšnm]x[x]
                "[ptfsšnm]x[x]",
                // [x]x[t]
                "[x]x[t]",
                // ————————————————
                // [ptrřmnň]ļ[tkļ]
                "[ptrřmnň]ļ[tkļ]",
                // [ļl]ļ[tk]
                "[ļl]ļ[tk]",
                // ————————————————
                // [rlřmnňpkf]s[ptkfţxs]
                "[rlřmnňpkf]s[ptkfţxs]",
                // [ţ]s[ptks]
                "[ţ]s[ptks]",
                // [s]s[ptkfţx]
                "[s]s[ptkfţx]",
                // ————————————————
                // [rlřmnňpkf]š[ptkfţxš]
                "[rlřmnňpkf]š[ptkfţxš]",
                // [ţ]š[ptkš]
                "[ţ]š[ptkš]",
                // [š]š[ptkfţx]
                "[š]š[ptkfţx]",
                // ————————————————
                // [rřl]v[vzž]
                "[rřl]v[vzž]",
                // [bgmň]v[v]
                "[bgmň]v[v]",
                // [v]v[zž]
                "[v]v[zž]",
                // ————————————————
                // [bgrřlnmň]ḑ[ḑ]
                "[bgrřlnmň]ḑ[ḑ]",
                // ————————————————
                // [rřlnmň]z[bdgz]
                "[rřlnmň]z[bdgz]",
                // [bgv]z[z]
                "[bgv]z[z]",
                // [z]z[bdg]
                "[z]z[bdg]",
                // ————————————————
                // [rřlnmň]ž[bdgž]
                "[rřlnmň]ž[bdgž]",
                // [bgv]ž[ž]
                "[bgv]ž[ž]",
                // [ž]ž[bdg]
                "[ž]ž[bdg]",
                // ————————————————
                // [rřl]c[tkch]
                "[rřl]c[tkch]",
                // ————————————————
                // [rřl]č[tkčh]
                "[rřl]č[tkčh]",
                // ————————————————
                // [rřl]ż[dgż]
                "[rřl]ż[dgż]",
                // ————————————————
                // [rřl]j[dgj]
                "[rřl]j[dgj]",
                // ————————————————
                // [rlř]m[ptkbdfţxsšvḑzžmļç]
                "[rlř]m[ptkbdfţxsšvḑzžmļç]",
                // [m]m[ptkbdfţxsšvḑzžļç]
                "[m]m[ptkbdfţxsšvḑzžļç]",
                // ————————————————
                // [rř]n[tkdgfţxsšvḑzžnļç]
                "[rř]n[tkdgfţxsšvḑzžnļç]",
                // [l]n[tkdgţsšzžļç]
                "[l]n[tkdgţsšzžļç]",
                // [n]n[tkdgfţxsšvḑzžļç]
                "[n]n[tkdgfţxsšvḑzžļç]",
                // ————————————————
                // [rř]ň[tdfţsšvḑzžňç]
                "[rř]ň[tdfţsšvḑzžňç]",
                // [l]ň[ň]
                "[l]ň[ň]",
                // [ň]ň[tdfţsšvḑzžç]
                "[ň]ň[tdfţsšvḑzžç]",
                // ————————————————
                // [l]l[pbtdkgfţxsšvḑzžcčżjmnňç]
                "[l]l[pbtdkgfţxsšvḑzžcčżjmnňç]",
                // ————————————————
                // [r]r[pbtdkgfţxsšvḑzžcčżjmnňlļç]
                "[r]r[pbtdkgfţxsšvḑzžcčżjmnňlļç]",
                // ————————————————
                // [ř]ř[pbtdkgfţxsšvḑzžcčżjmnňlļç]
                "[ř]ř[pbtdkgfţxsšvḑzžcčżjmnňlļç]",
                // ————————————————
                // [ptkmnňrlř]ç[tkç]
                "[ptkmnňrlř]ç[tkç]",
                // [ç]ç[tk]
                "[ç]ç[tk]",
            ),

            // 4.4. the following tetra-consonantal conjuncts are okay:
            4 => matches_any!(
                &self.0,
                //      [lrř][kp][sšţç][tk]
                "[lrř][kp][sšţç][tk]",
                //      [lrř]tç[tk]
                "[lrř]tç[tk]",
                //      [lrř]pf[tk]
                "[lrř]pf[tk]",
                //      [lrř]f[sš][tk]
                "[lrř]f[sš][tk]",
                //      r[nňm][sšţç][tk]
                "r[nňm][sšţç][tk]",
                //      r[ňm]f[tk]
                "r[ňm]f[tk]",
            ),

            // 4.5. penta-consonantal conjuncts are only okay if they're geminated version of the tetra-consonantal conjuncts above
            5 => ConsonantForm::new(&self.remove_geminates()).is_valid_word_final(),

            _ => false,
        }
    }
}
