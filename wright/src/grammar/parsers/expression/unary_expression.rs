use crate::grammar::ast::{eq::AstEq, Expression, UnaryExpression, UnaryOp};
use crate::grammar::model::{Fragment, HasSourceReference};
use nom::IResult;

impl UnaryOp {}

impl<'s> UnaryExpression<'s> {
    /// Parse a unary expression in source code.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        todo!()
    }
}

impl<'s> Into<Expression<'s>> for UnaryExpression<'s> {
    fn into(self) -> Expression<'s> {
        Expression::UnaryExpression(self)
    }
}

impl<'s> HasSourceReference<'s> for UnaryExpression<'s> {
    fn get_source_ref(&self) -> &Fragment<'s> {
        &self.frag
    }
}

impl<'s> AstEq for UnaryExpression<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        fst.op == snd.op && AstEq::ast_eq(&*fst.inner, &*snd.inner)
    }
}
