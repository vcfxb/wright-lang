use crate::grammar::ast::eq::AstEq;
use crate::grammar::ast::NumLit;
use crate::grammar::ast::NumLitPattern;
use crate::grammar::model::{HasSourceReference, WrightInput};
use crate::grammar::parsers::with_input;
use nom::character::complete::char;
use nom::combinator::opt;
use nom::sequence::pair;
use nom::IResult;
use crate::grammar::tracing::{
    parsers::map::map,
    trace_result
};


impl<T: Clone + std::fmt::Debug> NumLitPattern<T> {
    /// Name of this parser when appearing in traces.
    pub const TRACE_NAME: &'static str = "NumLitPattern";
}

impl<I: WrightInput> NumLitPattern<I> {
    /// Parse a numerical literal pattern. (e.g. "-12", "4")
    pub fn parse(input: I) -> IResult<I, Self> {
        trace_result(Self::TRACE_NAME, map(
            with_input(pair(opt(char('-')), NumLit::parse)),
            |(consumed, (neg, inner))| NumLitPattern {
                source: consumed,
                negative: neg.is_some(),
                inner,
            },
        )(input.trace_start_clone(Self::TRACE_NAME)))
    }
}

impl<I: std::fmt::Debug + Clone> AstEq for NumLitPattern<I> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        fst.negative == snd.negative && NumLit::ast_eq(&fst.inner, &snd.inner)
    }
}

impl<I: std::fmt::Debug + Clone> HasSourceReference<I> for NumLitPattern<I> {
    fn get_source_ref(&self) -> &I {
        &self.source
    }
}
