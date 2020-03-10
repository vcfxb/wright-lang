use crate::grammar::model::{HasFragment, Fragment};
use crate::grammar::ast::IndexExpression;

impl<'s> HasFragment<'s> for IndexExpression<'s> {
    fn get_fragment(&self) -> Fragment<'s> {self.frag}
}