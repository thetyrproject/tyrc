//! Source cursor used by the lexer.

use tyr_common::{source::Source, span::Span};

/// Sequential cursor over a source file.
pub struct Cursor<'a> {
    source: &'a Source,
    start: usize,
    position: usize,
}

impl<'a> Cursor<'a> {
    #[must_use]
    pub const fn new(source: &'a Source) -> Self {
        Self {
            source,
            start: 0,
            position: 0,
        }
    }

    /// Marks the beginning of the next token.
    pub fn begin_token(&mut self) {
        self.start = self.position;
    }

    /// Returns the span of the current token.
    #[must_use]
    pub const fn finish_token(&self) -> Span {
        Span::new(self.start, self.position)
    }

    /// Returns the current byte position.
    #[must_use]
    pub const fn position(&self) -> usize {
        self.position
    }

    /// Returns true if the cursor reached end of file.
    #[must_use]
    pub fn is_eof(&self) -> bool {
        self.position >= self.source.text.len()
    }

    /// Returns the character `n` positions ahead without consuming it.
    ///
    /// `peek_n(0)` is equivalent to `peek()`.
    #[must_use]
    pub fn peek_n(&self, n: usize) -> Option<char> {
        self.source.text[self.position..].chars().nth(n)
    }

    /// Returns the current character without consuming it.
    #[must_use]
    pub fn peek(&self) -> Option<char> {
        self.peek_n(0)
    }

    /// Returns the next character without consuming it.
    #[must_use]
    pub fn peek_next(&self) -> Option<char> {
        self.peek_n(1)
    }

    /// Consumes and returns the current character.
    pub fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.position += ch.len_utf8();
        Some(ch)
    }

    /// Consumes characters while the predicate returns true.
    ///
    /// Returns the consumed characters.
    #[must_use]
    pub fn eat_while<F>(&mut self, predicate: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let start = self.position;

        while let Some(ch) = self.peek() {
            if !predicate(ch) {
                break;
            }

            self.advance();
        }

        self.slice(start, self.position).to_owned()
    }

    /// Consumes the current character if it matches the expected value.
    ///
    /// Returns true if the character was consumed.
    pub fn match_char(&mut self, expected: char) -> bool {
        match self.peek() {
            Some(ch) if ch == expected => {
                self.advance();
                true
            }
            _ => false,
        }
    }

    /// Returns a slice of the underlying source text.
    ///
    /// The range is half-open: [start, end)
    #[must_use]
    pub fn slice(&self, start: usize, end: usize) -> &str {
        &self.source.text[start..end]
    }

    /// Returns the remaining unconsumed source.
    #[must_use]
    pub fn remaining(&self) -> &str {
        &self.source.text[self.position..]
    }

    /// Returns true if the remaining source begins with the given string.
    #[must_use]
    pub fn starts_with(&self, pattern: &str) -> bool {
        self.remaining().starts_with(pattern)
    }

    /// Consumes the given string if it matches the remaining source.
    ///
    /// Returns `true` if the pattern was matched and consumed.
    pub fn match_str(&mut self, pattern: &str) -> bool {
        if !self.starts_with(pattern) {
            return false;
        }

        for ch in pattern.chars() {
            self.position += ch.len_utf8();
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tyr_common::source::Source;

    #[test]
    fn cursor_walks_source() {
        let source = Source::from_string("test.tyr", "abc");

        let mut cursor = Cursor::new(&source);

        assert_eq!(cursor.peek(), Some('a'));

        assert_eq!(cursor.advance(), Some('a'));
        assert_eq!(cursor.advance(), Some('b'));
        assert_eq!(cursor.advance(), Some('c'));

        assert!(cursor.is_eof());
    }

    #[test]
    fn cursor_token_span() {
        let source = Source::from_string("test.tyr", "module");

        let mut cursor = Cursor::new(&source);

        cursor.begin_token();

        let text = cursor.eat_while(|c| c.is_alphabetic());

        let span = cursor.finish_token();

        assert_eq!(text, "module");
        assert_eq!(span.start, 0);
        assert_eq!(span.end, 6);
    }

    #[test]
    fn match_char_consumes() {
        let source = Source::from_string("test.tyr", "->");

        let mut cursor = Cursor::new(&source);

        assert!(cursor.match_char('-'));
        assert_eq!(cursor.peek(), Some('>'));
    }

    #[test]
    fn remaining_source() {
        let source = Source::from_string("test.tyr", "module Main");

        let mut cursor = Cursor::new(&source);

        cursor.advance();
        cursor.advance();

        assert_eq!(cursor.remaining(), "dule Main");
    }

    #[test]
    fn starts_with_pattern() {
        let source = Source::from_string("test.tyr", "#import math");

        let cursor = Cursor::new(&source);

        assert!(cursor.starts_with("#import"));
        assert!(cursor.starts_with("#"));
        assert!(!cursor.starts_with("#include"));
    }
}
