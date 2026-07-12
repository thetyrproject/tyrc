//! Common compiler result types.

use crate::diagnostic::Diagnostic;

/// Standard compiler result type.
pub type Result<T> = std::result::Result<T, Diagnostic>;
