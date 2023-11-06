//! Integer literal representation and parsing in wright source.

use num::{BigUint, Num};
use std::cmp;
use crate::parser::{state::ParserState, ast::metadata::AstNodeMeta, util::{NodeParserResult, ParserSuccess}, lexer::{IndexedToken, tokens::{Token, TokenTy}}, error::{ParserError, ParserErrorVariant}};


/// An integer in Wright source code.
#[derive(Debug)]
pub struct IntegerLiteral<'src> {
    /// Metadata about this literal in source code.
    pub meta: AstNodeMeta<'src>,
    /// The value represented in source code.
    pub value: BigUint,
}

/// Parse an [`IntegerLiteral`] from source code. 
pub fn parse_integer_literal<'src>(mut parser_state: ParserState<'src>) -> NodeParserResult<'src, IntegerLiteral<'src>> {
    // Get the initial index of the lexer for later error reporting. 
    let initial_index = parser_state.lexer.index;

    // Match on the next token from the lexer, erroring if it's anything but an integer literal. 
    match parser_state.lexer.next() {
        Some(IndexedToken {
            index,
            token:
                Token {
                    variant: TokenTy::IntegerLit,
                    length,
                },
        }) => {
            // Get the matching source of this token.
            let matching_source = &parser_state.source[index..index + length];

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

            // Make the AST node metadata for the parsed value
            let ast_node_meta = parser_state.make_ast_node_meta(index, length);

            Ok(ParserSuccess { 
                updated_parser_state: parser_state, 
                ast_node: IntegerLiteral {
                    meta: ast_node_meta,
                    value,
                }
            })
        }

        _ => Err(ParserError {
            byte_range: initial_index..parser_state.lexer.index,
            ty: ParserErrorVariant::Expected("integer literal"),
        }),
    }
}
