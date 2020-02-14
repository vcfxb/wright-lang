use crate::grammar::model::{HasFragment, Fragment};
use crate::grammar::ast::{BinaryExpression, Expression, BinaryOp};
use crate::grammar::parsers::expression::ToExpression;
use nom::IResult;

impl<'s> BinaryExpression<'s> {
    fn new(frag: Fragment<'s>,
           left: Expression<'s>,
           op: BinaryOp,
           right: Expression<'s>) -> Self
    {
        Self {
            frag,
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }

    /// Parse a binary expression in source code.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        unimplemented!()
    }
}

impl<'s> HasFragment<'s> for BinaryExpression<'s> {
    fn get_fragment(&self) -> Fragment<'s> {self.frag}
}

impl<'s> ToExpression<'s> for BinaryExpression<'s> {
    fn create_expr(self) -> Expression<'s> {Expression::BinaryExpression(self)}
}