mod num_lit;

use nom::branch::alt;
use nom::combinator::map;

use crate::grammar::ast::BooleanLit;
use crate::grammar::ast::CharLit;
use crate::grammar::ast::Identifier;
use crate::grammar::ast::NumLitPattern;
use crate::grammar::ast::ParensPattern;
use crate::grammar::ast::Pattern;
use crate::grammar::ast::StringLit;
use crate::grammar::ast::Underscore;
use crate::grammar::model::{Fragment, HasFragment};

use nom::IResult;

impl<'s> Pattern<'s> {
    fn parse_num_lit(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(NumLitPattern::parse, Pattern::NumLit)(input)
    }

    fn parse_char_lit(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(CharLit::parse, Pattern::CharLit)(input)
    }

    fn parse_string_lit(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(StringLit::parse, Pattern::StringLit)(input)
    }

    fn parse_boolean_lit(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(BooleanLit::parse, Pattern::BooleanLit)(input)
    }

    fn parse_identifier(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(Identifier::parse, Pattern::Identifier)(input)
    }

    fn parse_underscore(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(Underscore::parse, Pattern::Underscore)(input)
    }

    /// Parse a pattern
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        alt((
            Self::parse_num_lit,
            Self::parse_char_lit,
            Self::parse_string_lit,
            Self::parse_boolean_lit,
            Self::parse_identifier,
            Self::parse_underscore,
        ))(input)
    }
}

impl<'s> HasFragment<'s> for Pattern<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        use Pattern::*;
        match self {
            NumLit(i) => i.get_fragment(),
            CharLit(i) => i.get_fragment(),
            StringLit(i) => i.get_fragment(),
            BooleanLit(i) => i.get_fragment(),
            Identifier(i) => i.get_fragment(),
            Underscore(i) => i.get_fragment(),
        }
    }
}
