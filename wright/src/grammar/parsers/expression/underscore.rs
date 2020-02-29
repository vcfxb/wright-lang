use crate::grammar::ast::{Expression, Underscore};
use crate::grammar::model::{Fragment, HasFragment};
use crate::grammar::parsers::expression::ToExpression;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;

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
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}

impl<'s> ToExpression<'s> for Underscore<'s> {
    fn create_expr(self) -> Expression<'s> {
        Expression::Underscore(self)
    }
    fn parse_self(f:Fragment<'s>) -> IResult<Fragment<'s>, Self> {Self::parse(f)}
}
