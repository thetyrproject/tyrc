//! AST constant declarations.

use tyr_common::span::Span;

use tyr_lexer::literal::Literal;

use crate::{identifier::Identifier, r#type::Type};

/// A constant declaration.
///
/// Constants represent immutable compile-time values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constant {
    /// Constant name.
    pub name: Identifier,

    /// Constant type.
    pub ty: Type,

    /// Constant value.
    pub value: Literal,

    /// Source span.
    pub span: Span,
}

impl Constant {
    /// Creates a new constant declaration.
    #[must_use]
    pub fn new(name: Identifier, ty: Type, value: Literal, span: Span) -> Self {
        Self {
            name,
            ty,
            value,
            span,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_constant() {
        let constant = Constant::new(
            Identifier::new("WIDTH", Span::new(0, 5)),
            Type::Bit,
            Literal::Integer("1".into()),
            Span::new(0, 20),
        );

        assert_eq!(constant.name.name, "WIDTH");
        assert_eq!(constant.ty, Type::Bit);
        assert_eq!(constant.value, Literal::Integer("1".into()));
    }
}
