//! Boolean literal representation and parsing in Wright source.

use crate::parser::{
    ast::metadata::AstNodeMeta,
    error::{ParserError, ParserErrorVariant},
    lexer::{
        tokens::{Token, TokenTy},
        IndexedToken,
    },
    state::ParserState,
    util::{NodeParserResult, ParserSuccess},
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
pub fn parse_boolean_literal(mut parser_state: ParserState) -> NodeParserResult<BooleanLiteral> {
    // Get the initial lexer index for later error reporting.
    let initial_index = parser_state.lexer.index;

    // Match on the next token
    match parser_state.lexer.next() {
        Some(IndexedToken {
            index,
            token:
                Token {
                    variant: TokenTy::True,
                    length,
                },
        }) => {
            // Generate the AST node metadata for the parsed node.
            let ast_node_meta = parser_state.make_ast_node_meta(index, length);

            Ok(ParserSuccess {
                updated_parser_state: parser_state,
                ast_node: BooleanLiteral {
                    meta: ast_node_meta,
                    value: true,
                },
            })
        }

        Some(IndexedToken {
            token:
                Token {
                    variant: TokenTy::False,
                    length,
                },
            index,
        }) => {
            // Make the AST node meta for the parsed node.
            let ast_node_meta = parser_state.make_ast_node_meta(index, length);

            Ok(ParserSuccess {
                updated_parser_state: parser_state,
                ast_node: BooleanLiteral {
                    meta: ast_node_meta,
                    value: false,
                },
            })
        }

        _ => Err(ParserError {
            byte_range: initial_index..parser_state.lexer.index,
            ty: ParserErrorVariant::Expected("boolean literal"),
        }),
    }
}
