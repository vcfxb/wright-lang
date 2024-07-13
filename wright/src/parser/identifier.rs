use crate::{ast::identifier::Identifier, lexer::{token::TokenTy, Lexer}};
use super::{Parse, ParseError};

impl Parse for Identifier {
    fn parse(lexer: &mut Lexer) -> Result<Self, ParseError> {
        let ident_token = lexer.expect(TokenTy::Identifier)?;
        Ok(Identifier { fragment: ident_token.fragment })
    }
}
