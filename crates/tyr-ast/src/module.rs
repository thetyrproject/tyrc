//! AST module node.

use tyr_common::span::Span;

use crate::{identifier::Identifier, item::Item};

/// A Tyr module.
///
/// Modules are the fundamental compilation units of a Tyr design.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    /// Module name.
    pub name: Identifier,

    /// Items declared inside the module.
    pub items: Vec<Item>,

    /// Source span of the entire module.
    pub span: Span,
}

impl Module {
    /// Creates a new module.
    #[must_use]
    pub fn new(name: Identifier, items: Vec<Item>, span: Span) -> Self {
        Self { name, items, span }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_module() {
        let module = Module::new(
            Identifier::new("Main", Span::new(7, 11)),
            Vec::new(),
            Span::new(0, 14),
        );

        assert_eq!(module.name.name, "Main");
        assert!(module.items.is_empty());
    }
}
