//! AST compilation unit.

use tyr_common::span::Span;

use crate::module::Module;

/// The root node of a Tyr source file.
///
/// Every parsed `.tyr` source file is represented as a single
/// compilation unit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilationUnit {
    /// Modules contained in this source file.
    pub modules: Vec<Module>,

    /// Source span of the entire file.
    pub span: Span,
}

impl CompilationUnit {
    /// Creates a new compilation unit.
    #[must_use]
    pub fn new(modules: Vec<Module>, span: Span) -> Self {
        Self { modules, span }
    }

    /// Returns true if the compilation unit contains no modules.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.modules.is_empty()
    }

    /// Adds a module to the compilation unit.
    pub fn add_module(&mut self, module: Module) {
        self.modules.push(module);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_compilation_unit() {
        let unit = CompilationUnit::new(Vec::new(), Span::new(0, 0));

        assert!(unit.is_empty());
    }

    #[test]
    fn compilation_unit_span() {
        let unit = CompilationUnit::new(Vec::new(), Span::new(0, 128));

        assert_eq!(unit.span.start, 0);
        assert_eq!(unit.span.end, 128);
    }
}
