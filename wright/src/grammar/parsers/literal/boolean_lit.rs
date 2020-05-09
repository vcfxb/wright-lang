use crate::grammar::ast::{eq::AstEq, BooleanLit, Expression};
use crate::grammar::model::{Fragment, HasSourceReference};
use crate::grammar::parsers::with_input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, value};
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

    fn parser_inner(inp: Fragment<'s>) -> IResult<Fragment<'s>, bool> {
        alt((value(true, tag(Self::TRUE)), value(false, tag(Self::FALSE))))(inp)
    }

    /// Parses a boolean literal from wright source code.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(with_input(Self::parser_inner), |(fr, v)| Self::new(fr, v))(input)
    }
}

impl<'s> HasSourceReference<'s> for BooleanLit<'s> {
    fn get_source_ref(&self) -> &Fragment<'s> {
        &self.frag
    }
}

impl<'s> Into<Expression<'s>> for BooleanLit<'s> {
    fn into(self) -> Expression<'s> {
        Expression::BooleanLit(self)
    }
}

impl<'s> AstEq for BooleanLit<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        fst.inner == snd.inner
    }
}
