//! Token cursor used by the parser.

use tyr_common::{
    diagnostic::Diagnostic,
    span::Span,
};

use tyr_lexer::token::{Token, TokenKind};

/// Sequential cursor over a token stream.
pub struct Cursor<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl<'a> Cursor<'a> {
    /// Creates a new token cursor.
    #[must_use]
    pub const fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    /// Returns the current token index.
    #[must_use]
    pub const fn position(&self) -> usize {
        self.position
    }

    /// Returns true if the cursor reached EOF.
    #[must_use]
    pub fn is_eof(&self) -> bool {
        matches!(
            self.peek().map(|t| &t.kind),
            Some(TokenKind::Eof) | None
        )
    }

    /// Returns the current token.
    #[must_use]
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    /// Returns the token `n` positions ahead.
    ///
    /// `peek_n(0)` is equivalent to `peek()`.
    #[must_use]
    pub fn peek_n(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.position + n)
    }

    /// Consumes and returns the current token.
    pub fn advance(&mut self) -> Option<&'a Token> {
        let token = self.tokens.get(self.position)?;

        self.position += 1;

        Some(token)
    }

    /// Consumes the current token if it has the expected kind.
    pub fn match_kind(&mut self, kind: &TokenKind) -> bool {
        match self.peek() {
            Some(token) if token.kind.same_variant(kind) => {
                self.position += 1;
                true
            }

            _ => false,
        }
    }

    /// Consumes the current token if it has the expected kind.
    ///
    /// Returns an error otherwise.
    pub fn expect(
        &mut self,
        expected: &TokenKind,
    ) -> Result<&'a Token, Diagnostic> {
        match self.peek() {
            Some(token) if token.kind.same_variant(kind) => {
                Ok(self.advance().unwrap())
            }

            Some(token) => Err(Diagnostic::error(
                format!(
                    "expected {:?}, found {:?}",
                    expected,
                    token.kind
                ),
                token.span,
            )),

            None => Err(Diagnostic::error(
                format!("expected {:?}, found end of input", expected),
                Span::new(0, 0),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tyr_common::span::Span;
    use tyr_lexer::{
        keyword::Keyword,
        token::{Token, TokenKind},
    };

    fn make_tokens() -> Vec<Token> {
        vec![
            Token::new(
                TokenKind::Keyword(Keyword::Module),
                Span::new(0, 6),
            ),
            Token::new(
                TokenKind::Identifier("Main".into()),
                Span::new(7, 11),
            ),
            Token::new(
                TokenKind::Eof,
                Span::new(11, 11),
            ),
        ]
    }

    #[test]
    fn peek_returns_first_token() {
        let tokens = make_tokens();

        let cursor = Cursor::new(&tokens);

        assert!(matches!(
            cursor.peek().unwrap().kind,
            TokenKind::Keyword(Keyword::Module)
        ));
    }

    #[test]
    fn advance_moves_cursor() {
        let tokens = make_tokens();

        let mut cursor = Cursor::new(&tokens);

        cursor.advance();

        assert!(matches!(
            cursor.peek().unwrap().kind,
            TokenKind::Identifier(_)
        ));
    }

    #[test]
    fn match_kind_consumes_matching_token() {
        let tokens = make_tokens();

        let mut cursor = Cursor::new(&tokens);

        assert!(cursor.match_kind(
            &TokenKind::Keyword(Keyword::Module)
        ));

        assert_eq!(cursor.position(), 1);
    }

    #[test]
    fn expect_returns_token() {
        let tokens = make_tokens();

        let mut cursor = Cursor::new(&tokens);

        let token = cursor
            .expect(&TokenKind::Keyword(Keyword::Module))
            .unwrap();

        assert!(matches!(
            token.kind,
            TokenKind::Keyword(Keyword::Module)
        ));
    }

    #[test]
    fn eof_detected() {
        let tokens = make_tokens();

        let mut cursor = Cursor::new(&tokens);

        cursor.advance();
        cursor.advance();

        assert!(cursor.is_eof());
    }
}