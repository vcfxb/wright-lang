use crate::grammar::ast::eq::AstEq;
use crate::grammar::ast::Underscore;
use crate::grammar::model::{Fragment, HasFragment};
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;

impl<'s> Underscore<'s> {
    /// The constant for an underscore literal in source code. Unlikely to change.
    pub const UNDERSCORE: &'static str = "_";

    /// Parse an underscore from source code.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(tag(Self::UNDERSCORE), |f| Self { frag: f })(input)
    }
}

impl<'s> HasFragment<'s> for Underscore<'s> {
    #[inline]
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}

impl<'s> AstEq for Underscore<'s> {
    fn ast_eq(_: &Self, _: &Self) -> bool {
        true
    }
}
