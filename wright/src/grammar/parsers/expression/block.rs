use crate::grammar::ast::{eq::AstEq, Block, Expression, Statement};
use crate::grammar::model::HasSourceReference;
use crate::grammar::parsers::whitespace::token_delimiter;
use crate::grammar::parsers::with_input;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::{delimited, pair, terminated};
use nom::IResult;
use crate::grammar::tracing::input::OptionallyTraceable;
use crate::grammar::tracing::parsers::map::map;
use crate::grammar::tracing::trace_result;

impl<T: Clone + std::fmt::Debug> Block<T> {
    /// Name that appears in traces.
    pub const TRACE_NAME: &'static str = "Block";

    /// Start of block in source code.
    pub const START_DELIMITER: &'static str = "{";

    /// End of a block in source code.
    pub const END_DELIMITER: &'static str = "}";
}

impl<I: OptionallyTraceable + std::fmt::Debug + Clone> Block<I> {
    fn inner(input: I) -> IResult<I, (Vec<Statement<I>>, Option<Box<Expression<I>>>)> {
        pair(
            many0(terminated(Statement::parse, token_delimiter)),
            opt(map(Expression::parse, |e| Box::new(e))),
        )(input)
    }

    /// Parse a block in source code. A block is delimited by `{` and `}`.
    /// Blocks contain a series of statements, and optionally an expression at
    /// the end. There may be no statements, in which case it is considered to
    /// be a series of length 0.
    pub fn parse(input: I) -> IResult<I, Self> {
        trace_result(Self::TRACE_NAME, map(
            with_input(delimited(
                pair(tag(Self::START_DELIMITER), token_delimiter),
                Self::inner,
                pair(token_delimiter, tag(Self::END_DELIMITER)),
            )),
            |(consumed, (statements, terminal))| Self {
                frag: consumed,
                statements,
                result: terminal,
            },
        )(input.trace_start_clone(Self::TRACE_NAME)))
    }
}

impl<I: std::fmt::Debug + Clone> HasSourceReference<I> for Block<I> {
    fn get_source_ref(&self) -> &I {
        &self.frag
    }
}

impl<I: std::fmt::Debug + Clone> Into<Expression<I>> for Block<I> {
    fn into(self) -> Expression<I> {
        Expression::Block(self)
    }
}

impl<I: Clone + std::fmt::Debug> AstEq for Block<I> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        AstEq::ast_eq(&fst.result, &snd.result) &&
            AstEq::ast_eq(&fst.statements, &snd.statements)
    }
}
