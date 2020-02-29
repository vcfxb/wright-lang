use crate::grammar::ast::{BooleanLit, Expression};
use crate::grammar::model::{Fragment, HasFragment};
use crate::grammar::parsers::expression::ToExpression;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, recognize, value};
use nom::IResult;

impl<'s> BooleanLit<'s> {
    /// Literal representing a true value.
    pub const TRUE: &'static str = "true";

    /// Literal representing a false value.
    pub const FALSE: &'static str = "false";

    fn new(fr: Fragment<'s>, val: bool) -> Self {
        Self {
            frag: fr,
            inner: val,
        }
    }

    /// Parses a boolean literal from wright source code.
    fn parser_inner(inp: Fragment<'s>) -> IResult<Fragment<'s>, bool> {
        alt((value(true, tag(Self::TRUE)), value(false, tag(Self::FALSE))))(inp)
    }
}

impl<'s> HasFragment<'s> for BooleanLit<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}

impl<'s> ToExpression<'s> for BooleanLit<'s> {
    fn create_expr(self) -> Expression<'s> {
        Expression::BooleanLit(self)
    }

    fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(recognize(Self::parser_inner), |fr: Fragment<'s>| {
            Self::new(fr, Self::parser_inner(fr).unwrap().1)
        })(input)
    }
}
