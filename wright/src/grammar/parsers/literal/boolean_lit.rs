use crate::grammar::ast::{eq::AstEq, BooleanLit, Expression};
use crate::grammar::model::{HasSourceReference, WrightInput};
use crate::grammar::parsers::with_input;
use crate::grammar::tracing::{parsers::map, trace_result};
use nom::branch::alt;
use crate::grammar::tracing::parsers::tag;
use nom::combinator::value;
use nom::IResult;

impl<T: std::fmt::Debug + Clone> BooleanLit<T> {
    /// Literal representing a true value.
    pub const TRUE: &'static str = "true";

    /// Literal representing a false value.
    pub const FALSE: &'static str = "false";

    /// The name of this parser when appearing in function traces.
    pub const TRACE_NAME: &'static str = "BooleanLit";
}

impl<I: WrightInput> BooleanLit<I> {
    fn new(source: I, val: bool) -> Self {
        Self { source, inner: val }
    }

    fn parser_inner(inp: I) -> IResult<I, bool> {
        alt((value(true, tag(Self::TRUE)), value(false, tag(Self::FALSE))))(inp)
    }

    /// Parses a boolean literal from wright source code.
    pub fn parse(input: I) -> IResult<I, Self> {
        let res = map(with_input(Self::parser_inner), |(consumed, v)| {
            Self::new(consumed, v)
        })(input.trace_start_clone(Self::TRACE_NAME));
        trace_result(Self::TRACE_NAME, res)
    }
}

impl<I: std::fmt::Debug + Clone> HasSourceReference<I> for BooleanLit<I> {
    fn get_source_ref(&self) -> &I {
        &self.source
    }
}

impl<I: std::fmt::Debug + Clone> Into<Expression<I>> for BooleanLit<I> {
    fn into(self) -> Expression<I> {
        Expression::BooleanLit(self)
    }
}

impl<I: std::fmt::Debug + Clone> AstEq for BooleanLit<I> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        fst.inner == snd.inner
    }
}
