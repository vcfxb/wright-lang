//! Boolean literal representation and parsing in Wright source.

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

/// A boolean literal (true or false) in Wright source code.
#[derive(Debug)]
pub struct BooleanLiteral<'src> {
    /// The AST Node Metadata.
    pub meta: AstNodeMeta<'src>,
    /// The value of this literal
    pub value: bool,
}

/// Attempt to parse a boolean literal from the lexer held by the parser state.
pub fn parse_boolean_literal<'src>(
    parser_state: &mut ParserState<'src>,
) -> NodeParserResult<BooleanLiteral<'src>> {
    // Try to parse a `true` token and a `false` token.
    for (token_ty, value) in [(TokenTy::True, true), (TokenTy::False, false)] {
        // Try to take the appropriate token from the parser state.
        if let Some(IndexedToken {
            index,
            token: Token { length, .. },
        }) = parser_state.next_token_if_ty_eq(token_ty)
        {
            // On success, return the popped token's appropriate AST node.
            return Ok(BooleanLiteral {
                meta: parser_state.make_ast_node_meta(index, length),
                value,
            });
        }
    }

    // If neither parse succeeds, return an error.
    Err(ParserError {
        byte_range: parser_state.peek_byte_range(),
        ty: ParserErrorVariant::Expected("boolean literal"),
    })
}
