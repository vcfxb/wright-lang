//! Boolean literal representation and parsing in Wright source.

use crate::parser::{
    ast::metadata::AstNodeMeta,
    lexer::{
        tokens::{Token, TokenTy},
        IndexedToken,
    },
    Parser, ParserError, ParserErrorVariant, ParserResult,
};

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
    pub fn parse_boolean(&mut self) -> ParserResult<BooleanLiteral<'src>> {
        // Clone lexer to parse with.
        let mut lexer = self.lexer.clone();

        match lexer.next() {
            Some(IndexedToken {
                token:
                    Token {
                        variant: TokenTy::True,
                        ..
                    },
                ..
            }) => Ok(BooleanLiteral {
                meta: self.update_lexer(lexer),
                value: true,
            }),

            Some(IndexedToken {
                token:
                    Token {
                        variant: TokenTy::False,
                        ..
                    },
                ..
            }) => Ok(BooleanLiteral {
                meta: self.update_lexer(lexer),
                value: false,
            }),

            _ => Err(ParserError {
                byte_range: self.lexer.index..lexer.index,
                ty: ParserErrorVariant::Expected("boolean literal"),
            }),
        }
    }
}
