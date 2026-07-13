//! Tyr lexical analyzer.

use tyr_common::{diagnostic::Diagnostic, source::Source};

use crate::{
    cursor::Cursor,
    error::LexerResult,
    keyword::Keyword,
    literal::Literal,
    operator::Operator,
    punctuation::Punctuation,
    token::{Token, TokenKind},
};

/// The Tyr lexical analyzer.
///
/// The lexer converts a stream of UTF-8 source characters into a
/// sequence of lexical tokens.
pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer.
    #[must_use]
    pub fn new(source: &'a Source) -> Self {
        Self {
            cursor: Cursor::new(source),
        }
    }

    // ============================================================
    // Private helper functions
    // ============================================================

    // skip_whitespace()
    // skip_comment()
    // lex_identifier()
    // lex_number()
    // lex_string()
    // lex_operator()
    // lex_punctuation()
    // next_token()

    // ============================================================
    // Public API
    // ============================================================

    // tokenize()

    /// Consumes all consecutive whitespace characters.
    fn skip_whitespace(&mut self) {
        let _ = self.cursor.eat_while(char::is_whitespace);
    }

    /// Consumes a comment if one is present.
    ///
    /// Supports both single-line (`//`) and nested multi-line
    /// (`/* ... */`) comments.
    ///
    /// Returns `true` if a comment was consumed.
    fn skip_comment(&mut self) -> bool {
        // --------------------------------------------------------
        // Single-line comment
        // --------------------------------------------------------
        if self.cursor.starts_with("//") {
            self.cursor.advance();
            self.cursor.advance();

            let _ = self.cursor.eat_while(|c| c != '\n');

            return true;
        }

        // --------------------------------------------------------
        // Nested multi-line comment
        // --------------------------------------------------------
        if self.cursor.starts_with("/*") {
            let mut depth = 1;

            self.cursor.advance();
            self.cursor.advance();

            while !self.cursor.is_eof() {
                if self.cursor.starts_with("/*") {
                    depth += 1;

                    self.cursor.advance();
                    self.cursor.advance();

                    continue;
                }

                if self.cursor.starts_with("*/") {
                    depth -= 1;

                    self.cursor.advance();
                    self.cursor.advance();

                    if depth == 0 {
                        break;
                    }

                    continue;
                }

                self.cursor.advance();
            }

            return true;
        }

        false
    }

    /// Lexes an identifier or reserved keyword.
    fn lex_identifier(&mut self) -> Token {
        self.cursor.begin_token();

        let identifier = self
            .cursor
            .eat_while(|c| c.is_ascii_alphanumeric() || c == '_');

        let kind = if let Some(keyword) = Keyword::lookup(&identifier) {
            TokenKind::Keyword(keyword)
        } else {
            TokenKind::Identifier(identifier)
        };

        Token::new(kind, self.cursor.finish_token())
    }

    /// Lexes an integer or floating-point literal.
    fn lex_number(&mut self) -> Token {
        self.cursor.begin_token();

        let mut value = self.cursor.eat_while(|c| c.is_ascii_digit());

        if self.cursor.peek() == Some('.')
            && self.cursor.peek_next().is_some_and(|c| c.is_ascii_digit())
        {
            value.push('.');

            self.cursor.advance();

            value.push_str(&self.cursor.eat_while(|c| c.is_ascii_digit()));

            return Token::new(
                TokenKind::Literal(Literal::Float(value)),
                self.cursor.finish_token(),
            );
        }

        Token::new(
            TokenKind::Literal(Literal::Integer(value)),
            self.cursor.finish_token(),
        )
    }

    /// Lexes a string literal.
    fn lex_string(&mut self) -> LexerResult<Token> {
        self.cursor.begin_token();

        // Consume opening quote.
        self.cursor.advance();

        let value = self.cursor.eat_while(|c| c != '"');

        if !self.cursor.match_char('"') {
            return Err(Diagnostic::error(
                "unterminated string literal",
                self.cursor.finish_token(),
            ));
        }

        Ok(Token::new(
            TokenKind::Literal(Literal::String(value)),
            self.cursor.finish_token(),
        ))
    }

    fn lex_operator(&mut self) -> LexerResult<Token> {
        self.cursor.begin_token();

        if let Some((text, operator)) = Operator::longest_match(self.cursor.remaining()) {
            self.cursor.match_str(text);

            return Ok(Token::new(
                TokenKind::Operator(operator),
                self.cursor.finish_token(),
            ));
        }

        Err(Diagnostic::error(
            "invalid operator",
            self.cursor.finish_token(),
        ))
    }

    /// Lexes a punctuation symbol.
    fn lex_punctuation(&mut self) -> LexerResult<Token> {
        self.cursor.begin_token();

        if let Some((text, punctuation)) = Punctuation::longest_match(self.cursor.remaining()) {
            self.cursor.match_str(text);

            return Ok(Token::new(
                TokenKind::Punctuation(punctuation),
                self.cursor.finish_token(),
            ));
        }

        Err(Diagnostic::error(
            "invalid punctuation",
            self.cursor.finish_token(),
        ))
    }

    /// Returns the next token from the source.
    fn next_token(&mut self) -> LexerResult<Token> {
        loop {
            self.skip_whitespace();

            if !self.skip_comment() {
                break;
            }
        }

        if self.cursor.is_eof() {
            self.cursor.begin_token();

            return Ok(Token::new(TokenKind::Eof, self.cursor.finish_token()));
        }

        let ch = self.cursor.peek().unwrap();

        if ch.is_ascii_alphabetic() || ch == '_' {
            return Ok(self.lex_identifier());
        }

        if ch.is_ascii_digit() {
            return Ok(self.lex_number());
        }

        if ch == '"' {
            return self.lex_string();
        }

        if matches!(
            ch,
            '+' | '-' | '*' | '/' | '%' | '=' | '!' | '<' | '>' | '&' | '|' | '^' | '~' | '.'
        ) {
            return self.lex_operator();
        }

        if matches!(ch, '(' | ')' | '{' | '}' | '[' | ']' | ',' | ':' | ';') {
            return self.lex_punctuation();
        }

        self.cursor.begin_token();
        self.cursor.advance();

        Err(Diagnostic::error(
            format!("unexpected character '{ch}'"),
            self.cursor.finish_token(),
        ))
    }

    /// Lexes the entire source file.
    pub fn tokenize(&mut self) -> LexerResult<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;

            let eof = token.kind == TokenKind::Eof;

            tokens.push(token);

            if eof {
                break;
            }
        }

        Ok(tokens)
    }
}
