//! AST register declarations.

use tyr_common::span::Span;

use crate::{identifier::Identifier, r#type::Type};

/// A register declaration.
///
/// Registers represent stateful storage elements within a design.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Register {
    /// Register name.
    pub name: Identifier,

    /// Register type.
    pub ty: Type,

    /// Source span.
    pub span: Span,
}

impl Register {
    /// Creates a new register declaration.
    #[must_use]
    pub fn new(name: Identifier, ty: Type, span: Span) -> Self {
        Self { name, ty, span }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_register() {
        let register = Register::new(
            Identifier::new("counter", Span::new(0, 7)),
            Type::Trit,
            Span::new(0, 24),
        );

        assert_eq!(register.name.name, "counter");
        assert_eq!(register.ty, Type::Trit);
    }
}
