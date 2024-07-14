
#![cfg(feature = "lexer")]

use std::sync::Arc;
use wright::{lexer::{
    token::TokenTy,
    Lexer,
}, source_tracking::{filename::FileName, source::Source}};

fn new_test_lexer(s: &'static str) -> Lexer {
    Lexer::new(Arc::new(Source::new_from_static_str(FileName::None, s)))
}

/// Test unterminated string literal.
#[test]
fn unterminated_string_literal() {
    let mut lexer = new_test_lexer(r#""this string is not closed"#);

    let token = lexer.next_token().unwrap();

    assert_eq!(token.variant, TokenTy::StringLiteral { terminated: false });
    assert_eq!(token.fragment.as_str(), lexer.remaining.source.source().as_str());
    assert_eq!(lexer.bytes_remaining(), 0);

    assert!(lexer.next_token().is_none());
}

/// Test string literal with escaped terminal.
#[test]
fn string_with_escape() {
    let mut lexer = new_test_lexer(r#""this string has an escaped terminator \" ""#);

    let token = lexer.next_token().unwrap();

    assert_eq!(token.variant, TokenTy::StringLiteral { terminated: true });
    assert_eq!(token.fragment.as_str(), lexer.remaining.source.source().as_str());
    assert_eq!(lexer.bytes_remaining(), 0);
    
    assert!(lexer.next_token().is_none());
}
