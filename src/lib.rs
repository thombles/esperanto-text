/*!
Convert Esperanto text between UTF-8, x-system and h-system transliterations.

When correctly printed, Esperanto text has various diacritics that can be
properly represented in UTF-8. Those who are limited to ASCII or are unable
to type these characters often resort to the "h-system" or "x-system". In
these, a suffix is added to those letters which should have a diacritic.

This crate provides convenience functions for converting a string from one
transliteration to another. For the x-system this can be done with complete
accuracy as there is no ambiguity. For the h-system, a small vocabulary list
is used to avoid changing the meaning of real words.

A binary called `eotext` is included to use these functions from a CLI.

# Example: UTF-8 to x-system

```
let input = "eĥoŝanĝo ĉiuĵaŭde";
assert_eq!(
    esperanto_text::utf8_to_x_system(input),
    "ehxosxangxo cxiujxauxde".to_owned(),
);
```

# Example: h-system to UTF-8

```
let input = "Chiuj estas senchavaj kaj taugaj ideoj.";
assert_eq!(
    esperanto_text::h_system_to_utf8(input),
    "Ĉiuj estas senchavaj kaj taŭgaj ideoj.".to_owned(),
);
```

*/

use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};

/// Patterns to match for x-system input (case-insensitive)
const FROM_X_CI: &[&str] = &[
    "cx", "gx", "hx", "jx", "sx", "ux",
];

/// Patterns to match for UTF-8 input
///
/// Need to specify both cases as AhoCorasick's insensitive mode is ASCII-only.
const FROM_UTF8: &[&str] = &[
    "ĉ", "ĝ", "ĥ", "ĵ", "ŝ", "ŭ",
    "Ĉ", "Ĝ", "Ĥ", "Ĵ", "Ŝ", "Ŭ",
];

/// Patterns to match for h-system input (case-insensitive)
///
/// This includes all the transliterations but also a reasonably exhaustive
/// list of word fragments that need to be left alone, rather than blindly
/// substituting "something+h" with a diacritic. These longer segments will
/// be allowed to pass through unchanged.
const FROM_H_CI: &[&str] = &[
    // Uses of "h" to leave alone
    "komenchor", "kuracherb", "potenchav", "prononchelp", "senchav",
    /* (ŝ) */ "pruchelp", "drogherb", "flughaven", "longhar",
    /* (ŝ) */ "lesvigholstini", "vanghar", "gajhumor", "amashisteri",
    /* (aŭ) */ "tobushaltej", "bushaltej", /* (ĉ) */ "ashund", "dishak",
    "disharmoni", "dishelig", "dishirtig", "fikshejm", "grashav",
    "grashepata", "invershav", "kashal", "misharmoni", "mishelp",
    "mishumor", "neinvershav", "plushor", "sekshontem", "seshektar",
    "seshor", "sukceshav",

    // Uses of "au" (without circumflex) to leave alone
    "blankaurs", "doganauni", /* (eŭ) */ "ropauni", "grandaursin",
    "imaginaraunu", "kakauj", "malgrandaursin", "matricaunu",
    "naur", "praul", "saudaarabuj", "tiaul", "traurb", "unuaul",

    // Regular letters to transliterate
    "ch", "gh", "hh", "jh", "sh",

    // In most situations this is meant to become "aŭ"
    "au",
];

/// Convert UTF-8 "ĵaŭdo" to x-system "jxauxdo"
pub fn utf8_to_x_system(s: &str) -> String {
    let ac = AhoCorasick::new(FROM_UTF8);
    let mut result = String::new();
    ac.replace_all_with(s, &mut result, |m, found, dst| {
        let leading_capital = match dst.chars().rev().next() {
            Some(c) if c.is_uppercase() => false,
            Some(_) => true,
            None => true,
        };
        let (_, tail) = s.split_at(m.end());
        let capital_follows = match tail.chars().next() {
            Some(c) if c.is_uppercase() => true,
            Some(_) => false,
            None => false,
        };
        dst.push_str(match found {
            "ĉ" => "cx",
            "ĝ" => "gx",
            "ĥ" => "hx",
            "ĵ" => "jx",
            "ŝ" => "sx",
            "ŭ" => "ux",
            other => match (other, leading_capital && !capital_follows) {
                ("Ĉ", false) => "CX",
                ("Ĝ", false) => "GX",
                ("Ĥ", false) => "HX",
                ("Ĵ", false) => "JX",
                ("Ŝ", false) => "SX",
                ("Ŭ", false) => "UX",
                ("Ĉ", true) => "Cx",
                ("Ĝ", true) => "Gx",
                ("Ĥ", true) => "Hx",
                ("Ĵ", true) => "Jx",
                ("Ŝ", true) => "Sx",
                ("Ŭ", true) => "Ux",
                _ => other,
            }
        });
        true
    });
    result
}

/// Convert UTF-8 "ĵaŭdo" to h-system "jhaudo"
pub fn utf8_to_h_system(s: &str) -> String {
    let ac = AhoCorasick::new(FROM_UTF8);
    let mut result = String::new();
    ac.replace_all_with(s, &mut result, |m, found, dst| {
        let leading_capital = match dst.chars().rev().next() {
            Some(c) if c.is_uppercase() => false,
            Some(_) => true,
            None => true,
        };
        let (_, tail) = s.split_at(m.end());
        let capital_follows = match tail.chars().next() {
            Some(c) if c.is_uppercase() => true,
            Some(_) => false,
            None => false,
        };
        dst.push_str(match found {
            "ĉ" => "ch",
            "ĝ" => "gh",
            "ĥ" => "hh",
            "ĵ" => "jh",
            "ŝ" => "sh",
            "ŭ" => "u",
            other => match (other, leading_capital && !capital_follows) {
                ("Ĉ", false) => "CH",
                ("Ĝ", false) => "GH",
                ("Ĥ", false) => "HH",
                ("Ĵ", false) => "JH",
                ("Ŝ", false) => "SH",
                ("Ŭ", false) => "U",
                ("Ĉ", true) => "Ch",
                ("Ĝ", true) => "Gh",
                ("Ĥ", true) => "Hh",
                ("Ĵ", true) => "Jh",
                ("Ŝ", true) => "Sh",
                ("Ŭ", true) => "U",
                _ => other,
            }
        });
        true
    });
    result
}

/// Convert x-system "jxauxdo" to UTF-8 "ĵaŭdo"
pub fn x_system_to_utf8(s: &str) -> String {
    let ac = AhoCorasickBuilder::new()
        .ascii_case_insensitive(true)
        .build(FROM_X_CI);
    let mut result = String::new();
    ac.replace_all_with(s, &mut result, |_, found, dst| {
        dst.push_str(match found {
            "cx" => "ĉ",
            "gx" => "ĝ",
            "hx" => "ĥ",
            "jx" => "ĵ",
            "sx" => "ŝ",
            "ux" => "ŭ",
            "CX" | "Cx" | "cX" => "Ĉ",
            "GX" | "Gx" | "gX" => "Ĝ",
            "HX" | "Hx" | "hX" => "Ĥ",
            "JX" | "Jx" | "jX" => "Ĵ",
            "SX" | "Sx" | "sX" => "Ŝ",
            "UX" | "Ux" | "uX" => "Ŭ",
            _ => found,
        });
        true
    });
    result
}

/// Convert h-system "jhaudo" to UTF-8 "ĵaŭdo"
pub fn h_system_to_utf8(s: &str) -> String {
    let ac = AhoCorasickBuilder::new()
        .ascii_case_insensitive(true)
        .match_kind(MatchKind::LeftmostLongest)
        .build(FROM_H_CI);
    let mut result = String::new();
    ac.replace_all_with(s, &mut result, |_, found, dst| {
        dst.push_str(match found {
            "ch" => "ĉ",
            "gh" => "ĝ",
            "hh" => "ĥ",
            "jh" => "ĵ",
            "sh" => "ŝ",
            "au" => "aŭ",
            "CH" | "Ch" | "cH" => "Ĉ",
            "GH" | "Gh" | "gH" => "Ĝ",
            "HH" | "Hh" | "hH" => "Ĥ",
            "JH" | "Jh" | "jH" => "Ĵ",
            "SH" | "Sh" | "sH" => "Ŝ",
            "AU" => "AŬ",
            "Au" => "Aŭ",
            "aU" => "aŬ",
            // all the word fragments go through with existing casing
            // and without messing up the legitimate usage of "h"
            // or the legitimate usage of "au"
            _ => found,
        });
        true
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x_system_to_utf8_noop() {
        let input = "The quick brown fox jumps over the lazy dog. And my axe.".to_owned();
        assert_eq!(input, x_system_to_utf8(&input));
    }

    #[test]
    fn test_x_system_to_utf8_echo_change() {
        let input = "ehxosxangxo cxiujxauxde EHXOSXANGXO CXIUJXAUXDE";
        let expected = "eĥoŝanĝo ĉiuĵaŭde EĤOŜANĜO ĈIUĴAŬDE";
        assert_eq!(&x_system_to_utf8(input), expected);
    }

    #[test]
    fn test_x_system_to_utf8_mixed_case() {
        let input = "eHxoSxanGxo CxiuJxaUxde ehXosXangXo cXiujXauXde";
        let expected = "eĤoŜanĜo ĈiuĴaŬde eĤoŜanĜo ĈiuĴaŬde";
        assert_eq!(&x_system_to_utf8(input), expected);
    }

    #[test]
    fn test_utf8_to_x_system_noop() {
        let input = "The quick brown fox jumps over the lazy dog. And my axe.".to_owned();
        assert_eq!(input, utf8_to_x_system(&input));
    }

    #[test]
    fn test_utf8_to_x_system_echo_change() {
        let input = "eĥoŝanĝo ĉiuĵaŭde EĤOŜANĜO ĈIUĴAŬDE";
        let expected = "ehxosxangxo cxiujxauxde EHXOSXANGXO CXIUJXAUXDE";
        assert_eq!(&utf8_to_x_system(input), expected);
    }

    #[test]
    fn test_utf8_to_h_system_noop() {
        let input = "The quick brown fox jumps over the lazy dog. And my axe.".to_owned();
        assert_eq!(input, utf8_to_h_system(&input));
    }

    #[test]
    fn test_utf8_to_h_system_echo_change() {
        let input = "eĥoŝanĝo ĉiuĵaŭde EĤOŜANĜO ĈIUĴAŬDE";
        let expected = "ehhoshangho chiujhaude EHHOSHANGHO CHIUJHAUDE";
        assert_eq!(&utf8_to_h_system(input), expected);
    }

    #[test]
    fn test_h_system_to_utf8_noop() {
        let input = "The quick brown fox jumps over the lazy dog. And my axe.".to_owned();
        assert_eq!(input, h_system_to_utf8(&input));
    }

    #[test]
    fn test_h_system_to_utf8_echo_change() {
        let input = "ehhoshangho chiujhaude EHHOSHANGHO CHIUJHAUDE";
        let expected = "eĥoŝanĝo ĉiuĵaŭde EĤOŜANĜO ĈIUĴAŬDE";
        assert_eq!(&h_system_to_utf8(input), expected);
    }

    #[test]
    fn test_h_system_to_utf8_mixed_case() {
        let input = "eHhoShanGho ChiuJhAUde ehHosHangHo cHiujHaUde";
        let expected = "eĤoŜanĜo ĈiuĴAŬde eĤoŜanĜo ĈiuĴaŬde";
        assert_eq!(&h_system_to_utf8(input), expected);
    }

    #[test]
    fn test_h_system_ambiguous_h() {
        let input = "Chiuj estas senchavaj ideoj.";
        let expected = "Ĉiuj estas senchavaj ideoj.";
        assert_eq!(&h_system_to_utf8(input), expected);
    }

    #[test]
    fn test_h_system_ambiguous_u() {
        let input = "Hierau mi vizitis Nauron.";
        let expected = "Hieraŭ mi vizitis Nauron.";
        assert_eq!(&h_system_to_utf8(input), expected);
    }

    #[test]
    fn test_leading_capital_x_system() {
        let input = "Ĉiuj estas belaj. Ĥ Ŝ Ĝ Ĉ Ĵ Ŭ ĤO ŜO ĜO ĈO ĴO ŬO";
        let expected = "Cxiuj estas belaj. Hx Sx Gx Cx Jx Ux HXO SXO GXO CXO JXO UXO";
        assert_eq!(&utf8_to_x_system(input), expected);
    }

    #[test]
    fn test_leading_capital_h_system() {
        let input = "Ĉiuj estas belaj. Ĥ Ŝ Ĝ Ĉ Ĵ Ŭ ĤO ŜO ĜO ĈO ĴO ŬO";
        let expected = "Chiuj estas belaj. Hh Sh Gh Ch Jh U HHO SHO GHO CHO JHO UO";
        assert_eq!(&utf8_to_h_system(input), expected);
    }
}
