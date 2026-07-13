//! Recursive-descent parser for the Tyr language.

use tyr_ast::{CompilationUnit, Module};

use tyr_common::span::Span;

use tyr_lexer::{keyword::Keyword, token::Token};

use crate::{cursor::Cursor, error::ParserResult};

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

        self.cursor.expect_keyword(Keyword::End)?;

        let end = self.cursor.previous().unwrap().span.end;

        Ok(Module::new(name, Vec::new(), Span::new(start, end)))
    }
}
