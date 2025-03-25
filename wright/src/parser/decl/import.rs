//! Parser implementation for `use path::to::thing;` declaration.

use crate::{
    ast::{decl::import::ImportDecl, identifier::Identifier, path::Path},
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
        
        // Whitespace and then "as ...;" or optional whitespace and semi ";".

        // The "as ...;" requires a whitespace.
        let imported_as = if parser.matches(&[TokenTy::Whitespace, TokenTy::KwAs]) {
            parser.advance(2);
            
            whitespace::require_whitespace(parser)
                .map_err(|e| e.with_help("whitespace needed between \"as\" and binding."))?;

            let imported_as = Identifier::parse(parser)
                .map_err(|e| e.with_help("expected binding in \"use ... as\" declaration."))?;

            Some(imported_as)
        } else {
            None
        };

        whitespace::optional_whitespace(parser);

        if let Some(semi) = parser.next_if_is(TokenTy::Semi) {
            Ok(ImportDecl {
                matching_source: Fragment::cover(use_kw.fragment, semi.fragment),
                imported_item: path,
                imported_as,
            })
        } else {
            Err(ParserErrorKind::ImportMustEndWithSemicolon
                .at(parser.peek_fragment_or_rest_cloned()))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::decl::import::ImportDecl, lexer::Lexer, parser::Parser};

    #[test]
    fn test_import() {
        let mut parser = Parser::new(Lexer::new_test("use wright::util;"));
        let import_decl = ImportDecl::parse(&mut parser).unwrap();
        assert!(parser.lexer.remaining.is_empty());
        assert_eq!(import_decl.imported_item.head.fragment.as_str(), "wright");
        assert_eq!(import_decl.imported_item.tail[0].fragment.as_str(), "util");
    }

    #[test]
    fn test_import_with_whitespace() {
        let mut parser = Parser::new(Lexer::new_test("use wright :: util ;"));
        let import_decl = ImportDecl::parse(&mut parser).unwrap();
        assert!(parser.lexer.remaining.is_empty());
        assert_eq!(import_decl.imported_item.head.fragment.as_str(), "wright");
        assert_eq!(import_decl.imported_item.tail[0].fragment.as_str(), "util");
    }
    
    #[test]
    fn test_import_as() {
        let mut parser = Parser::new(Lexer::new_test("use wright::util as u;"));
        let import_decl = ImportDecl::parse(&mut parser).unwrap();
        assert!(parser.lexer.remaining.is_empty());
        assert_eq!(import_decl.imported_item.head.fragment.as_str(), "wright");
        assert_eq!(import_decl.imported_item.tail[0].fragment.as_str(), "util");
        assert_eq!(import_decl.imported_as.unwrap().fragment.as_str(), "u");
    }
}
