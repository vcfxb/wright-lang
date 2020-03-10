use crate::grammar::ast::Conditional;
use crate::grammar::model::{Fragment, HasFragment};

impl<'s> HasFragment<'s> for Conditional<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}
