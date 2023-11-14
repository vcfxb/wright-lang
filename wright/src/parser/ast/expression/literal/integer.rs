//! Integer literal representation and parsing in wright source.

use crate::parser::{
    ast::metadata::AstNodeMeta,
    error::{ParserError, ParserErrorVariant},
    lexer::{
        tokens::{Token, TokenTy},
        IndexedToken,
    },
    state::ParserState,
    util::NodeParserResult,
};
use num::{BigUint, Num};
use std::cmp;

/// An integer in Wright source code.
#[derive(Debug)]
pub struct IntegerLiteral<'src> {
    /// Metadata about this literal in source code.
    pub meta: AstNodeMeta<'src>,
    /// The value represented in source code.
    pub value: BigUint,
}

/// Parse an [`IntegerLiteral`] from source code.
pub fn parse_integer_literal<'src>(
    parser_state: &mut ParserState<'src>,
) -> NodeParserResult<IntegerLiteral<'src>> {
    // Read and destructure an integer literal token from the lexer.
    let IndexedToken {
        index,
        token: Token { length, .. },
    } = parser_state
        // All integer literals should be of this token type.
        .next_token_if_ty_eq(TokenTy::IntegerLit)
        // Error out if the next token is not an integer literal.
        .ok_or_else(|| ParserError {
            byte_range: parser_state.peek_byte_range(),
            ty: ParserErrorVariant::Expected("integer literal"),
        })?;

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

    // Return ok.
    Ok(IntegerLiteral {
        meta: parser_state.make_ast_node_meta(index, length),
        value,
    })
}
