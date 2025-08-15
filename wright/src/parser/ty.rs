//! Parser implementation for parsing types.

use crate::{
    ast::{
        // identifier::Identifier,
        ty::{AtomicTy, ReferenceTy, Type},
    },
    lexer::token::TokenTy,
};

use super::{
    Parser,
    error::{ParserError, ParserErrorKind},
};

mod constrained_ty;
mod primitive;
mod reference;

impl Type {
    /// Parse a type signature in source code.
    pub fn parse(parser: &mut Parser) -> Result<Self, ParserError> {
        // First try to parse a type, then check to see if the `constrain` keyword follows it,
        // since that's effectively a type suffix.

        // Atempt to parse atomic types first -- they're the simplest. If we fail to parse, the parser doesn't advance.
        // Since they're all keywords we don't have to worry at all about under-greedy parsing (yet).
        let atomic_ty_parse_fn = |parser: &mut Parser| AtomicTy::parse(parser).map(Type::Atomic);
        let reference_ty_parse_fn =
            |parser: &mut Parser| ReferenceTy::parse(parser).map(Type::Reference);

        let order = [atomic_ty_parse_fn, reference_ty_parse_fn];

        for parse_fn in order {
            let initial_bytes_remaining = parser.bytes_remaining();

            match (parse_fn)(parser) {
                // Successful parse.
                Ok(t) => return Ok(t),

                // Partial parse with error.
                Err(err) if parser.bytes_remaining() != initial_bytes_remaining => return Err(err),

                // Parsing error with no tokens consumed.
                Err(_) => continue,
            }
        }

        Err(ParserErrorKind::ExpectedTypeSignature.at(parser.peek_fragment_or_rest_cloned()))
    }
}
