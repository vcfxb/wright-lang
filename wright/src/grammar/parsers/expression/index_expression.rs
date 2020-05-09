use crate::grammar::ast::eq::AstEq;
use crate::grammar::ast::{Expression, IndexExpression};
use crate::grammar::model::{Fragment, HasSourceReference};
use crate::grammar::parsers::whitespace::token_delimiter;
use crate::grammar::parsers::with_input;
use nom::character::complete::char as ch;
use nom::sequence::{delimited, pair, terminated};
use nom::IResult;
use crate::grammar::tracing::{
    input::OptionallyTraceable,
    parsers::map::map,
    trace_result
};

impl<T> IndexExpression<T> {
    /// Name used in parse traces.
    pub const TRACE_NAME: &'static str = "IndexExpression";

    /// Square brace characters.
    pub const DELIMITERS: (char, char) = ('[', ']');
}

impl<I: OptionallyTraceable> IndexExpression<I> {
    /// Parse an index expression in wright source code.
    pub fn parse(input: I) -> IResult<I, Self> {
        trace_result(Self::TRACE_NAME, map(
            with_input(pair(
                terminated(
                    Expression::parse,
                    pair(token_delimiter, ch(Self::DELIMITERS.0)),
                ),
                terminated(
                    Expression::parse,
                    delimited(token_delimiter, ch(Self::DELIMITERS.1), token_delimiter),
                ),
            )),
            move |(consumed, (subject, object))| Self {
                source: consumed,
                subject: Box::new(subject),
                object: Box::new(object),
            },
        )(input.trace_start_clone(Self::TRACE_NAME)))
    }
}

impl<I> HasSourceReference<I> for IndexExpression<I> {
    fn get_source_ref(&self) -> &I {
        &self.source
    }
}

impl<I> AstEq for IndexExpression<I> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        AstEq::ast_eq(&*fst.subject, &*snd.subject) && AstEq::ast_eq(&*fst.object, &*snd.object)
    }
}

impl<I> Into<Expression<I>> for IndexExpression<I> {
    fn into(self) -> Expression<I> {
        Expression::IndexExpression(self)
    }
}
