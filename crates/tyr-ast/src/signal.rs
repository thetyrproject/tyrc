//! AST signal declaration.

use tyr_common::span::Span;

use crate::{identifier::Identifier, r#type::Type};

/// A signal declaration.
///
/// Signals represent named hardware wires or storage elements
/// depending on their usage within the design.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signal {
    /// Signal name.
    pub name: Identifier,

    /// Signal type.
    pub ty: Type,

    /// Source span.
    pub span: Span,
}

impl Signal {
    /// Creates a new signal declaration.
    #[must_use]
    pub fn new(name: Identifier, ty: Type, span: Span) -> Self {
        Self { name, ty, span }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_bit_signal() {
        let signal = Signal::new(
            Identifier::new("clk", Span::new(0, 3)),
            Type::Bit,
            Span::new(0, 16),
        );

        assert_eq!(signal.name.name, "clk");
        assert_eq!(signal.ty, Type::Bit);
    }

    #[test]
    fn create_trit_signal() {
        let signal = Signal::new(
            Identifier::new("data", Span::new(0, 4)),
            Type::Trit,
            Span::new(0, 18),
        );

        assert_eq!(signal.name.name, "data");
        assert_eq!(signal.ty, Type::Trit);
    }
}
