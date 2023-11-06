//! Boolean literal representation and parsing in Wright source.

use crate::parser::{ast::metadata::AstNodeMeta, util::{NodeParserResult, ParserSuccess}, state::ParserState, lexer::{IndexedToken, tokens::{Token, TokenTy}}, error::{ParserError, ParserErrorVariant}};

/// A boolean literal (true or false) in Wright source code.
#[derive(Debug)]
pub struct BooleanLiteral<'src> {
    /// The AST Node Metadata.
    pub meta: AstNodeMeta<'src>,
    /// The value of this literal
    pub value: bool,
}

/// Attempt to parse a boolean literal from the lexer held by the parser state. 
pub fn parse_boolean_literal<'src>(mut parser_state: ParserState<'src>) -> NodeParserResult<'src, BooleanLiteral<'src>> {
    // Get the initial lexer index for later error reporting. 
    let initial_index = parser_state.lexer.index;

    // Match on the next token 
    match parser_state.lexer.next() {
        Some(IndexedToken {
            index,
            token: Token {
                variant: TokenTy::True,
                length
            }
        }) => {
            // Generate the AST node metadata for the parsed node. 
            let ast_node_meta = parser_state.make_ast_node_meta(index, length);

            Ok(ParserSuccess {
                updated_parser_state: parser_state,
                ast_node: BooleanLiteral { meta: ast_node_meta, value: true },
            })
        },

        Some(IndexedToken {
            token:
                Token {
                    variant: TokenTy::False,
                    length
                },
            index
        }) => {
            // Make the AST node meta for the parsed node.
            let ast_node_meta = parser_state.make_ast_node_meta(index, length);

            Ok(ParserSuccess { updated_parser_state: parser_state, ast_node: BooleanLiteral { meta: ast_node_meta, value: false } })
        },

        _ => Err(ParserError {
            byte_range: initial_index..parser_state.lexer.index,
            ty: ParserErrorVariant::Expected("boolean literal"),
        }),
    }
}



// impl<'a> Parse for ParseBooleanLiteral<'a> {
//     type Success = BooleanLiteral<'a>;

//     type Error = ParserError;

//     /// Attempt to parse a boolean literal from the given parser. This will mutate the parser, and return either [`Ok`]
//     /// with a [`BooleanLiteral`] or an [`Err`] containing a [`ParserError`]. The parser will have the front-most 
//     /// [`Token`] consumed from its lexer regardless. 
//     fn parse<'src>(&self, parser: &mut Parser<'src>) -> Result<Self::Success, Self::Error> {
//         // Get the initial lexer index for use in calculating the span of errors reported. 
//         let initial_index = parser.lexer.index;

//        
//     }
// }
