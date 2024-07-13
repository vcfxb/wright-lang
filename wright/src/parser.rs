//! This parser module is responsible for turning the stream of [Token]s from the [Lexer] into a tree of [AST] nodes.
//!
//! [AST]: crate::ast

use super::lexer::{
    token::{Token, TokenTy},
    Lexer,
};

mod identifier;

/// Errors that can arise when parsing a source to an abstract syntax tree node.
#[derive(Debug)]
pub enum ParseError {
    /// Expected one type of token, found another
    Expected {
        /// The expected variant.
        expected: TokenTy,
        /// The token found from the lexer.
        found: Option<Token>,
    },
}

/// Trait implemented by all AST nodes that can be parsed.
pub trait Parse: Sized {
    /// Attempt to parse a tree node of this type from a given [Lexer].
    fn parse(lexer: &mut Lexer) -> Result<Self, ParseError>;
}

impl Lexer {
    /// Pull the next token from a lexer, and return an error if it's not of the given variant.
    pub fn expect(&mut self, token_ty: TokenTy) -> Result<Token, ParseError> {
        let next_token = self.next_token().ok_or(ParseError::Expected {
            expected: token_ty,
            found: None,
        })?;

        if next_token.variant != token_ty {
            return Err(ParseError::Expected {
                expected: token_ty,
                found: Some(next_token),
            });
        }

        Ok(next_token)
    }
}
