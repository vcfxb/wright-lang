use crate::grammar::ast::{eq::AstEq, Expression, Parens};
use crate::grammar::model::{Fragment, HasSourceReference};
use crate::grammar::parsers::whitespace::token_delimiter;
use crate::grammar::parsers::with_input;
use nom::character::complete::char as ch;
use nom::sequence::{delimited, terminated};
use nom::IResult;
use crate::grammar::tracing::input::OptionallyTraceable;
use crate::grammar::tracing::parsers::map::map;
use crate::grammar::tracing::trace_result;

impl<T: Clone + std::fmt::Debug> Parens<T> {
    /// Name that appears in parse traces.
    pub const TRACE_NAME: &'static str = "";
}

impl<I: OptionallyTraceable + std::fmt::Debug + Clone> Parens<I> {
    fn inner(input: I) -> IResult<I, Expression<I>> {
        delimited(
            token_delimiter,
            delimited(
                terminated(ch('('), token_delimiter),
                Expression::parse,
                terminated(ch(')'), token_delimiter),
            ),
            token_delimiter,
        )(input)
    }

    /// Parse parentheses and the expression between them in source code. Will
    /// ignore any whitespace before and after.
    pub fn parse(input: I) -> IResult<I, Self> {
        trace_result(
            Self::TRACE_NAME,
            map(with_input(Self::inner), |(consumed, expr)| Parens {
                source: consumed,
                inner: Box::new(expr),
            })(input.trace_start_clone(Self::TRACE_NAME))
        )
    }
}

impl<I: std::fmt::Debug + Clone> HasSourceReference<I> for Parens<I> {
    fn get_source_ref(&self) -> &I {
        &self.source
    }
}

impl<I: std::fmt::Debug + Clone> Into<Expression<I>> for Parens<I> {
    fn into(self) -> Expression<I> {
        Expression::Parens(self)
    }
}

impl<I: Clone + std::fmt::Debug> AstEq for Parens<I> {
    #[inline]
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        AstEq::ast_eq(&*fst.inner, &*snd.inner)
    }
}
