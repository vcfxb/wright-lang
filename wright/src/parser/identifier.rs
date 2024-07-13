use crate::{ast::identifier::Identifier, lexer::{token::TokenTy, Lexer}};
use super::{Parse, ParseError};

impl Parse for Identifier {
    fn parse(lexer: &mut Lexer) -> Result<Self, ParseError> {
        let ident_token = lexer.expect(TokenTy::Identifier)?;
        Ok(Identifier { fragment: ident_token.fragment })
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::identifier::Identifier, lexer::{token::TokenTy, Lexer}, parser::{Parse, ParseError}};

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
            assert!(matches!(&error, ParseError::Expected { expected: TokenTy::Identifier, .. }));
        }
    }
}
