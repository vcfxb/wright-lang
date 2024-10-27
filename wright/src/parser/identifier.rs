//! [Parse] implementation for [Identifier].

#![warn(rustdoc::broken_intra_doc_links)]

use super::{
    error::{ParserError, ParserErrorKind},
    Parser,
};
use crate::{
    ast::identifier::Identifier,
    lexer::token::{Token, TokenTy},
};

impl Identifier {
    /// Parse an [Identifier] from a [Parser]. Leave the [Parser] unadvanced otherwise.
    pub fn parse(parser: &mut Parser) -> Result<Self, ParserError> {
        match parser.next_if_is(TokenTy::Identifier) {
            Some(Token { fragment, .. }) => Ok(Identifier { fragment }),

            None => match parser.peek_fragment() {
                Some(next_frag) => Err(ParserError {
                    kind: ParserErrorKind::ExpectedIdentifier,
                    location: next_frag.clone(),
                    help: None,
                }),

                None => Err(ParserError {
                    kind: ParserErrorKind::ExpectedIdentifier,
                    location: parser.lexer.remaining.clone(),
                    help: Some("found end of source".into()),
                }),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::identifier::Identifier,
        lexer::Lexer,
        parser::{error::ParserErrorKind, Parser},
    };

    #[test]
    fn test_parse_ident() {
        let mut parser = Parser::new(Lexer::new_test("source"));
        let ident = Identifier::parse(&mut parser).unwrap();
        assert_eq!(ident.fragment.as_str(), "source");
        assert_eq!(parser.lexer().remaining.len(), 0);
    }

    #[test]
    fn test_parse_ident_fail() {
        for fail in ["12", "+", " ", " test", "_", "record"] {
            let mut parser = Parser::new(Lexer::new_test(&fail));
            let error = Identifier::parse(&mut parser).unwrap_err();
            assert_eq!(error.kind, ParserErrorKind::ExpectedIdentifier);
        }
    }
}
