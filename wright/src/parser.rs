//! This parser module is responsible for turning the stream of [Token]s from the [Lexer] into a tree of [AST] nodes.
//!
//! [AST]: crate::ast

use super::lexer::Lexer;
use error::ParserError;

mod identifier;
mod path;
pub mod error;

/// Trait implemented by all AST nodes that can be parsed.
pub trait Parse: Sized {
    /// Attempt to parse a tree node of this type from a given [Lexer].
    fn parse(lexer: &mut Lexer) -> Result<Self, ParserError>;
}
