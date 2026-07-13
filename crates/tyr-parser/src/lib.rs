//! Recursive-descent parser for the Tyr language.
//!
//! The parser consumes the token stream produced by `tyr-lexer` and
//! constructs an Abstract Syntax Tree (AST) defined in `tyr-ast`.

pub mod cursor;
pub mod error;
pub mod parser;

pub use cursor::Cursor;
pub use error::ParserResult;
pub use parser::Parser;

#[cfg(test)]
mod tests;
