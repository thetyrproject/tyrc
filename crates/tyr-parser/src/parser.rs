//! Recursive-descent parser for the Tyr language.

use crate::{cursor::Cursor, error::ParserResult};
use tyr_common::{diagnostic::Diagnostic, span::Span};
use tyr_lexer::{keyword::Keyword, punctuation::Punctuation, token::Token, token::TokenKind};

use tyr_ast::{CompilationUnit, Item, Module, Signal, Type};

/// Recursive-descent parser.
///
/// The parser consumes a stream of lexical tokens and constructs
/// an Abstract Syntax Tree (AST).
pub struct Parser<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Parser<'a> {
    /// Creates a new parser.
    #[must_use]
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            cursor: Cursor::new(tokens),
        }
    }

    /// Parses the entire token stream into an AST.
    pub fn parse(&mut self) -> ParserResult<CompilationUnit> {
        self.parse_compilation_unit()
    }

    /// Parses a complete Tyr source file.
    fn parse_compilation_unit(&mut self) -> ParserResult<CompilationUnit> {
        let mut unit = CompilationUnit::new(Vec::new(), Span::new(0, 0));
        unit.add_module(self.parse_module()?);

        while !self.cursor.is_eof() {
            unit.add_module(self.parse_module()?);
        }

        if let Some(last) = unit.modules.last() {
            unit.span = Span::new(0, last.span.end);
        }

        Ok(unit)
    }

    /// Parses a module declaration.
    fn parse_module(&mut self) -> ParserResult<Module> {
        let start = self.cursor.expect_keyword(Keyword::Module)?.span.start;

        let name = self.cursor.parse_identifier()?;

        let mut items = Vec::new();

        while !self.cursor.is_eof() {
            if self.cursor.match_keyword(Keyword::End) {
                let end = self.cursor.previous().unwrap().span.end;

                return Ok(Module::new(name, items, Span::new(start, end)));
            }

            items.push(self.parse_item()?);
        }

        Err(Diagnostic::error(
            "expected 'end' before end of file",
            Span::new(start, start),
        ))
    }

    /// Parses a Tyr type.
    fn parse_type(&mut self) -> ParserResult<Type> {
        let token = self.cursor.advance().expect("parser advanced past EOF");

        match &token.kind {
            TokenKind::Keyword(Keyword::Bit) => Ok(Type::Bit),
            TokenKind::Keyword(Keyword::Trit) => Ok(Type::Trit),
            TokenKind::Keyword(Keyword::Clock) => Ok(Type::Clock),
            TokenKind::Keyword(Keyword::Event) => Ok(Type::Event),
            _ => Err(tyr_common::diagnostic::Diagnostic::error(
                "expected a type",
                token.span,
            )),
        }
    }

    /// Parses a signal declaration.
    fn parse_signal(&mut self) -> ParserResult<Signal> {
        let start = self.cursor.expect_keyword(Keyword::Signal)?.span.start;

        let name = self.cursor.parse_identifier()?;
        self.cursor.expect_punctuation(Punctuation::Colon)?;
        let ty = self.parse_type()?;
        self.cursor.expect_punctuation(Punctuation::Semicolon)?;
        let end = self.cursor.previous().unwrap().span.end;

        Ok(Signal::new(name, ty, Span::new(start, end)))
    }

    /// Parses a module item.
    fn parse_item(&mut self) -> ParserResult<Item> {
        match self.cursor.peek() {
            Some(token) => match &token.kind {
                TokenKind::Keyword(Keyword::Signal) => Ok(Item::Signal(self.parse_signal()?)),

                _ => Err(tyr_common::diagnostic::Diagnostic::error(
                    "expected a module item",
                    token.span,
                )),
            },

            None => Err(tyr_common::diagnostic::Diagnostic::error(
                "unexpected end of input",
                Span::new(0, 0),
            )),
        }
    }
}
