use crate::grammar::ast::{Underscore, Expression};
use crate::grammar::model::{Fragment, HasFragment};
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;
use crate::grammar::parsers::expression::ToExpression;

impl<'s> Underscore<'s> {
    /// Underscore literal.
    pub const UNDERSCORE: &'static str = "_";

    fn new(frag: Fragment<'s>) -> Self {
        Self { frag }
    }

    /// Parse underscore.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(tag(Self::UNDERSCORE), Self::new)(input)
    }
}

impl<'s> HasFragment<'s> for Underscore<'s> {
    fn get_fragment(&self) -> Fragment<'s> {self.frag}
}

impl<'s> ToExpression<'s> for Underscore<'s> {
    fn create_expr(self) -> Expression<'s> {Expression::Underscore(self)}
}