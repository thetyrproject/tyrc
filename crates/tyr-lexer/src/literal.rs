//! Literal values.

/// Literal recognized by the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Integer(String),
    Float(String),

    Binary(String),
    Octal(String),
    Decimal(String),
    Hexadecimal(String),

    UnbalancedTernary(String),
    BalancedTernary(String),
    FractionalTernary(String),

    Bit(bool),

    Trit(TritValue),

    String(String),
}

/// Balanced ternary value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TritValue {
    Negative,
    Zero,
    Positive,
}

impl TritValue {
    #[must_use]
    pub const fn as_char(&self) -> char {
        match self {
            Self::Negative => 'n',
            Self::Zero => '0',
            Self::Positive => '1',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trit_values() {
        assert_eq!(TritValue::Negative.as_char(), 'n');
        assert_eq!(TritValue::Zero.as_char(), '0');
        assert_eq!(TritValue::Positive.as_char(), '1');
    }

    #[test]
    fn integer_literal() {
        let literal = Literal::Integer(String::from("42"));
        assert_eq!(literal, Literal::Integer(String::from("42")));
    }
}
