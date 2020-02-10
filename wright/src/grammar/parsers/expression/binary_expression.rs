use crate::grammar::model::{HasFragment, Fragment};
use crate::grammar::ast::BinaryExpression;

impl<'s> BinaryExpression<'s> {

}

impl<'s> HasFragment<'s> for BinaryExpression<'s> {
    fn get_fragment(&self) -> Fragment<'s> {self.frag}
}