//! This parser module is responsible for turning the stream of [Token]s from the [Lexer] into a tree of [AST] nodes.
//!
//! [AST]: crate::ast
//! [Token]: crate::lexer::token::Token

use super::lexer::Lexer;
use error::ParserError;

pub mod error;
mod identifier;
mod path;

/// Trait implemented by all AST nodes that can be parsed.
pub trait Parse: Sized {
    /// Attempt to parse a tree node of this type from a given [Lexer].
    fn parse(lexer: &mut Lexer) -> Result<Self, ParserError>;
}
