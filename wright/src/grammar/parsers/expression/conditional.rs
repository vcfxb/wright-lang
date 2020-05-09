use crate::grammar::ast::{eq::AstEq, Block, Conditional, Expression};
use crate::grammar::model::{Fragment, HasSourceReference};
use crate::grammar::parsers::whitespace::token_delimiter;
use crate::grammar::parsers::with_input;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded, separated_pair, tuple};
use nom::IResult;
use crate::grammar::tracing::input::OptionallyTraceable;
use crate::grammar::tracing::parsers::map::map;
use crate::grammar::tracing::trace_result;

impl<T> Conditional<T> {
    /// Name that appears in parse traces.
    pub const TRACE_NAME: &'static str = "Conditional";

    /// `if` token in source code. This constant is unlikely to change.
    pub const IF: &'static str = "if";

    /// `else` token in source code. This constant is unlikely to change.
    pub const ELSE: &'static str = "else";
}

impl<I: OptionallyTraceable> Conditional<I> {
    // parse an expression followed by a block.
    fn parse_branch(input: I) -> IResult<I, (Expression<I>, Block<I>)> {
        separated_pair(Expression::parse, token_delimiter, Block::parse)(input)
    }

    // discards the output of the prefix (T is never used)
    fn parse_branch_prefixed<T>(
        prefix: impl Fn(I) -> IResult<I, T>,
    ) -> impl Fn(I) -> IResult<I, (Expression<I>, Block<I>)> {
        preceded(prefix, Self::parse_branch)
    }

    // parse the if branch of a conditional expression.
    fn parse_if(input: I) -> IResult<I, (Expression<I>, Block<I>)> {
        Self::parse_branch_prefixed(pair(tag(Self::IF), token_delimiter))(input)
    }

    // parse an `else if` branch of a conditional expression.
    fn parse_elif(input: I) -> IResult<I, (Expression<I>, Block<I>)> {
        Self::parse_branch_prefixed(pair(
            tag(Self::IF),
            delimited(token_delimiter, tag(Self::ELSE), token_delimiter),
        ))(input)
    }

    // parse all the elifs in a conditional statement
    fn parse_elifs(input: I) -> IResult<I, Vec<(Expression<I>, Block<I>)>> {
        many0(preceded(token_delimiter, Self::parse_elif))(input)
    }

    // parse the else branch.
    fn parse_else(input: I) -> IResult<I, Block<I>> {
        preceded(pair(tag(Self::ELSE), token_delimiter), Block::parse)(input)
    }

    /// Parse a conditional expression in source code.
    pub fn parse(input: I) -> IResult<I, Self> {
        trace_result(Self::TRACE_NAME, map(
            with_input(tuple((
                Self::parse_if,
                Self::parse_elifs,
                preceded(token_delimiter, opt(Self::parse_else)),
            ))),
            |(consumed, ((pa, pb), elifs, terminal))| Self {
                source: consumed,
                primary: (Box::new(pa), pb),
                elifs,
                default: terminal,
            },
        )(input.trace_start_clone(Self::TRACE_NAME)))
    }
}

impl<I> HasSourceReference<I> for Conditional<I> {
    fn get_source_ref(&self) -> &I {
        &self.source
    }
}

impl<I> AstEq for Conditional<I> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        AstEq::ast_eq(&fst.default, &snd.default)
            && AstEq::ast_eq(&fst.elifs, &snd.elifs)
            && AstEq::ast_eq(&fst.primary, &snd.primary)
    }
}

impl<I> Into<Expression<I>> for Conditional<I> {
    fn into(self) -> Expression<I> {
        Expression::Conditional(self)
    }
}
