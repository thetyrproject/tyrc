//! Tests for the Tyr lexer.

use crate::{
    keyword::Keyword,
    lexer::Lexer,
    literal::Literal,
    punctuation::Punctuation,
    token::{Token, TokenKind},
};

use tyr_common::source::Source;

/// Lexes a source string.
///
/// Panics if lexing fails.
fn lex(source: &str) -> Vec<Token> {
    let source = Source::from_string("test.tyr", source);

    let mut lexer = Lexer::new(&source);

    lexer.tokenize().expect("lexing failed")
}

//
// Empty source
//

#[test]
fn empty_source() {
    let tokens = lex("");

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Eof);
}

//
// Keywords
//

#[test]
fn keyword_module() {
    let tokens = lex("module");

    assert_eq!(tokens[0].kind, TokenKind::Keyword(Keyword::Module));
}

#[test]
fn keyword_signal() {
    let tokens = lex("signal");

    assert_eq!(tokens[0].kind, TokenKind::Keyword(Keyword::Signal));
}

//
// Identifiers
//

#[test]
fn identifier() {
    let tokens = lex("counter");

    assert_eq!(tokens[0].kind, TokenKind::Identifier("counter".into()));
}

#[test]
fn identifier_with_underscore() {
    let tokens = lex("_signal_1");

    assert_eq!(tokens[0].kind, TokenKind::Identifier("_signal_1".into()));
}

//
// Integer literals
//

#[test]
fn integer_literal() {
    let tokens = lex("12345");

    assert_eq!(
        tokens[0].kind,
        TokenKind::Literal(Literal::Integer("12345".into()))
    );
}

//
// Float literals
//

#[test]
fn float_literal() {
    let tokens = lex("3.14159");

    assert_eq!(
        tokens[0].kind,
        TokenKind::Literal(Literal::Float("3.14159".into()))
    );
}

//
// String literals
//

#[test]
fn string_literal() {
    let tokens = lex("\"hello\"");

    assert_eq!(
        tokens[0].kind,
        TokenKind::Literal(Literal::String("hello".into()))
    );
}

//
// Comments
//

#[test]
fn single_line_comment() {
    let tokens = lex("// comment\nmodule");

    assert_eq!(tokens[0].kind, TokenKind::Keyword(Keyword::Module));
}

#[test]
fn nested_comment() {
    let tokens = lex("/* one /* two */ three */ module");

    assert_eq!(tokens[0].kind, TokenKind::Keyword(Keyword::Module));
}

//
// Punctuation
//

#[test]
fn semicolon() {
    let tokens = lex(";");

    assert_eq!(
        tokens[0].kind,
        TokenKind::Punctuation(Punctuation::Semicolon)
    );
}

//
// Complete program
//

#[test]
fn simple_module() {
    let tokens = lex(r#"
module Main

signal x : trit;

end
"#);

    assert_eq!(tokens[0].kind, TokenKind::Keyword(Keyword::Module));

    assert_eq!(tokens[1].kind, TokenKind::Identifier("Main".into()));

    assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
}
