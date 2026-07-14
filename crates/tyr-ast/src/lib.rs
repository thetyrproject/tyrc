//! Abstract Syntax Tree (AST) for the Tyr language.
//!
//! This crate defines the immutable syntax tree produced by
//! `tyr-parser` and consumed by later compiler stages such as
//! semantic analysis and HIR lowering.

pub mod compilation_unit;
pub mod constant;
pub mod identifier;
pub mod item;
pub mod module;
pub mod signal;
pub mod r#type;

pub use compilation_unit::CompilationUnit;
pub use constant::Constant;
pub use identifier::Identifier;
pub use item::Item;
pub use module::Module;
pub use signal::Signal;
pub use r#type::Type;
