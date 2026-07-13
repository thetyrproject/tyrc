//! AST item nodes.

/// An item contained within a module.
///
/// Every declaration that may appear inside a module is represented
/// as an item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    /// Placeholder until real items are implemented.
    Empty,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_item() {
        let item = Item::Empty;

        assert_eq!(item, Item::Empty);
    }
}
