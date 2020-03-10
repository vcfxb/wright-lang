use crate::grammar::model::{HasFragment, Fragment};
use crate::grammar::ast::Conditional;

impl<'s> HasFragment<'s> for Conditional<'s> {
    fn get_fragment(&self) -> Fragment<'s> {self.frag}
}