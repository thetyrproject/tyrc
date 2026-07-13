//! Token definitions for the Tyr lexer.

use tyr_common::span::Span;

use crate::{keyword::Keyword, literal::Literal, operator::Operator, punctuation::Punctuation};

/// A lexical token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// The token kind.
    pub kind: TokenKind,

    /// Location within the source file.
    pub span: Span,
}

impl Token {
    #[must_use]
    pub const fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// Every token recognized by the Tyr lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    /// Reserved language keyword.
    Keyword(Keyword),

    /// User-defined identifier.
    Identifier(String),

    /// Literal value.
    Literal(Literal),

    /// Language operator.
    Operator(Operator),

    /// Punctuation symbol.
    Punctuation(Punctuation),

    /// Compiler directive.
    Directive(String),

    /// End of input.
    Eof,
}

impl TokenKind {
    /// Returns true if both token kinds are the same variant,
    /// ignoring any associated data.
    #[must_use]
    pub fn same_variant(&self, other: &Self) -> bool {
        use TokenKind::*;

        matches!(
            (self, other),
            (Identifier(_), Identifier(_))
                | (Keyword(_), Keyword(_))
                | (Literal(_), Literal(_))
                | (Operator(_), Operator(_))
                | (Punctuation(_), Punctuation(_))
                | (Eof, Eof)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_eof_token() {
        let token = Token::new(TokenKind::Eof, Span::new(0, 0));

        assert_eq!(token.kind, TokenKind::Eof);
    }
}
