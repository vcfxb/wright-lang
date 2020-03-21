use crate::grammar::ast::{Expression, SelfLit, ASTEq};
use crate::grammar::model::{Fragment, HasFragment};
use crate::grammar::parsers::expression::ToExpression;
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

impl<'s> ToExpression<'s> for SelfLit<'s> {
    fn create_expr(self) -> Expression<'s> {
        Expression::SelfLit(self)
    }
}

impl<'s> HasFragment<'s> for SelfLit<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}

impl<'s> ASTEq for SelfLit<'s> {
    #[inline]
    fn ast_eq(fst: &Self, snd: &Self) -> bool {true}
}