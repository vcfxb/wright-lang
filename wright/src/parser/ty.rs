//! Parser implementation for parsing types.

use crate::ast::ty::{AtomicTy, ReferenceTy, Type};

use super::{error::{ParserError, ParserErrorKind}, Parser};

mod primitive;
mod reference;

impl Type {
    /// Parse a type signature in source code.
    pub fn parse(parser: &mut Parser) -> Result<Self, ParserError> {
        // Atempt to parse atomic types first -- they're the simplest. If we fail to parse, the parser doesn't advance.
        // Since they're all keywords we don't have to worry at all about under-greedy parsing (yet).
        if let Ok(atomic) = AtomicTy::parse(parser) {
            return Ok(Type::Atomic(atomic));
        }

        let bytes_remaining = parser.bytes_remaining();

        match ReferenceTy::parse(parser) {
            Ok(reference_ty) => return Ok(Type::Reference(reference_ty)),

            Err(err) => {
                // If the parser was advanced in parsing the reference type, error out here.
                if bytes_remaining != parser.bytes_remaining() {
                    return Err(err.with_help("encountered error while parsing reference type signature"));
                }

                // If we didn't advance we can just ignore the error and try parsing other type signature
                // forms or fall through to the catch all "expected type signature" error (since it means 
                // we would have not seen an `@` to start a reference type signature).
            },
        }

        Err(ParserErrorKind::ExpectedTypeSignature.at(parser.peek_fragment_or_rest_cloned()))
    }
}

