use crate::grammar::ast::{Parens, Expression};
use crate::grammar::model::{HasFragment, Fragment};
use nom::IResult;
use nom::combinator::{map, recognize};
use nom::sequence::delimited;
use nom::character::complete::{
    space0, char as ch
};

impl<'s> Parens<'s> {
    fn new(frag: Fragment<'s>, inner: Box<Expression<'s>>) -> Self {
        Self {frag, inner}
    }

    fn inner(frag: Fragment<'s>) -> IResult<Fragment<'s>, Expression<'s>> {
        delimited(
            space0, // change to general whitespace
            delimited(ch('('), Expression::parse, ch(')')),
            space0, // change to general whitespace
        )(frag)
    }

    /// Parse parentheses and the expression between them in source code. Will
    /// ignore any whitespace before and after.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(
            recognize(Self::inner),
            |parse| {
                Parens {
                    frag: parse,
                    inner: Box::new(Self::inner(parse).unwrap().1),
                }
            },
        )(input)
    }
}

impl<'s> HasFragment<'s> for Parens<'s> {
    fn get_fragment(&self) -> Fragment<'s> {self.frag}
}