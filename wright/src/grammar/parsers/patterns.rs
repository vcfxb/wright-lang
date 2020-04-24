mod num_lit;

use crate::grammar::ast::eq::AstEq;
use crate::grammar::ast::BooleanLit;
use crate::grammar::ast::CharLit;
use crate::grammar::ast::Identifier;
use crate::grammar::ast::NumLitPattern;
use crate::grammar::ast::Pattern;
use crate::grammar::ast::StringLit;
use crate::grammar::ast::Underscore;
use crate::grammar::model::{Fragment, HasFragment};

use std::mem::discriminant;

use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;
use crate::grammar::ast::ScopedName;

pub(crate) mod underscore;

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

    fn parse_scoped_name(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(ScopedName::parse, Pattern::ScopedName)(input)
    }

    /// Parse a pattern
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        alt((
            Self::parse_num_lit,
            Self::parse_char_lit,
            Self::parse_string_lit,
            Self::parse_boolean_lit,
            Self::parse_scoped_name, // must come before identifier due to precedence issues
            Self::parse_identifier,
            Self::parse_underscore,
        ))(input)
    }
}

impl<'s> HasFragment<'s> for Pattern<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        use Pattern::*;
        match self {
            NumLit(p) => p.get_fragment(),
            CharLit(p) => p.get_fragment(),
            StringLit(p) => p.get_fragment(),
            BooleanLit(p) => p.get_fragment(),
            Identifier(p) => p.get_fragment(),
            Underscore(p) => p.get_fragment(),
            ScopedName(p) => p.get_fragment(),
        }
    }
}

impl<'s> AstEq for Pattern<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        if discriminant(fst) != discriminant(snd) {
            return false;
        }

        // shorthand fn
        fn aeq<T: AstEq>(a: &T, b: &T) -> bool {
            AstEq::ast_eq(a, b)
        }

        use Pattern::*;
        match (fst, snd) {
            (Underscore(a), Underscore(b)) => aeq(a, b),
            (NumLit(a), NumLit(b)) => aeq(a, b),
            (CharLit(a), CharLit(b)) => aeq(a, b),
            (StringLit(a), StringLit(b)) => aeq(a, b),
            (BooleanLit(a), BooleanLit(b)) => aeq(a, b),
            (Identifier(a), Identifier(b)) => aeq(a, b),
            (ScopedName(a), ScopedName(b)) => aeq(a,b),
            _ => unimplemented!(),
        }
    }
}
