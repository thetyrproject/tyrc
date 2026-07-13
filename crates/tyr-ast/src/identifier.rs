//! AST identifier node.

use tyr_common::span::Span;

/// An identifier in the Tyr AST.
///
/// Identifiers represent user-defined names such as modules,
/// signals, memories and registers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    /// Identifier text.
    pub name: String,

    /// Source span.
    pub span: Span,
}

impl Identifier {
    /// Creates a new identifier.
    #[must_use]
    pub fn new(name: impl Into<String>, span: Span) -> Self {
        Self {
            name: name.into(),
            span,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_identifier() {
        let id = Identifier::new("Main", Span::new(0, 4));

        assert_eq!(id.name, "Main");
        assert_eq!(id.span.start, 0);
        assert_eq!(id.span.end, 4);
    }
}
