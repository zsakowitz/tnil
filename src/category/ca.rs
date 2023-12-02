use super::{Affiliation, Ca, Configuration, Essence, Extension, Perspective};
use crate::{ca, ca_pat};
use std::convert::identity;

/// Creates an ungeminated Ca string without allomorphic substitutions.
#[inline(always)]
fn ca_to_string(ca: Ca, geminate_fn: impl Fn(String) -> String) -> String {
    match ca {
        ca_pat!(ASO) => return "nļ".to_owned(),
        ca_pat!(COA) => return "rļ".to_owned(),
        ca_pat!(VAR) => return "ň".to_owned(),
        ca_pat!(M, NRM) => return "l".to_owned(),
        ca_pat!(M, RPV) => return "tļ".to_owned(),
        ca_pat!(N, NRM) => return "v".to_owned(),
        ca_pat!(A, NRM) => return "j".to_owned(),
        _ => {}
    };

    let affiliation = match ca.affiliation {
        Affiliation::CSL => "",
        Affiliation::ASO => "l",
        Affiliation::COA => "r",
        Affiliation::VAR => "ř",
    };

    let configuration = match ca.configuration {
        Configuration::UPX => "",
        Configuration::MSS => "t",
        Configuration::MSC => "k",
        Configuration::MSF => "p",
        Configuration::MDS => "ţ",
        Configuration::MDC => "f",
        Configuration::MDF => "ç",
        Configuration::MFS => "z",
        Configuration::MFC => "ž",
        Configuration::MFF => "ẓ",
        Configuration::DPX => "s",
        Configuration::DSS => "c",
        Configuration::DSC => "ks",
        Configuration::DSF => "ps",
        Configuration::DDS => "ţs",
        Configuration::DDC => "fs",
        Configuration::DDF => "š",
        Configuration::DFS => "č",
        Configuration::DFC => "kš",
        Configuration::DFF => "pš",
    };

    let extension = match ca.configuration {
        Configuration::UPX => match ca.extension {
            Extension::DEL => "",
            Extension::PRX => "d",
            Extension::ICP => "g",
            Extension::ATV => "b",
            Extension::GRA => "gz",
            Extension::DPL => "bz",
        },
        _ => match ca.extension {
            Extension::DEL => "",
            Extension::PRX => "t",
            Extension::ICP => "k",
            Extension::ATV => "p",
            Extension::GRA => "g",
            Extension::DPL => "b",
        },
    };

    let (without_tpk, with_tpk) = match (ca.perspective, ca.essence) {
        (Perspective::M, Essence::NRM) => ("", ""),
        (Perspective::G, Essence::NRM) => ("r", "r"),
        (Perspective::N, Essence::NRM) => ("w", "w"),
        (Perspective::A, Essence::NRM) => ("y", "y"),
        (Perspective::M, Essence::RPV) => ("l", "l"),
        (Perspective::G, Essence::RPV) => ("ř", "ř"),
        (Perspective::N, Essence::RPV) => ("m", "h"),
        (Perspective::A, Essence::RPV) => ("n", "ç"),
    };

    let mut output = String::with_capacity(
        affiliation.len() + configuration.len() + extension.len() + with_tpk.len(),
    );

    output += affiliation;
    output += configuration;
    output += extension;

    if output.ends_with(['t', 'p', 'k']) {
        output += with_tpk;
    } else {
        output += without_tpk;
    }

    geminate_fn(output)
}

impl Ca {
    /// Applies allomorphs to an ungeminated Ca string.
    pub fn allomorph(ca: &str) -> String {
        if ca.len() <= 1 {
            return ca.to_owned();
        }

        let ca = ca
            .replace("pp", "mp")
            .replace("tt", "nt")
            .replace("kk", "nk")
            .replace("ll", "pļ")
            .replace("pb", "mb")
            .replace("kg", "ng")
            .replace("çy", "nd")
            .replace("rr", "ns")
            .replace("rř", "nš")
            .replace("řr", "ňs")
            .replace("řř", "ňš")
            .replace("ngn", "ňn");

        let mut chars = ca.chars();

        let Some(first) = chars.next() else {
            return String::new();
        };

        let rest: String = chars.collect();

        let rest = rest
            .replace("gm", "x")
            .replace("gn", "ň")
            .replace("çx", "xw")
            .replace("bm", "v")
            .replace("bn", "ḑ");

        let mut output = first.to_string();
        output += &rest;

        output.replace("fv", "vw").replace("ţḑ", "ḑy")
    }

    /// Unapplies allomorphs on an ungeminated Ca string.
    pub fn unallomorph(ca: &str) -> String {
        if ca.len() <= 1 {
            return ca.to_owned();
        }

        let ca = ca.replace("ḑy", "ţḑ").replace("vw", "fv");

        let mut chars = ca.chars();

        let Some(first) = chars.next() else {
            return String::new();
        };

        let rest: String = chars.collect();

        let rest = rest
            .replace("ḑ", "bn")
            .replace("v", "bm")
            .replace("xw", "çx")
            .replace("ň", "gn")
            .replace("x", "gm");

        let mut output = first.to_string();
        output += &rest;

        output
            .replace("ňn", "ngn")
            .replace("gnn", "ngn")
            .replace("ňš", "řř")
            .replace("gnš", "řř")
            .replace("ňs", "řr")
            .replace("gns", "řr")
            .replace("nš", "rř")
            .replace("ns", "rr")
            .replace("nd", "çy")
            .replace("ng", "kg")
            .replace("mb", "pb")
            .replace("pļ", "ll")
            .replace("nk", "kk")
            .replace("nt", "tt")
            .replace("mp", "pp")
    }

    /// Geminates a Ca form, ignoring the final l/r/ř substitution. Returns [`None`] if it is not
    /// possible to geminate the input string.
    pub fn try_geminate_ignoring_lrř(ca: &str) -> Option<String> {
        // This algorithm converts the string slice into a `Vec<char>` to make it easier to work
        // with. It doesn't matter that much performance-wise because Ca string are never longer
        // than a few characters.

        match ca.len() {
            0 => return Some(String::new()),
            1 => {
                let mut output = ca.to_owned();
                output += &ca;
                return Some(output);
            }
            _ => {}
        }

        if ca == "tļ" {
            return Some("ttļ".to_owned());
        }

        let ca_chars: Vec<char> = ca.chars().collect();

        // Slicing is okay because we already checked that `len >= 2`.
        match ca_chars[0] {
            stop @ ('t' | 'k' | 'p' | 'd' | 'g' | 'b') => {
                // Slicing is okay because we already checked that `len >= 2`.
                match ca_chars[1] {
                    'l' | 'r' | 'ř' | 'w' | 'y' => {
                        let mut output = String::with_capacity(ca.len() + 1);
                        output.push(stop);
                        output.push(stop);
                        for char in &ca_chars[1..] {
                            output.push(*char);
                        }
                        return Some(output);
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        match ca_chars
            .iter()
            .enumerate()
            .find(|(_, el)| matches!(el, 's' | 'š' | 'z' | 'ž' | 'ç' | 'c' | 'č'))
        {
            Some((index, silibant)) => {
                let mut output = String::with_capacity(ca.len() + silibant.len_utf8());
                for char in &ca_chars[0..index] {
                    output.push(*char);
                }
                // Only push once because it's included in the iteration below.
                output.push(*silibant);
                for char in &ca_chars[index..] {
                    output.push(*char);
                }
                return Some(output);
            }
            _ => {}
        }

        match ca_chars[0] {
            char @ ('f' | 'ţ' | 'v' | 'ḑ' | 'm' | 'n' | 'ň') => {
                let mut output = String::with_capacity(ca.len() + char.len_utf8());
                output.push(char);
                output += &ca;
                return Some(output);
            }
            _ => {}
        }

        match ca_chars[0] {
            tkp @ ('t' | 'k' | 'p') => match ca_chars[1] {
                // The morphology doc also says we should check for `s` and `š`, but we already did
                // in the checker above.
                fricative @ ('f' | 'ţ' | 'ç') => {
                    let mut output = String::with_capacity(ca.len() + fricative.len_utf8());
                    output.push(tkp);
                    output.push(fricative);
                    // Indexing is safe because it starts at `1 + length of fricative`, which we
                    // know is a character boundary.
                    output += &ca[fricative.len_utf8() + 1..];
                    return Some(output);
                }
                _ => {}
            },
            _ => {}
        }

        macro_rules! substitute {
            (match ca { $($ungeminate:literal => $geminate:literal,)+ }) => {
                $(if ca.contains($ungeminate) {
                    return Some(ca.replace($ungeminate, $geminate));
                })+
            };
        }

        substitute!(match ca {
            "pt" => "bbḑ",
            "pk" => "bbv",
            "kt" => "ggḑ",
            "kp" => "ggv",
            "tk" => "ḑvv",
            "tp" => "ddv",
            "pm" => "vvm",
            "pm" => "vvn",
            "km" => "xxm",
            "kn" => "xxn",
            "tm" => "ḑḑm",
            "tn" => "ḑḑn",
            "bm" => "mmw",
            "bn" => "mml",
            "gm" => "ňňw",
            "gn" => "ňňl",
            "dm" => "nnw",
            "dn" => "nnl",
        });

        None
    }

    /// Geminates a Ca form (including the final l/r/ř substitution). Returns [`None`] if it is not
    /// possible to geminate the input string.
    pub fn try_geminate(ca: &str) -> Option<String> {
        let mut prefix = 'l';

        if ca.starts_with(|char| {
            if matches!(char, 'l' | 'r' | 'ř') {
                prefix = char;
                true
            } else {
                false
            }
        }) {
            match Ca::try_geminate_ignoring_lrř(&ca[prefix.len_utf8()..]) {
                Some(mut value) => {
                    value.insert(0, prefix);
                    Some(value)
                }
                None => {
                    let mut output = String::with_capacity(prefix.len_utf8() + ca.len());
                    output.push(prefix);
                    output += ca;
                    Some(output)
                }
            }
        } else {
            Ca::try_geminate_ignoring_lrř(ca)
        }
    }

    /// Geminates a Ca form. If it is not possible to geminate the input string, the first character
    /// is duplicated, regardless of phonotactic correctness.
    pub fn geminate(ca: &str) -> String {
        match Ca::try_geminate(ca) {
            Some(value) => value,
            None => {
                let first = ca
                    .chars()
                    .next()
                    .expect("`Ca::try_geminate` returns `Some` if input is zero characters long");

                let mut output = String::with_capacity(first.len_utf8() + ca.len());
                output.push(first);
                output += ca;
                output
            }
        }
    }

    /// Ungeminates a geminated Ca form, returning an [`Option`] indicating whether anything was
    /// removed.
    pub fn ungeminate(ca: &str) -> Option<String> {
        macro_rules! substitute {
            (match ca { $($geminate:literal => $ungeminate:literal,)+ }) => {
                $(if ca.contains($geminate) {
                    return Some(ca.replace($geminate, $ungeminate));
                })+
            };
        }

        substitute!(match ca {
            "bbḑ" => "pt",
            "bbv" => "pk",
            "ggḑ" => "kt",
            "ggv" => "kp",
            "ḑvv" => "tk",
            "ddv" => "tp",
            "vvm" => "pm",
            "vvn" => "pm",
            "xxm" => "km",
            "xxn" => "kn",
            "ḑḑm" => "tm",
            "ḑḑn" => "tn",
            "mmw" => "bm",
            "mml" => "bn",
            "ňňw" => "gm",
            "ňňl" => "gn",
            "nnw" => "dm",
            "nnl" => "dn",
        });

        let mut output = String::with_capacity(ca.len() + 4);
        let mut chars = ca.chars();
        let Some(mut last_char) = chars.next() else {
            return None;
        };

        while let Some(char) = chars.next() {
            output.push(last_char);

            if last_char == char {
                output.extend(chars);
                return Some(output);
            } else {
                last_char = char;
            }
        }

        output.push(last_char);
        Some(output)
    }

    /// Creates an ungeminated Ca string without allomorphic substitutions.
    pub fn to_unallomorphed_string(self) -> String {
        ca_to_string(self, identity)
    }

    /// Creates an ungeminated Ca string.
    pub fn to_ungeminated_string(self) -> String {
        ca_to_string(self, |output| Ca::allomorph(&output))
    }

    /// Creates a geminated Ca string.
    pub fn to_geminated_string(self) -> String {
        Ca::geminate(&self.to_ungeminated_string())
    }

    /// Creates a possibly-geminated Ca string.
    pub fn to_string(self, is_geminate: bool) -> String {
        if is_geminate {
            self.to_geminated_string()
        } else {
            self.to_ungeminated_string()
        }
    }

    /// Attempts to parse a [`Ca`] from an ungeminated Ca string without allomorphic substitutions.
    pub fn from_unallomorphed_string(ca: &str) -> Option<Ca> {
        match ca {
            "nļ" => return Some(ca!(ASO)),
            "rļ" => return Some(ca!(COA)),
            "ň" => return Some(ca!(VAR)),
            "l" => return Some(ca!(M, NRM)),
            "tļ" => return Some(ca!(M, RPV)),
            "v" => return Some(ca!(N, NRM)),
            "j" => return Some(ca!(A, NRM)),
            _ => {}
        }

        let mut chars: Vec<char> = ca.chars().collect();
        chars.reverse();

        let affiliation = if chars.len() <= 1 {
            Affiliation::CSL
        } else {
            match chars.pop() {
                None => return None,
                Some('l') => Affiliation::ASO,
                Some('r') => Affiliation::COA,
                Some('ř') => Affiliation::VAR,
                Some(char) => {
                    chars.push(char);
                    Affiliation::CSL
                }
            }
        };

        let configuration = match chars.pop() {
            None => Configuration::UPX,
            Some('s') => Configuration::DPX,
            Some('t') => Configuration::MSS,
            Some('c') => Configuration::DSS,
            Some('k') => match chars.pop() {
                None => Configuration::MSC,
                Some('s') => Configuration::DSC,
                Some('š') => Configuration::DFC,
                Some(char) => {
                    chars.push(char);
                    Configuration::MSC
                }
            },
            Some('p') => match chars.pop() {
                None => Configuration::MSF,
                Some('s') => Configuration::DSF,
                Some('š') => Configuration::DFF,
                Some(char) => {
                    chars.push(char);
                    Configuration::MSF
                }
            },
            Some('ţ') => match chars.pop() {
                None => Configuration::MDS,
                Some('s') => Configuration::DDS,
                Some(char) => {
                    chars.push(char);
                    Configuration::MDS
                }
            },
            Some('f') => match chars.pop() {
                None => Configuration::MDC,
                Some('s') => Configuration::DDC,
                Some(char) => {
                    chars.push(char);
                    Configuration::MDC
                }
            },
            Some('ç') => Configuration::MDF,
            Some('š') => Configuration::DDF,
            Some('z') => Configuration::MFS,
            Some('č') => Configuration::DFS,
            Some('ž') => Configuration::MFC,
            Some('ẓ') => Configuration::MFF,
            Some(char) => {
                chars.push(char);
                Configuration::UPX
            }
        };

        let extension = if matches!(configuration, Configuration::UPX) {
            match chars.pop() {
                None => Extension::DEL,
                Some('d') => Extension::PRX,
                Some('g') => match chars.pop() {
                    None => Extension::ICP,
                    Some('z') => Extension::GRA,
                    Some(char) => {
                        chars.push(char);
                        Extension::ICP
                    }
                },
                Some('b') => match chars.pop() {
                    None => Extension::ATV,
                    Some('z') => Extension::DPL,
                    Some(char) => {
                        chars.push(char);
                        Extension::ATV
                    }
                },
                Some(char) => {
                    chars.push(char);
                    Extension::DEL
                }
            }
        } else {
            match chars.pop() {
                None => Extension::DEL,
                Some('t') => Extension::PRX,
                Some('k') => Extension::ICP,
                Some('p') => Extension::ATV,
                Some('g') => Extension::GRA,
                Some('b') => Extension::DPL,
                Some(char) => {
                    chars.push(char);
                    Extension::DEL
                }
            }
        };

        let (perspective, essence) = match chars.pop() {
            None => (Perspective::M, Essence::NRM),
            Some('l') => (Perspective::M, Essence::RPV),
            Some('r') => (Perspective::G, Essence::NRM),
            Some('ř') => (Perspective::G, Essence::RPV),
            Some('w') => (Perspective::N, Essence::NRM),
            Some('m') => (Perspective::N, Essence::RPV),
            Some('h') => (Perspective::N, Essence::RPV),
            Some('y') => (Perspective::A, Essence::NRM),
            Some('n') => (Perspective::A, Essence::RPV),
            Some('ç') => (Perspective::A, Essence::RPV),
            Some(char) => {
                chars.push(char);
                (Perspective::M, Essence::NRM)
            }
        };

        if !chars.is_empty() {
            None
        } else {
            Some(Ca {
                affiliation,
                configuration,
                extension,
                perspective,
                essence,
            })
        }
    }

    /// Attempts to parse a [`Ca`] from an ungeminated Ca string.
    pub fn from_ungeminated_string(ca: &str) -> Option<Ca> {
        Ca::from_unallomorphed_string(&Ca::unallomorph(ca))
    }

    /// Attempts to parse a [`Ca`] from a geminated Ca string.
    pub fn from_geminated_string(ca: &str) -> Option<Ca> {
        Ca::from_ungeminated_string(&Ca::ungeminate(ca)?)
    }

    /// Attempts to parse a [`Ca`] from a possibly geminated Ca string.
    pub fn from_string(ca: &str) -> Option<Ca> {
        match Ca::ungeminate(ca) {
            Some(ca) => Ca::from_geminated_string(&ca),
            None => Ca::from_geminated_string(ca),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gloss::{Gloss, GlossFlags};

    #[test]
    fn check() {
        for affiliation in Affiliation::ALL_ITEMS {
            for configuration in Configuration::ALL_ITEMS {
                for extension in Extension::ALL_ITEMS {
                    for perspective in Perspective::ALL_ITEMS {
                        for essence in Essence::ALL_ITEMS {
                            let ca = Ca {
                                affiliation,
                                configuration,
                                extension,
                                perspective,
                                essence,
                            };

                            let ungeminate = ca.to_string(false);
                            let geminate = ca.to_string(true);
                            assert_eq!(
                                Some(ca),
                                Ca::from_string(&ungeminate),
                                "{} was {:?} (originally ungeminated)",
                                ca.gloss(GlossFlags::SHOW_DEFAULTS),
                                ungeminate,
                            );
                            assert_eq!(
                                Some(ca),
                                Ca::from_string(&geminate),
                                "{} was {:?} (ungeminated is {:?})",
                                ca.gloss(GlossFlags::SHOW_DEFAULTS),
                                geminate,
                                ungeminate,
                            );
                        }
                    }
                }
            }
        }
    }
}
