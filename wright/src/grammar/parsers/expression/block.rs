use crate::grammar::model::{HasFragment, Fragment};
use crate::grammar::ast::Block;

impl<'s> HasFragment<'s> for Block<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}