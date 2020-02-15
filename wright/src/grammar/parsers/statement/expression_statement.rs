use crate::grammar::ast::{Expression, ExpressionSt};
use crate::grammar::model::{HasFragment, Fragment};
use nom::IResult;
use nom::character::complete::{char as ch, multispace0};
use nom::combinator::{map, recognize};
use nom::sequence::{preceded, terminated};

impl<'s> ExpressionSt<'s> {
    fn inner(frag: Fragment<'s>) -> IResult<Fragment<'s>, Expression<'s>> {
        terminated(
            Expression::parse,
            preceded(
                multispace0,
                ch(';'),
            ),
        )(frag)
    }
    
    /// Matches an expression followed by any amount of whitespace, then a semicolon.
    pub fn parse(frag: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(
            recognize(Self::inner),
            |parse| {
                ExpressionSt {
                    frag: parse,
                    inner: Self::inner(parse).unwrap().1,
                }
            }
        )(frag)
    }
}

impl<'s> HasFragment<'s> for ExpressionSt<'s> {
    fn get_fragment(&self) -> Fragment<'s> {self.frag}
}
