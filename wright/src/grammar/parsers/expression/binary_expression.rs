use crate::grammar::model::{HasFragment, Fragment};
use crate::grammar::ast::{BinaryExpression, Expression};
use crate::grammar::parsers::expression::ToExpression;

impl<'s> BinaryExpression<'s> {

}

impl<'s> HasFragment<'s> for BinaryExpression<'s> {
    fn get_fragment(&self) -> Fragment<'s> {self.frag}
}

impl<'s> ToExpression<'s> for BinaryExpression<'s> {
    fn create_expr(self) -> Expression<'s> {Expression::BinaryExpression(self)}
}