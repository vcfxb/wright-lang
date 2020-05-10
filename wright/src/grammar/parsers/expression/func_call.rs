use crate::grammar::ast::eq::AstEq;
use crate::grammar::ast::{Block, Expression, FuncCall, Parens, ScopedName};
use crate::grammar::model::{HasSourceReference, WrightInput};
use crate::grammar::parsers::whitespace::token_delimiter;
use crate::grammar::parsers::with_input;
use crate::grammar::tracing::parsers::map;
use crate::grammar::tracing::trace_result;
use crate::grammar::tracing::parsers::alt;
use nom::character::complete::char as ch;
use nom::multi::separated_list;
use nom::sequence::{delimited, pair, terminated};
use nom::IResult;

impl<T: Clone + std::fmt::Debug> FuncCall<T> {
    /// Name of this parser in parse traces.
    pub const TRACE_NAME: &'static str = "FuncCall";

    /// Left parenthesis delimiting argument list. Probably should never change.
    pub const DELIMITER_LEFT: char = '(';

    /// Left parenthesis delimiting argument list. Probably should never change.
    pub const DELIMITER_RIGHT: char = ')';

    /// Comma separating arguments. Probably should never change.
    pub const ARG_SEPARATOR: char = ',';
}

impl<I: WrightInput> FuncCall<I> {
    fn func_call_primary(input: I) -> IResult<I, Expression<I>> {
        todo!("recursion");
        alt((
            // map(IndexExpression::parse, Expression::IndexExpression),
            // commented out to avoid possible recursion.
            map(Block::parse, Block::into),
            map(Parens::parse, Parens::into),
            map(ScopedName::parse, ScopedName::into),
        ))(input)
    }

    /// Parse an index expression in wright source code.
    pub fn parse(input: I) -> IResult<I, Self> {
        trace_result(
            Self::TRACE_NAME,
            map(
                with_input(pair(
                    terminated(Self::func_call_primary, token_delimiter),
                    delimited(
                        ch(Self::DELIMITER_LEFT),
                        separated_list(
                            delimited(token_delimiter, ch(Self::ARG_SEPARATOR), token_delimiter),
                            Expression::parse,
                        ),
                        ch(Self::DELIMITER_RIGHT),
                    ),
                )),
                move |(consumed, (func, args))| Self {
                    source: consumed,
                    func: Box::new(func),
                    args,
                },
            )(input.trace_start_clone(Self::TRACE_NAME)),
        )
    }
}

impl<I: std::fmt::Debug + Clone> HasSourceReference<I> for FuncCall<I> {
    fn get_source_ref(&self) -> &I {
        &self.source
    }
}

impl<I: Clone + std::fmt::Debug + PartialEq> AstEq for FuncCall<I> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        AstEq::ast_eq(&fst.func, &snd.func) && AstEq::ast_eq(&fst.args, &snd.args)
    }
}

impl<I: std::fmt::Debug + Clone> Into<Expression<I>> for FuncCall<I> {
    fn into(self) -> Expression<I> {
        Expression::FuncCall(self)
    }
}
