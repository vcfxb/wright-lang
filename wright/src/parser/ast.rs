//! Various [AST] (abstract syntax tree) constructs used in Wright.
//!
//! [AST]: https://en.wikipedia.org/wiki/Abstract_syntax_tree

pub mod declaration;
pub mod expression;
pub mod identifier;
pub mod metadata;
pub mod path;
pub mod types;
pub mod statement;

/// Trait implementd
pub trait AstNode<'src> {
        
}
