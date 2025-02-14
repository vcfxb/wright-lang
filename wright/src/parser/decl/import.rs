//! Parser implementation for `use path::to::thing;` declaration.

use crate::{
    ast::{decl::import::ImportDecl, path::Path},
    lexer::token::{Token, TokenTy},
    parser::{
        error::{ParserError, ParserErrorKind},
        whitespace, Parser,
    },
    source_tracking::fragment::Fragment,
};

impl ImportDecl {
    /// Parse an import declaration.
    ///
    /// This will advance the parser if `use` is seen -- if a valid formed import does not follow,
    /// the parser may be left in the middle of a malformed declaration.
    pub fn parse(parser: &mut Parser) -> Result<Self, ParserError> {
        let use_kw: Token = parser.next_if_is(TokenTy::KwUse).ok_or(
            ParserErrorKind::ExpectedImportDeclaration.at(parser.peek_fragment_or_rest_cloned()),
        )?;

        // Require a whitespace after the keyword.
        whitespace::require_whitespace(parser)?;
        // Parse the path.
        let path: Path = Path::parse(parser)?;
        // End with an optional whitespace and then a semi-colon.
        whitespace::optional_whitespace(parser);

        if let Some(semi) = parser.next_if_is(TokenTy::Semi) {
            Ok(ImportDecl {
                matching_source: Fragment::cover(use_kw.fragment, semi.fragment),
                imported_item: path,
            })
        } else {
            Err(ParserErrorKind::ImportMustEndWithSemicolon
                .at(parser.peek_fragment_or_rest_cloned()))
        }
    }
}
