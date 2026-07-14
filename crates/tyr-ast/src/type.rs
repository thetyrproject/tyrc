//! Tyr type system.

use std::fmt;

/// Primitive and composite Tyr data types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    /// Binary logic value.
    Bit,

    /// Balanced ternary logic value.
    Trit,

    /// Clock signal.
    Clock,

    /// Event signal.
    Event,

    /// Fixed-width bus.
    Bus(Box<Type>, usize),

    /// Fixed-size array.
    Array(Box<Type>, usize),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bit => write!(f, "bit"),
            Self::Trit => write!(f, "trit"),
            Self::Clock => write!(f, "clock"),
            Self::Event => write!(f, "event"),

            Self::Bus(ty, width) => {
                write!(f, "bus<{}, {}>", ty, width)
            }

            Self::Array(ty, size) => {
                write!(f, "array<{}, {}>", ty, size)
            }
        }
    }
}

impl Type {
    #[must_use]
    pub const fn is_primitive(&self) -> bool {
        matches!(self, Self::Bit | Self::Trit | Self::Clock | Self::Event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_types() {
        assert_eq!(Type::Bit.to_string(), "bit");
        assert_eq!(Type::Trit.to_string(), "trit");
        assert_eq!(Type::Clock.to_string(), "clock");
    }

    #[test]
    fn bus_type() {
        let bus = Type::Bus(Box::new(Type::Bit), 8);

        assert_eq!(bus.to_string(), "bus<bit, 8>");
    }
}
