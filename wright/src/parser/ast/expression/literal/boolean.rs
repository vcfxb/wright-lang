//! Boolean literal representation and parsing in Wright source. 

use crate::parser::{ast::{metadata::AstNodeMeta, AstNode}, Parser, ParserError, lexer::{tokens::{Token, TokenTy}, IndexedToken}, ParserErrorVariant};

/// A boolean literal (true or false) in Wright source code.
#[derive(Debug)]
pub struct BooleanLiteral<'src> {
    /// The AST Node Metadata. 
    pub meta: AstNodeMeta<'src>,
    /// The value of this literal
    pub value: bool,
}

impl<'src> Parser<'src> {
    /// Parse a boolean literal or error. 
    pub fn parse_boolean(&mut self) -> Result<BooleanLiteral<'src>, ParserError> {
        // Clone lexer to parse with. 
        let mut lexer = self.lexer.clone();

        match lexer.next() {
            Some(IndexedToken { token: Token { variant: TokenTy::True, ..}, ..}) => {
                Ok(BooleanLiteral {
                    meta: self.update_lexer(lexer),
                    value: true
                })
            },

            Some(IndexedToken { token: Token { variant: TokenTy::False, ..}, ..}) => {
                Ok(BooleanLiteral { 
                    meta: self.update_lexer(lexer), 
                    value: false 
                })
            }

            _ => Err(ParserError { 
                byte_range: self.lexer.index..lexer.index, 
                ty: ParserErrorVariant::Expected("boolean literal") 
            }),
        }
    }
}

impl<'src> AstNode for BooleanLiteral<'src> {
    fn write_self(&self, w: &mut dyn std::io::Write, style: &ptree::Style) -> std::io::Result<()> {
        write!(w, "{}", style.paint(format!("BooleanLiteral ({})", self.value)))
    }

    fn children(&self) -> std::borrow::Cow<[&dyn AstNode]> {
        // No childeren
        Vec::new().into()
    }
}
