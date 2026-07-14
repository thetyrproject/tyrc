//! Tests for the Tyr parser.

use tyr_ast::{CompilationUnit, Item, Type};
use tyr_common::source::Source;
use tyr_lexer::{lexer::Lexer, literal::Literal};

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

//
// Signals
//

#[test]
fn parse_bit_signal() {
    let ast = parse_ok(
        r#"
module Main

signal clk : bit;

end
"#,
    );

    assert_eq!(ast.modules[0].items.len(), 1);
}

#[test]
fn parse_trit_signal() {
    let ast = parse_ok(
        r#"
module Main

signal data : trit;

end
"#,
    );

    assert_eq!(ast.modules[0].items.len(), 1);
}

#[test]
fn parse_multiple_signals() {
    let ast = parse_ok(
        r#"
module Main

signal clk : bit;
signal a : trit;
signal b : trit;

end
"#,
    );

    assert_eq!(ast.modules[0].items.len(), 3);
}

#[test]
fn signal_name_and_type() {
    let ast = parse_ok(
        r#"
module Main

signal clk : bit;

end
"#,
    );

    let module = &ast.modules[0];

    match &module.items[0] {
        Item::Signal(signal) => {
            assert_eq!(signal.name.name, "clk");
            assert_eq!(signal.ty, Type::Bit);
        }

        Item::Constant(_) => panic!("expected signal"),
    }
}

//
// Constants
//

#[test]
fn parse_bit_constant() {
    let ast = parse_ok(
        r#"
module Main

const WIDTH : bit = 1;

end
"#,
    );

    assert_eq!(ast.modules[0].items.len(), 1);
}

#[test]
fn parse_trit_constant() {
    let ast = parse_ok(
        r#"
module Main

const ZERO : trit = 0tb0;

end
"#,
    );

    assert_eq!(ast.modules[0].items.len(), 1);
}

#[test]
fn constant_name_type_and_value() {
    let ast = parse_ok(
        r#"
module Main

const WIDTH : bit = 1;

end
"#,
    );

    let module = &ast.modules[0];

    match &module.items[0] {
        Item::Constant(constant) => {
            assert_eq!(constant.name.name, "WIDTH");
            assert_eq!(constant.ty, Type::Bit);
            assert_eq!(constant.value, Literal::Integer("1".into()));
        }

        Item::Signal(_) => panic!("expected constant"),
    }
}
