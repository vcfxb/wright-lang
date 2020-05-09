use crate::grammar::ast::{eq::AstEq, Expression, SelfLit};
use crate::grammar::model::{Fragment, HasSourceReference};
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;

impl<'s> SelfLit<'s> {
    /// Literal self identifier.
    pub const SELF: &'static str = "self";

    fn new(f: Fragment<'s>) -> Self {
        Self { frag: f }
    }

    /// Parse a self literal from input.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(tag(Self::SELF), Self::new)(input)
    }
}

impl<'s> Into<Expression<'s>> for SelfLit<'s> {
    fn into(self) -> Expression<'s> {
        Expression::SelfLit(self)
    }
}

impl<'s> HasSourceReference<'s> for SelfLit<'s> {
    fn get_source_ref(&self) -> &Fragment<'s> {
        &self.frag
    }
}

impl<'s> AstEq for SelfLit<'s> {
    #[inline]
    fn ast_eq(_: &Self, _: &Self) -> bool {
        true
    }
}
