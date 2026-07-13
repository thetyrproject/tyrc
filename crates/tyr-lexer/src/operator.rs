//! Tyr language operators.

use std::fmt;

/// Operators recognized by the Tyr lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
    // Assignment
    Assign,

    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,

    // Comparison
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Logical
    LogicalAnd,
    LogicalOr,
    LogicalNot,

    // Bitwise
    BitAnd,
    BitOr,
    BitXor,
    BitNot,

    // Shift
    LeftShift,
    RightShift,

    // Range
    Range,
}

/// Lookup table.
///
/// IMPORTANT:
/// Longer operators MUST appear before shorter operators.
pub const OPERATORS: &[(&str, Operator)] = &[
    ("<-", Operator::Assign),
    ("==", Operator::Equal),
    ("!=", Operator::NotEqual),
    ("<=", Operator::LessEqual),
    (">=", Operator::GreaterEqual),
    ("<<", Operator::LeftShift),
    (">>", Operator::RightShift),
    ("&&", Operator::LogicalAnd),
    ("||", Operator::LogicalOr),
    ("..", Operator::Range),
    ("+", Operator::Add),
    ("-", Operator::Subtract),
    ("*", Operator::Multiply),
    ("/", Operator::Divide),
    ("%", Operator::Modulo),
    ("<", Operator::Less),
    (">", Operator::Greater),
    ("&", Operator::BitAnd),
    ("|", Operator::BitOr),
    ("^", Operator::BitXor),
    ("~", Operator::BitNot),
    ("!", Operator::LogicalNot),
];

impl Operator {
    /// Attempts to convert a string into an operator.
    #[must_use]
    pub fn lookup(op: &str) -> Option<Self> {
        OPERATORS
            .iter()
            .find(|(text, _)| *text == op)
            .map(|(_, operator)| *operator)
    }

    /// Returns the longest operator beginning at the given input.
    #[must_use]
    pub fn longest_match(input: &str) -> Option<(&'static str, Self)> {
        for &(text, operator) in OPERATORS {
            if input.starts_with(text) {
                return Some((text, operator));
            }
        }

        None
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = OPERATORS
            .iter()
            .find(|(_, operator)| operator == self)
            .map(|(text, _)| *text)
            .unwrap_or("<unknown>");

        write!(f, "{text}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operator_lookup() {
        assert_eq!(Operator::lookup("<-"), Some(Operator::Assign));
        assert_eq!(Operator::lookup("&&"), Some(Operator::LogicalAnd));
        assert_eq!(Operator::lookup(".."), Some(Operator::Range));
        assert_eq!(Operator::lookup("???"), None);
    }

    #[test]
    fn operator_display() {
        assert_eq!(Operator::Assign.to_string(), "<-");
        assert_eq!(Operator::LogicalAnd.to_string(), "&&");
        assert_eq!(Operator::Range.to_string(), "..");
    }
}
