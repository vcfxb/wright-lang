use crate::grammar::ast::Underscore;
use crate::grammar::model::Fragment;
use nom::IResult;
use nom::combinator::map;
use nom::bytes::complete::tag;

impl<'s> Underscore<'s> {
    /// Underscore literal.
    pub const UNDERSCORE: &'static str = "_";

    fn new(frag: Fragment<'s>) -> Self {
        Self {frag}
    }

    /// Parse underscore.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(tag(Self::UNDERSCORE), Self::new)(input)
    }
}