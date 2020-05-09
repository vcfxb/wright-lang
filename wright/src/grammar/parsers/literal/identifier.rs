use crate::grammar::ast::{
    eq::AstEq, BinaryOp, Conditional, SelfLit, Underscore,
};
use crate::grammar::ast::{BooleanLit, Identifier};
use crate::grammar::model::{HasSourceReference, WrightInput};
use nom::bytes::complete::take_while;
use nom::character::complete::anychar;
use nom::combinator::{recognize, verify};
use nom::error::context;
use nom::sequence::pair;
use nom::IResult;
use crate::grammar::tracing::{
    input::OptionallyTraceable,
    parsers::map::map,
    trace_result
};

impl<T: std::fmt::Debug + Clone> Identifier<T> {
    /// Name used in function tracing.
    pub const TRACE_NAME: &'static str = "Identifier";

    /// Reserved words that an identifier must not match.
    pub const RESERVED_WORDS: [&'static str; 8] = [
        BooleanLit::<T>::FALSE,
        BooleanLit::<T>::TRUE,
        SelfLit::<T>::SELF,
        Conditional::<T>::IF,
        Conditional::<T>::ELSE,
        Underscore::<T>::UNDERSCORE,
        BinaryOp::LOGICAL_AND,
        BinaryOp::LOGICAL_OR,
    ];
}

impl<'a, I: WrightInput<'a>> Identifier<I> {
    fn new(source: I) -> Self {
        Self { source }
    }

    fn raw_ident(input: I) -> IResult<I, I> {
        verify(
            recognize(pair(
                verify(anychar, |c| c.is_ascii_alphabetic() || *c == '_'),
                take_while(|c: char| c.is_ascii_alphanumeric() || c == '_'),
            )),
            |i: &I| {
                Self::RESERVED_WORDS
                    .iter()
                    .all(|s: &&str| i != s)
            },
        )(input)
    }

    /// Parse an identifier from source code. Identifiers may include
    /// ASCII alphanumerics and underscores, but must not start with a number.
    /// An Identifier also must not be a reserved word.
    pub fn parse(input: I) -> IResult<I, Self> {
        let i = input.trace_start_clone(Self::TRACE_NAME);
        let res =
            context("expected identifier", map(Self::raw_ident, Self::new))(i);
        trace_result(Self::TRACE_NAME, res)
    }
}

impl<I: std::fmt::Debug + Clone> HasSourceReference<I> for Identifier<I> {
    fn get_source_ref(&self) -> &I {
        &self.source
    }
}

impl<I: std::fmt::Debug + Clone + Into<String>> AstEq for Identifier<I> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        fst.source.into() == snd.source.into()
    }
}