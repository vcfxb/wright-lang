use crate::grammar::ast::Block;
use crate::grammar::model::{Fragment, HasFragment};

impl<'s> HasFragment<'s> for Block<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}
