//! [Parse] implementation for [Identifier].

use super::{error::{ParserError, ParserErrorKind}, Parse};
use crate::{
    ast::identifier::Identifier,
    lexer::{token::{Token, TokenTy}, Lexer},
};

impl Parse for Identifier {
    fn parse(lexer: &mut Lexer) -> Result<Self, ParserError> {
        let next_token = lexer.next_token();

        // Get the fragment from the next token if it's the right type (or produce an error). 
        let ident_fragment = match next_token {
            Some(Token { variant: TokenTy::Identifier, fragment }) => Ok(fragment),

            Some(Token { fragment, .. }) => Err(ParserError {
                kind: ParserErrorKind::ExpectedIdentifier,
                location: fragment,
                help: None,
            }),

            None => Err(ParserError {
                kind: ParserErrorKind::ExpectedIdentifier,
                location: lexer.remaining.clone(),
                help: Some("found end of source".into()),
            }),
        }?;

        Ok(Identifier { fragment: ident_fragment })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::identifier::Identifier,
        lexer::Lexer,
        parser::{error::ParserErrorKind, Parse},
    };

    #[test]
    fn test_parse_ident() {
        let mut lexer = Lexer::new_test("source");
        let ident = Identifier::parse(&mut lexer).unwrap();
        assert_eq!(ident.fragment.as_str(), "source");
        assert_eq!(lexer.remaining.len(), 0);
    }

    #[test]
    fn test_parse_ident_fail() {
        for fail in ["12", "+", " ", " test", "_", "record"] {
            let mut lexer = Lexer::new_test(&fail);
            let error = Identifier::parse(&mut lexer).unwrap_err();
            assert_eq!(error.kind, ParserErrorKind::ExpectedIdentifier);
        }
    }
}
