//! Tyr lexical analyzer.

pub mod cursor;
pub mod error;
pub mod keyword;
pub mod lexer;
pub mod literal;
pub mod operator;
pub mod punctuation;
pub mod token;

#[cfg(test)]
mod tests;

pub use cursor::Cursor;
pub use error::LexerResult;
pub use keyword::Keyword;
pub use lexer::Lexer;
pub use literal::{Literal, TritValue};
pub use operator::Operator;
pub use punctuation::Punctuation;
pub use token::{Token, TokenKind};
