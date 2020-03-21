use crate::grammar::ast::{UnaryExpression, Expression, UnaryOp};
use crate::grammar::model::{Fragment, HasFragment};
use crate::grammar::parsers::expression::ToExpression;

impl<'s> UnaryExpression<'s> {

    pub fn parse(input: Fragment<'s>) -> Self {

    }
}

impl<'s> ToExpression<'s> for UnaryExpression<'s> {
    fn create_expr(self) -> Expression<'s> {
        Expression::UnaryExpression(self)
    }
}

impl<'s> HasFragment<'s> for UnaryExpression<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}
