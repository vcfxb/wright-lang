use crate::grammar::ast::BooleanLit;
use crate::grammar::model::Fragment;
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

    fn parser_inner(inp: Fragment<'s>) -> IResult<Fragment<'s>, bool> {
        alt((value(true, tag(Self::TRUE)), value(false, tag(Self::FALSE))))(inp)
    }

    /// Parses a boolean literal from wright source code.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(recognize(Self::parser_inner), |fr: Fragment<'s>| {
            Self::new(fr, Self::parser_inner(fr).unwrap().1)
        })(input)
    }
}
