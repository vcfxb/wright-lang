//! Utilities for parsing through whitespace.

use super::{
    error::{ParserError, ParserErrorKind},
    Parser,
};
use crate::lexer::token::TokenTy;

/// Consume and ignore a [TokenTy::Whitespace] from the front of the [Parser].
/// If there is not one, do nothing.
pub fn optional_whitespace(parser: &mut Parser) {
    parser.next_if_is(TokenTy::Whitespace);
}

/// Require a whitespace from the [Parser]. Do not advance if the next [Token] is not a whitespace.
///
/// [Token]: crate::lexer::token::Token
pub fn require_whitespace(parser: &mut Parser) -> Result<(), ParserError> {
    match parser.next_if_is(TokenTy::Whitespace) {
        Some(_) => Ok(()),
        None => Err(ParserError {
            kind: ParserErrorKind::ExpectedWhitespace,
            location: parser.peek_fragment_or_rest_cloned(),
            help: None,
        })
    }
}
