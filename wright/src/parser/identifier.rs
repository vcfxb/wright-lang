//! Parsing implementation for [Identifier].

use super::{
    error::{ParserError, ParserErrorKind},
    Parser,
};
use crate::{ast::identifier::Identifier, lexer::token::TokenTy};

impl Identifier {
    /// Parse an [Identifier] from a [Parser]. Leave the [Parser] unadvanced otherwise.
    pub fn parse(parser: &mut Parser) -> Result<Self, ParserError> {
        parser
            .next_if_is(TokenTy::Identifier)
            .map(|token| Identifier {
                fragment: token.fragment,
            })
            .ok_or_else(|| {
                ParserErrorKind::ExpectedIdentifier.at(parser.peek_fragment_or_rest_cloned())
            })
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
