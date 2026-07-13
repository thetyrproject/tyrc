//! Tyr punctuation symbols.

use std::fmt;

/// Punctuation recognized by the lexer.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Punctuation {
    LeftParen,
    RightParen,

    LeftBrace,
    RightBrace,

    LeftBracket,
    RightBracket,

    Comma,
    Colon,
    Semicolon,

    Dot,
}

/// Lookup table.
const PUNCTUATIONS: &[(&str, Punctuation)] = &[
    ("(", Punctuation::LeftParen),
    (")", Punctuation::RightParen),
    ("{", Punctuation::LeftBrace),
    ("}", Punctuation::RightBrace),
    ("[", Punctuation::LeftBracket),
    ("]", Punctuation::RightBracket),
    (",", Punctuation::Comma),
    (":", Punctuation::Colon),
    (";", Punctuation::Semicolon),
    (".", Punctuation::Dot),
];

impl Punctuation {
    /// Attempts to convert a string into punctuation.
    #[must_use]
    pub fn lookup(text: &str) -> Option<Self> {
        for &(symbol, punctuation) in PUNCTUATIONS {
            if symbol == text {
                return Some(punctuation);
            }
        }

        None
    }

    /// Returns the longest punctuation beginning at the given input.
    #[must_use]
    pub fn longest_match(input: &str) -> Option<(&'static str, Self)> {
        for &(symbol, punctuation) in PUNCTUATIONS {
            if input.starts_with(symbol) {
                return Some((symbol, punctuation));
            }
        }

        None
    }
}

impl fmt::Display for Punctuation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = PUNCTUATIONS
            .iter()
            .find(|(_, punctuation)| punctuation == self)
            .map(|(text, _)| *text)
            .unwrap_or("<unknown>");

        write!(f, "{text}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn punctuation_lookup() {
        assert_eq!(Punctuation::lookup("("), Some(Punctuation::LeftParen));
        assert_eq!(Punctuation::lookup(";"), Some(Punctuation::Semicolon));
        assert_eq!(Punctuation::lookup("."), Some(Punctuation::Dot));
        assert_eq!(Punctuation::lookup("@"), None);
    }

    #[test]
    fn punctuation_display() {
        assert_eq!(Punctuation::LeftBrace.to_string(), "{");
        assert_eq!(Punctuation::Semicolon.to_string(), ";");
        assert_eq!(Punctuation::Dot.to_string(), ".");
    }
}
