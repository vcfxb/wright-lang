use crate::grammar::ast::Expression;
use crate::grammar::model::Fragment;
use std::fmt::Debug;

/// An expression whose results aren't used.
#[derive(Clone, Debug)]
pub struct ExpressionStatement<SourceCodeReference: Clone + Debug> {
    /// Associated source code.
    pub source: SourceCodeReference,
    /// The expression.
    pub inner: Box<Expression<SourceCodeReference>>,
}

/// A statement in wright source code.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum Statement<SourceCodeReference: Clone + Debug> {
    ExpressionStatement(ExpressionStatement<SourceCodeReference>),
}
