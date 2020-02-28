use crate::grammar::ast::NumLit;
use crate::grammar::ast::NumLitPattern;
use crate::grammar::ast::Pattern;
use crate::grammar::model::Fragment;
use crate::grammar::model::HasFragment;

use nom::character::complete::char;
use nom::combinator::map;
use nom::combinator::opt;
use nom::sequence::pair;
use nom::IResult;

impl<'s> NumLitPattern<'s> {
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(pair(opt(char('-')), NumLit::parse), |(neg, inner)| {
            NumLitPattern {
                negative: neg.is_some(),
                inner,
            }
        })(input)
    }
}

impl<'s> HasFragment<'s> for NumLitPattern<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.inner.frag
    }
}
