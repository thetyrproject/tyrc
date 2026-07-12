//! Compiler diagnostics.

use crate::span::Span;

/// Severity of a compiler diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Note,
    Help,
}

impl Severity {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Note => "note",
            Self::Help => "help",
        }
    }
}

/// A compiler diagnostic.
///
/// Diagnostics are generated throughout the compilation pipeline
/// and provide feedback to the user.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    pub span: Span,
}

impl Diagnostic {
    /// Creates a new diagnostic.
    #[must_use]
    pub fn new(severity: Severity, message: impl Into<String>, span: Span) -> Self {
        Self {
            severity,
            message: message.into(),
            span,
        }
    }

    /// Creates an error diagnostic.
    #[must_use]
    pub fn error(message: impl Into<String>, span: Span) -> Self {
        Self::new(Severity::Error, message, span)
    }

    /// Creates a warning diagnostic.
    #[must_use]
    pub fn warning(message: impl Into<String>, span: Span) -> Self {
        Self::new(Severity::Warning, message, span)
    }

    /// Creates a note diagnostic.
    #[must_use]
    pub fn note(message: impl Into<String>, span: Span) -> Self {
        Self::new(Severity::Note, message, span)
    }

    /// Creates a help diagnostic.
    #[must_use]
    pub fn help(message: impl Into<String>, span: Span) -> Self {
        Self::new(Severity::Help, message, span)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_error() {
        let diagnostic = Diagnostic::error("unexpected token", Span::new(0, 5));

        assert_eq!(diagnostic.severity, Severity::Error);
    }
}
