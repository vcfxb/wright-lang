use wright::parser::lexer::{Lexer, TokenTy};

/// Test unterminated string literal.
#[test]
fn unterminated_string_literal() {
    let tokens = Lexer::lex(r#""this string is not closed"#);
    assert_eq!(tokens.len(), 2);
    assert_eq!(
        tokens[0].variant,
        TokenTy::StringLit {
            is_format: false,
            is_terminated: false
        }
    );
}

/// Test string literal with escaped terminal.
#[test]
fn string_with_escape() {
    let tokens = Lexer::lex(r#" "this string has an escaped terminator \" " "#);
    assert_eq!(tokens.len(), 4);
    assert_eq!(
        tokens[1].variant,
        TokenTy::StringLit {
            is_format: false,
            is_terminated: true
        }
    );
}
