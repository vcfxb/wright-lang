use crate::grammar::ast::eq::{ast_eq, AstEq};
use crate::grammar::ast::{ExpressionStatement, Statement};
use crate::grammar::model::{Fragment, HasSourceReference};
use nom::IResult;
use std::mem::discriminant;
use crate::grammar::tracing::input::OptionallyTraceable;
use crate::grammar::tracing::parsers::map::map;
use crate::grammar::tracing::trace_result;

/// Expression statement parser.
pub(crate) mod expression_statement;

impl<T> Statement<T> {
    /// Name that appears in parse traces.
    pub const TRACE_NAME: &'static str = "Statement";

    /// Statement terminator.
    pub const TERMINATOR: char = ';';
}

impl<I: OptionallyTraceable> Statement<I> {
    /// Parses any statement.
    pub fn parse(input: I) -> IResult<I, Statement<I>> {
        trace_result(
            Self::TRACE_NAME,
            map(
                ExpressionStatement::parse,
                Statement::ExpressionStatement
            )(input.trace_start_clone(Self::TRACE_NAME))
        )
    }
}

impl<I> AstEq for Statement<I> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        use Statement::*;
        // discriminant is a function from std::mem
        // (https://doc.rust-lang.org/std/mem/fn.discriminant.html)
        // that returns an opaque type represents which variant of an enum
        // a value uses.
        // this check allows us to return `unreachable!()` at the bottom of
        // the match block instead of false. This will help us to catch bugs at
        // testing time.
        if discriminant(fst) != discriminant(snd) {
            return false;
        }

        match (fst, snd) {
            (ExpressionStatement(a), ExpressionStatement(b)) => ast_eq(a, b),
            _ => unimplemented!(),
        }
    }
}

impl<I> HasSourceReference<I> for Statement<I> {
    fn get_source_ref(&self) -> &I {
        use Statement::*;
        match self {
            ExpressionStatement(s) => s.get_source_ref(),
        }
    }
}
