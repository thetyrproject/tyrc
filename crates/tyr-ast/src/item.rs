//! AST item nodes.

use crate::signal::Signal;

/// An item contained within a module.
///
/// Every declaration that may appear inside a module is represented
/// as an item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    /// Signal declaration.
    Signal(Signal),
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
}
