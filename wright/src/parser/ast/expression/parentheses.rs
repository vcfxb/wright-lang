//! An expression in parentheses in Wright source code.

use crate::parser::ast::metadata::AstNodeMeta;

use super::Expression;

#[derive(Debug)]
pub struct ParenthesesExpression<'src> {
    /// The AST node metadata.
    pub meta: AstNodeMeta<'src>,
    /// The body of this block as an expression.
    pub body: Box<Expression<'src>>,
}
