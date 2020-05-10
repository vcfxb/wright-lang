use crate::grammar::ast::eq::ast_eq;
use crate::grammar::ast::{eq::AstEq, BinaryExpression, Expression};
use crate::grammar::model::{HasSourceReference, WrightInput};
use crate::grammar::tracing::trace_result;
use crate::grammar::tracing::parsers::alt;
use nom::IResult;
use std::mem::discriminant;
use crate::grammar::parsers::expression::binary_expression::primary::parse_expr;

/// Binary expression parser and utilities.
pub mod binary_expression;
#[cfg(test)]
mod binary_expression_tests;

/// Unary expression parser.
pub(self) mod unary_expression;

/// Index expression parsers.
pub(self) mod index_expression;

/// Function call expression parser.
pub(self) mod func_call;
#[cfg(test)]
mod func_call_tests;

/// Conditional expression parsers.
pub(crate) mod conditional;

/// Parentheses parser.
pub(crate) mod parens;
#[cfg(test)]
mod parens;

/// Block parser.
pub(crate) mod block;
#[cfg(test)]
mod block_tests;

#[cfg(test)]
mod expression_tests;

impl<T: Clone + std::fmt::Debug> Expression<T> {
    /// The name of this parser when appearing in traces.
    pub const TRACE_NAME: &'static str = "Expression";
}

impl<I: WrightInput> Expression<I> {
    /// Parse an expression in source code.
    pub fn parse(input: I) -> IResult<I, Self> {
        trace_result(
            Self::TRACE_NAME,
            parse_expr(input.trace_start_clone(Self::TRACE_NAME)),
        )
    }
}

impl<I: std::fmt::Debug + Clone> HasSourceReference<I> for Expression<I> {
    fn get_source_ref(&self) -> &I {
        use Expression::*;
        match self {
            NumLit(i) => i.get_source_ref(),
            CharLit(i) => i.get_source_ref(),
            StringLit(i) => i.get_source_ref(),
            BooleanLit(i) => i.get_source_ref(),
            ScopedName(i) => i.get_source_ref(),
            Parens(i) => i.get_source_ref(),
            BinaryExpression(i) => i.get_source_ref(),
            SelfLit(i) => i.get_source_ref(),
            Block(i) => i.get_source_ref(),
            UnaryExpression(i) => i.get_source_ref(),
            Conditional(i) => i.get_source_ref(),
            IndexExpression(i) => i.get_source_ref(),
            FuncCall(i) => i.get_source_ref(),
        }
    }
}

impl<T: Clone + std::fmt::Debug + PartialEq> AstEq for Expression<T> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        use Expression::*;

        // discriminant is a function from std::mem
        // (https://doc.rust-lang.org/std/mem/fn.discriminant.html)
        // that returns an opaque type represents which variant of an enum
        // a value uses.
        // this check allows us to return `unimplemented!()` at the bottom of
        // the match block instead of false. This will help us to catch bugs at
        // testing time.
        if discriminant(fst) != discriminant(snd) {
            return false;
        }

        match (fst, snd) {
            (NumLit(a), NumLit(b)) => ast_eq(a, b),
            (CharLit(a), CharLit(b)) => ast_eq(a, b),
            (StringLit(a), StringLit(b)) => ast_eq(a, b),
            (BooleanLit(a), BooleanLit(b)) => ast_eq(a, b),
            (ScopedName(a), ScopedName(b)) => ast_eq(a, b),
            (Parens(a), Parens(b)) => ast_eq(a, b),
            (BinaryExpression(a), BinaryExpression(b)) => ast_eq(a, b),
            (SelfLit(a), SelfLit(b)) => ast_eq(a, b),
            (Block(a), Block(b)) => ast_eq(a, b),
            (UnaryExpression(a), UnaryExpression(b)) => ast_eq(a, b),
            (Conditional(a), Conditional(b)) => ast_eq(a, b),
            (IndexExpression(a), IndexExpression(b)) => ast_eq(a, b),
            (_, _) => unimplemented!(),
        }
    }
}
