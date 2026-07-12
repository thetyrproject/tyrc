//! Source file representation.

use std::fs;
use std::io;
use std::path::Path;

/// Represents a Tyr source file.
#[derive(Debug, Clone)]
pub struct Source {
    /// File name or logical source name.
    pub name: String,

    /// UTF-8 source text.
    pub text: String,
}

impl Source {
    /// Creates a source from a string.
    #[must_use]
    pub fn from_string(name: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            text: text.into(),
        }
    }

    /// Loads a source file from disk.
    pub fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref();

        Ok(Self {
            name: path.display().to_string(),
            text: fs::read_to_string(path)?,
        })
    }

    /// Returns the source length in bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.text.len()
    }

    /// Returns true if the source is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_source() {
        let source = Source::from_string("test.tyr", "module Main\nend");

        assert_eq!(source.name, "test.tyr");
        assert!(!source.is_empty());
    }
}
