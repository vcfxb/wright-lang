//! Utilities for parsing through whitespace.

use crate::lexer::{
    token::{Token, TokenTy},
    Lexer,
};
use std::mem;

/// Consume and ignore a [TokenTy::Whitespace] from the front of the lexer.
/// If there is not one, do nothing.
pub fn optional_whitespace(lexer: &mut Lexer) {
    let mut fork = lexer.fork();

    if let Some(Token {
        variant: TokenTy::Whitespace,
        ..
    }) = fork.next_token()
    {
        // Replace the original lexer with the fork.
        let _ = mem::replace(lexer, fork);
    }
}
