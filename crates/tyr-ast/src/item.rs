//! AST item nodes.

use crate::{constant::Constant, register::Register, signal::Signal};

/// An item contained within a module.
///
/// Every declaration that may appear inside a module is represented
/// as an item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    /// Signal declaration.
    Signal(Signal),

    /// Constant declaration.
    Constant(Constant),

    /// Register declaration.
    Register(Register),
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{identifier::Identifier, signal::Signal, r#type::Type};

    use tyr_common::span::Span;

    #[test]
    fn create_signal_item() {
        let signal = Signal::new(
            Identifier::new("clk", Span::new(0, 3)),
            Type::Bit,
            Span::new(0, 16),
        );

        let item = Item::Signal(signal.clone());

        assert_eq!(item, Item::Signal(signal));
    }

    #[test]
    fn create_constant_item() {
        use crate::{constant::Constant, identifier::Identifier, r#type::Type};

        use tyr_common::span::Span;
        use tyr_lexer::literal::Literal;

        let constant = Constant::new(
            Identifier::new("WIDTH", Span::new(0, 5)),
            Type::Bit,
            Literal::Integer("1".into()),
            Span::new(0, 20),
        );

        let item = Item::Constant(constant.clone());

        assert_eq!(item, Item::Constant(constant));
    }

    #[test]
    fn create_register_item() {
        use crate::{identifier::Identifier, register::Register, r#type::Type};

        use tyr_common::span::Span;

        let register = Register::new(
            Identifier::new("counter", Span::new(0, 7)),
            Type::Trit,
            Span::new(0, 24),
        );

        let item = Item::Register(register.clone());

        assert_eq!(item, Item::Register(register));
    }
}
