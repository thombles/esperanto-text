//! Convert Esperanto text between UTF-8, x-system and h-system transliterations.
//!
//! When correctly printed, Esperanto text has various diacritics that can be
//! properly represented in UTF-8. Those who are limited to ASCII or are unable
//! to type these characters often resort to the "h-system" or "x-system". In
//! these, a suffix is added to those letters which should have a diacritic.
//!
//! This crate provides convenience functions for converting a string from one
//! transliteration to another. For the x-system this can be done with complete
//! accuracy as there is no ambiguity. For the h-system, a list of exceptions
//! is used to avoid replacing h suffixes that occur in normal Esperanto words.
//!
//! A binary called `eotext` is included to use these functions from a CLI.
//!
//! Example usage:
//!
//! TODO

use aho_corasick::AhoCorasick;

const FROM_X: &[&str] = &[
    "cx", "gx", "hx", "jx", "sx", "ux",
    "CX", "GX", "HX", "JX", "SX", "UX",
    "Cx", "Gx", "Hx", "Jx", "Sx", "Ux",
    "cX", "gX", "hX", "jX", "sX", "uX",
];
const TO_UTF8: &[&str] = &[
    "ĉ", "ĝ", "ĥ", "ĵ", "ŝ", "ŭ",
    "Ĉ", "Ĝ", "Ĥ", "Ĵ", "Ŝ", "Ŭ",
    "Ĉ", "Ĝ", "Ĥ", "Ĵ", "Ŝ", "Ŭ",
    "Ĉ", "Ĝ", "Ĥ", "Ĵ", "Ŝ", "Ŭ",
];
const FROM_UTF8: &[&str] = &[
    "ĉ", "ĝ", "ĥ", "ĵ", "ŝ", "ŭ",
    "Ĉ", "Ĝ", "Ĥ", "Ĵ", "Ŝ", "Ŭ",
];
const TO_X: &[&str] = &[
    "cx", "gx", "hx", "jx", "sx", "ux",
    "CX", "GX", "HX", "JX", "SX", "UX",
];

/// Convert UTF-8 "ĵaŭdo" to x-system "jxauxdo"
pub fn utf8_to_x_system(s: &str) -> String {
    do_replace(s, FROM_UTF8, TO_X)
}

/// Convert x-system "jxauxdo" to UTF-8 "ĵaŭdo"
pub fn x_system_to_utf8(s: &str) -> String {
    do_replace(s, FROM_X, TO_UTF8)
}

fn do_replace(haystack: &str, from: &[&str], to: &[&str]) -> String {
    let ac = AhoCorasick::new(from);
    let mut result = String::new();
    ac.replace_all_with(haystack, &mut result, |m, _, dst| {
        dst.push_str(to[m.pattern()]);
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
}
