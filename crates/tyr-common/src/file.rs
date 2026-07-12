//! Source file identifiers.

/// Uniquely identifies a source file during compilation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileId(pub usize);

impl FileId {
    /// Creates a new file identifier.
    #[must_use]
    pub const fn new(id: usize) -> Self {
        Self(id)
    }

    /// Returns the underlying numeric identifier.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_file_id() {
        let file = FileId::new(42);

        assert_eq!(file.index(), 42);
    }
}
