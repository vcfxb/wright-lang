//! Integer literal representation and parsing in wright source.

use num::{BigUint, Num};
use std::cmp;

use crate::parser::{
    ast::metadata::AstNodeMeta,
    lexer::{
        tokens::{Token, TokenTy},
        IndexedToken,
    },
    Parser, ParserError, ParserErrorVariant, ParserResult,
};

/// An integer in Wright source code.
#[derive(Debug)]
pub struct IntegerLiteral<'src> {
    /// Metadata about this literal in source code.
    pub meta: AstNodeMeta<'src>,
    /// The value represented in source code.
    pub value: BigUint,
}

impl<'src> Parser<'src> {
    /// Parse an integer literal or error.
    pub fn parse_integer(&mut self) -> ParserResult<IntegerLiteral<'src>> {
        // Clone the current lexer (token cursor) to parse an integer.
        let mut lexer = self.lexer.clone();

        // Take an integer literal token from the lexer or error.
        match lexer.next() {
            Some(IndexedToken {
                index,
                token:
                    Token {
                        variant: TokenTy::IntegerLit,
                        length,
                    },
            }) => {
                // Get the matching source of this token.
                let matching_source = &self.source[index..index + length];

                // Check for a prefix
                let prefix = &matching_source[..cmp::max(2, matching_source.len())];

                // Get a radix off the prefix
                let radix = match prefix {
                    "0x" | "0X" => 16,
                    "0b" | "0B" => 2,
                    "0o" => 8,
                    _ => 10,
                };

                // Strip the prefix from the string to get the body of it to parse.
                let body = if radix != 10 {
                    &matching_source[2..]
                } else {
                    matching_source
                };

                // Parse it.
                let value = BigUint::from_str_radix(body, radix)
                    // Panic here as the lexer should check for this.
                    .expect("lexer checks integer literal format");

                Ok(IntegerLiteral {
                    meta: self.update_lexer(lexer),
                    value,
                })
            }

            _ => Err(ParserError {
                byte_range: self.lexer.index..lexer.index,
                ty: ParserErrorVariant::Expected("integer literal"),
            }),
        }
    }
}
