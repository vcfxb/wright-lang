use crate::grammar::ast::IndexExpression;
use crate::grammar::model::{Fragment, HasFragment};

impl<'s> HasFragment<'s> for IndexExpression<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}
