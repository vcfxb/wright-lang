use crate::grammar::ast::UnaryExpression;
use crate::grammar::model::{Fragment, HasFragment};

impl<'s> HasFragment<'s> for UnaryExpression<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}
