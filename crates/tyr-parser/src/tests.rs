//! Tests for the Tyr parser.

use tyr_ast::CompilationUnit;
use tyr_common::source::Source;
use tyr_lexer::lexer::Lexer;

use crate::{Parser, error::ParserResult};

/// Parses a Tyr source string.
///
/// Returns the parser result without unwrapping.
fn parse(source: &str) -> ParserResult<CompilationUnit> {
    let source = Source::from_string("test.tyr", source);

    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().expect("lexing failed");

    let mut parser = Parser::new(&tokens);

    parser.parse()
}

/// Parses a Tyr source string.
///
/// Panics if parsing fails.
///
/// This helper is intended for tests that expect valid input.
fn parse_ok(source: &str) -> CompilationUnit {
    parse(source).expect("parsing failed")
}

//
// Error cases
//

#[test]
fn empty_source_is_error() {
    assert!(parse("").is_err());
}

#[test]
fn missing_module_name() {
    assert!(
        parse(
            r#"
module
end
"#,
        )
        .is_err()
    );
}

#[test]
fn missing_end_keyword() {
    assert!(
        parse(
            r#"
module Main
"#,
        )
        .is_err()
    );
}

#[test]
fn unexpected_keyword() {
    assert!(
        parse(
            r#"
end
"#,
        )
        .is_err()
    );
}

//
// Successful parses
//

#[test]
fn parse_empty_module() {
    let ast = parse_ok(
        r#"
module Main
end
"#,
    );

    assert_eq!(ast.modules.len(), 1);

    let module = &ast.modules[0];

    assert_eq!(module.name.name, "Main");
    assert!(module.items.is_empty());
}

#[test]
fn parse_two_modules() {
    let ast = parse_ok(
        r#"
module A
end

module B
end
"#,
    );

    assert_eq!(ast.modules.len(), 2);

    assert_eq!(ast.modules[0].name.name, "A");
    assert_eq!(ast.modules[1].name.name, "B");
}
