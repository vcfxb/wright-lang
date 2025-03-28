//! Utilities for parsing through whitespace.

use super::{
    Parser,
    error::{ParserError, ParserErrorKind},
};
use crate::lexer::token::TokenTy;

/// Consume and ignore a [TokenTy::Whitespace] from the front of the [Parser].
/// If there is not one, do nothing.
pub fn optional_whitespace(parser: &mut Parser) {
    while parser.peek_variant() == Some(TokenTy::Whitespace) {
        parser.advance(1);
    }
}

/// Require a whitespace from the [Parser]. Do not advance if the next [Token] is not a whitespace.
///
/// [Token]: crate::lexer::token::Token
pub fn require_whitespace(parser: &mut Parser) -> Result<(), ParserError> {
    match parser.next_if_is(TokenTy::Whitespace) {
        Some(_) => {
            // Remove any other non-contiguous whitespaces that may have followed.
            optional_whitespace(parser);
            Ok(())
        }

        None => Err(ParserError {
            kind: ParserErrorKind::ExpectedWhitespace,
            location: parser.peek_fragment_or_rest_cloned(),
            help: None,
        }),
    }
}
