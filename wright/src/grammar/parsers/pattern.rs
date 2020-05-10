use crate::grammar::ast::eq::{ast_eq, AstEq};
use crate::grammar::ast::BooleanLit;
use crate::grammar::ast::CharLit;
use crate::grammar::ast::Identifier;
use crate::grammar::ast::NumLitPattern;
use crate::grammar::ast::Pattern;
use crate::grammar::ast::StringLit;
use crate::grammar::ast::Underscore;
use crate::grammar::model::{HasSourceReference, WrightInput};
use crate::grammar::tracing::{parsers::map::map, trace_result};

use std::mem::discriminant;

use nom::branch::alt;
use nom::IResult;
use std::fmt::Debug;

/// Underscore Pattern.
pub(crate) mod underscore;

/// Numerical literal pattern.
mod num_lit;

impl<T: Debug + Clone> Pattern<T> {
    /// The name of this parser that appears in tracing.
    pub const TRACE_NAME: &'static str = "Pattern";
}

impl<I: WrightInput> Pattern<I> {
    fn parse_num_lit(input: I) -> IResult<I, Self> {
        map(NumLitPattern::parse, Pattern::NumLit)(input)
    }

    fn parse_char_lit(input: I) -> IResult<I, Self> {
        map(CharLit::parse, Pattern::CharLit)(input)
    }

    fn parse_string_lit(input: I) -> IResult<I, Self> {
        map(StringLit::parse, Pattern::StringLit)(input)
    }

    fn parse_boolean_lit(input: I) -> IResult<I, Self> {
        map(BooleanLit::parse, Pattern::BooleanLit)(input)
    }

    fn parse_identifier(input: I) -> IResult<I, Self> {
        map(Identifier::parse, Pattern::Identifier)(input)
    }

    fn parse_underscore(input: I) -> IResult<I, Self> {
        map(Underscore::parse, Pattern::Underscore)(input)
    }

    /// Parse a pattern
    pub fn parse(input: I) -> IResult<I, Self> {
        trace_result(
            Self::TRACE_NAME,
            alt((
                Self::parse_num_lit,
                Self::parse_char_lit,
                Self::parse_string_lit,
                Self::parse_boolean_lit,
                Self::parse_identifier,
                Self::parse_underscore,
            ))(input.trace_start_clone(Self::TRACE_NAME)),
        )
    }
}

impl<I: Debug + Clone> HasSourceReference<I> for Pattern<I> {
    fn get_source_ref(&self) -> &I {
        use Pattern::*;
        match self {
            NumLit(p) => p.get_source_ref(),
            CharLit(p) => p.get_source_ref(),
            StringLit(p) => p.get_source_ref(),
            BooleanLit(p) => p.get_source_ref(),
            Identifier(p) => p.get_source_ref(),
            Underscore(p) => p.get_source_ref(),
            ScopedName(p) => p.get_source_ref(),
        }
    }
}

impl<I: Debug + Clone + PartialEq> AstEq for Pattern<I> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        if discriminant(fst) != discriminant(snd) {
            return false;
        }
        use Pattern::*;
        match (fst, snd) {
            (Underscore(a), Underscore(b)) => ast_eq(a, b),
            (NumLit(a), NumLit(b)) => ast_eq(a, b),
            (CharLit(a), CharLit(b)) => ast_eq(a, b),
            (StringLit(a), StringLit(b)) => ast_eq(a, b),
            (BooleanLit(a), BooleanLit(b)) => ast_eq(a, b),
            (Identifier(a), Identifier(b)) => ast_eq(a, b),
            _ => unreachable!(),
        }
    }
}
