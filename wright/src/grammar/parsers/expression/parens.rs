use crate::grammar::ast::{Parens, Expression};
use crate::grammar::model::{HasFragment, Fragment};

impl<'s> Parens<'s> {
    fn new(frag: Fragment<'s>, inner: Box<Expression<'s>>) -> Self {
        Self {frag, inner}
    }
}

impl<'s> HasFragment<'s> for Parens<'s> {
    fn get_fragment(&self) -> Fragment<'s> {self.frag}
}