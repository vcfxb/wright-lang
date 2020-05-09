use crate::grammar::ast::{eq::AstEq, Expression, UnaryExpression, UnaryOp};
use crate::grammar::model::HasSourceReference;
use nom::IResult;
use crate::grammar::tracing::input::OptionallyTraceable;

impl UnaryOp {}

impl<T> UnaryExpression<T> {
    /// Name used in parser tracing.
    pub const TRACE_NAME: &'static str = "UnaryExpr";
}

impl<I: OptionallyTraceable> UnaryExpression<I> {
    /// Parse a unary expression in source code.
    pub fn parse(input: I) -> IResult<I, Self> {
        unimplemented!()
    }
}

impl<I> Into<Expression<I>> for UnaryExpression<I> {
    fn into(self) -> Expression<I> {
        Expression::UnaryExpression(self)
    }
}

impl<I> HasSourceReference<I> for UnaryExpression<I> {
    fn get_source_ref(&self) -> &I {
        &self.frag
    }
}

impl<I> AstEq for UnaryExpression<I> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        fst.op == snd.op && AstEq::ast_eq(&*fst.inner, &*snd.inner)
    }
}
