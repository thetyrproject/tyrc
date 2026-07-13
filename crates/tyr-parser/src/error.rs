//! Parser result types.

use tyr_common::diagnostic::Diagnostic;

/// Result type used throughout the parser.
pub type ParserResult<T> = Result<T, Diagnostic>;
