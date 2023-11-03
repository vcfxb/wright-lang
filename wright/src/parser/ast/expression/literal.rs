//! Representation for literal expressions in wright source code.

use crate::parser::{Parser, ParserResult, ParserErrorVariant};
use self::{integer::IntegerLiteral, boolean::BooleanLiteral};

pub mod boolean;
pub mod integer;

#[derive(Debug)]
pub enum Literal<'src> {
    /// An integer literal in source code.
    Integer(IntegerLiteral<'src>),
    /// A boolean literal in source code. 
    Boolean(BooleanLiteral<'src>),
}

impl<'src> Parser<'src> {
    /// Parse a literal from Wright source code. If no parse is successful, an error will be returned and
    /// the parser's internal state will remain unchaged. 
    pub fn parse_literal(&mut self) -> ParserResult<Literal<'src>> {
        // Try parsing an intger literal. 
        if let Ok(int_lit) = self.parse_integer() {
            Ok(Literal::Integer(int_lit))
        } 
        // If that fails, try parsing a boolean literal. 
        else if let Ok(bool_lit) = self.parse_boolean() {
            Ok(Literal::Boolean(bool_lit))
        }
        // If that fails, error
        else {
            Err(self.next_token_err(ParserErrorVariant::Expected("literal value")))
        }
    }
}

