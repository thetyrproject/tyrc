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
    #[must_use]
    pub fn matches_expected(&self, other: &Self) -> bool {
        use TokenKind::*;

        match (self, other) {
            (Identifier(_), Identifier(_)) => true,
            (Literal(_), Literal(_)) => true,

            (Keyword(a), Keyword(b)) => a == b,
            (Operator(a), Operator(b)) => a == b,
            (Punctuation(a), Punctuation(b)) => a == b,

            (Directive(a), Directive(b)) => a == b,

            (Eof, Eof) => true,

            _ => false,
        }
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
