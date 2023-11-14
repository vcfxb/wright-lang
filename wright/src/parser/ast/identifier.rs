//! Identifiers in wright source code.

use super::metadata::AstNodeMeta;
use crate::parser::{
    error::{ParserError, ParserErrorVariant},
    lexer::{
        tokens::{Token, TokenTy},
        IndexedToken,
    },
    state::ParserState,
    util::NodeParserResult,
};

/// An identifier in the source code being parsed.
#[derive(Debug, Clone, Copy)]
pub struct Identifier<'src> {
    /// An identifier is just a string in source code so we use a single metadata here
    /// and pass on the indetifier from the matching source.
    pub inner: AstNodeMeta<'src>,
}

impl<'src> Identifier<'src> {
    /// Get the matching source for this identifier.
    pub fn matching_source(&self) -> &'src str {
        self.inner.matching_source
    }
}

/// Parse an identifier in source code.
pub fn parse_identifier<'src>(
    parser_state: &mut ParserState<'src>,
) -> NodeParserResult<Identifier<'src>> {
    // Conditionally get an identifier token from the lexer.
    let IndexedToken {
        index,
        token: Token { length, .. },
    } = parser_state
        // Require the token to be an identifier token.
        .next_token_if_ty_eq(TokenTy::Identifier)
        // Error out if there is not an identifier token.
        .ok_or_else(|| ParserError {
            byte_range: parser_state.peek_byte_range(),
            ty: ParserErrorVariant::Expected("identifer"),
        })?;

    // Turn the token into an AST node and return OK.
    Ok(Identifier {
        inner: parser_state.make_ast_node_meta(index, length),
    })
}
