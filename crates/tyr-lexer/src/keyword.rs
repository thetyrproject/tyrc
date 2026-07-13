//! Tyr language keywords.

/// Reserved keywords.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keyword {
    // Modules
    Module,
    End,
    Use,

    // Interfaces
    Input,
    Output,
    Inout,

    // Objects
    Signal,
    Register,
    Memory,
    Const,

    // Types
    Bit,
    Trit,
    Bus,
    Array,
    Clock,
    Event,

    // Control Flow
    Flow,
    If,
    Else,
    Match,
    Default,
    Return,

    // Verification
    Assert,
    Assume,
    Cover,

    // Timing
    Wait,
    After,
    Delay,

    // Ternary Operators
    Tand,
    Tor,
    Txor,
    Tnot,
}

impl Keyword {
    /// Returns the textual representation.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Module => "module",
            Self::End => "end",
            Self::Use => "use",

            Self::Input => "input",
            Self::Output => "output",
            Self::Inout => "inout",

            Self::Signal => "signal",
            Self::Register => "register",
            Self::Memory => "memory",
            Self::Const => "const",

            Self::Bit => "bit",
            Self::Trit => "trit",
            Self::Bus => "bus",
            Self::Array => "array",
            Self::Clock => "clock",
            Self::Event => "event",

            Self::Flow => "flow",
            Self::If => "if",
            Self::Else => "else",
            Self::Match => "match",
            Self::Default => "default",
            Self::Return => "return",

            Self::Assert => "assert",
            Self::Assume => "assume",
            Self::Cover => "cover",

            Self::Wait => "wait",
            Self::After => "after",
            Self::Delay => "delay",

            Self::Tand => "tand",
            Self::Tor => "tor",
            Self::Txor => "txor",
            Self::Tnot => "tnot",
        }
    }

    /// Attempts to convert a string into a reserved keyword.
    #[must_use]
    pub fn lookup(value: &str) -> Option<Self> {
        match value {
            "module" => Some(Self::Module),
            "end" => Some(Self::End),
            "use" => Some(Self::Use),

            "input" => Some(Self::Input),
            "output" => Some(Self::Output),
            "inout" => Some(Self::Inout),

            "signal" => Some(Self::Signal),
            "register" => Some(Self::Register),
            "memory" => Some(Self::Memory),
            "const" => Some(Self::Const),

            "bit" => Some(Self::Bit),
            "trit" => Some(Self::Trit),
            "bus" => Some(Self::Bus),
            "array" => Some(Self::Array),
            "clock" => Some(Self::Clock),
            "event" => Some(Self::Event),

            "flow" => Some(Self::Flow),
            "if" => Some(Self::If),
            "else" => Some(Self::Else),
            "match" => Some(Self::Match),
            "default" => Some(Self::Default),
            "return" => Some(Self::Return),

            "assert" => Some(Self::Assert),
            "assume" => Some(Self::Assume),
            "cover" => Some(Self::Cover),

            "wait" => Some(Self::Wait),
            "after" => Some(Self::After),
            "delay" => Some(Self::Delay),

            "tand" => Some(Self::Tand),
            "tor" => Some(Self::Tor),
            "txor" => Some(Self::Txor),
            "tnot" => Some(Self::Tnot),

            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyword_text() {
        assert_eq!(Keyword::Module.as_str(), "module");
        assert_eq!(Keyword::Trit.as_str(), "trit");
    }

    #[test]
    fn keyword_lookup() {
        assert_eq!(Keyword::lookup("module"), Some(Keyword::Module));
        assert_eq!(Keyword::lookup("signal"), Some(Keyword::Signal));
        assert_eq!(Keyword::lookup("trit"), Some(Keyword::Trit));
        assert_eq!(Keyword::lookup("foobar"), None);
    }
}
