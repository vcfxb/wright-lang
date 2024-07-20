//! Utilities for parsing through whitespace.

use crate::lexer::token::TokenTy;
use super::{error::{ParserError, ParserErrorKind}, Parser};

/// Consume and ignore a [TokenTy::Whitespace] from the front of the [Parser].
/// If there is not one, do nothing.
pub fn optional_whitespace(parser: &mut Parser) {
    parser.next_if_is(TokenTy::Whitespace);
}

/// Require a whitespace from the [Parser]. Do not advance if the next [Token] is not a whitespace. 
pub fn require_whitespace(parser: &mut Parser) -> Result<(), ParserError> {
    if parser.next_if_is(TokenTy::Whitespace).is_none() {
        Err(ParserError { 
            kind: ParserErrorKind::ExpectedWhitespace,
            location: parser.peek_fragment().cloned().unwrap_or(parser.lexer().remaining.clone()),
            help: None 
        })
    } else {
        Ok(())
    }
}
