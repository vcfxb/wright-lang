//! Boolean literal parsing logic.

use crate::{
    ast::literal::BooleanLiteral,
    lexer::token::TokenTy,
    parser::{
        Parser,
        error::{ParserError, ParserErrorKind},
    },
};

impl BooleanLiteral {
    /// Parse a boolean literal from the given [Parser].
    pub fn parse(parser: &mut Parser) -> Result<Self, ParserError> {
        if let Some(token) = parser.next_if_is(TokenTy::KwTrue) {
            return Ok(BooleanLiteral {
                fragment: token.fragment,
                value: true,
            });
        }

        if let Some(token) = parser.next_if_is(TokenTy::KwFalse) {
            return Ok(BooleanLiteral {
                fragment: token.fragment,
                value: false,
            });
        }

        Err(ParserErrorKind::ExpectedBooleanLiteral.at(parser.peek_fragment_or_rest_cloned()))
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::literal::BooleanLiteral, lexer::Lexer, parser::Parser};

    #[test]
    fn works() {
        for s in ["true", "false"] {
            let mut p = Parser::new(Lexer::new_test(s));

            assert!(BooleanLiteral::parse(&mut p).is_ok());
        }
    }
}
