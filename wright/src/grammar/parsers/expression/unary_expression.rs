use crate::grammar::model::{HasFragment, Fragment};
use crate::grammar::ast::UnaryExpression;

impl<'s> HasFragment<'s> for UnaryExpression<'s> {
    fn get_fragment(&self) -> Fragment<'s> {self.frag}
}