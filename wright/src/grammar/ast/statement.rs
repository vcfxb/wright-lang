use crate::grammar::model::Fragment;
use crate::grammar::ast::Expression;

/// An expression whose results aren't used.
#[derive(Clone, Debug)]
pub struct ExpressionStatement<'s> {
    /// Associated Fragment in source code.
    pub frag: Fragment<'s>,
    /// The expression.
    pub inner: Box<Expression<'s>>,
}

/// A statement in wright source code.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum Statement<'s> {
    ExpressionStatement(ExpressionStatement<'s>),
}
