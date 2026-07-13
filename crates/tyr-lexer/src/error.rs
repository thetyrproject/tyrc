//! Common result types used by the lexer.

use tyr_common::error::Result;

/// Standard result type returned by lexer operations.
pub type LexerResult<T> = Result<T>;
