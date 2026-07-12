//! Source span definitions.

/// Represents a contiguous range within a source file.
///
/// The span is half-open:
/// [start, end)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    /// Byte offset of the first character.
    pub start: usize,

    /// Byte offset immediately after the last character.
    pub end: usize,
}

impl Span {
    /// Creates a new source span.
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Returns the span length in bytes.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns true if the span is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.start == self.end
    }

    #[must_use]
    pub const fn contains(&self, position: usize) -> bool {
        position >= self.start && position < self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_span() {
        let span = Span::new(5, 10);

        assert_eq!(span.start, 5);
        assert_eq!(span.end, 10);
        assert_eq!(span.len(), 5);
        assert!(!span.is_empty());
    }

    #[test]
    fn empty_span() {
        let span = Span::new(8, 8);

        assert!(span.is_empty());
        assert_eq!(span.len(), 0);
    }
}
