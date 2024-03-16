//! Representation of block expressions in wright source code.

use crate::parser::ast::{expression::Expression, metadata::AstNodeMeta};

/// A block in wright source code.
#[derive(Debug)]
pub struct Block<'src> {
    /// The AST node metadata.
    pub meta: AstNodeMeta<'src>,
    /// The body of this block as an expression.
    pub body: Box<Expression<'src>>,
}
