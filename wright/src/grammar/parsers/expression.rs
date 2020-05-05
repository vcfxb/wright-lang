use std::mem::discriminant;

/// Binary expression parser and utilities.
pub(self) mod binary_expression;
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

/// Block parser.
pub(crate) mod block;
#[cfg(test)]
mod block_tests;

#[cfg(test)]
mod expression_tests;

use crate::grammar::ast::{eq::AstEq, BinaryExpression, Expression};
use crate::grammar::model::{Fragment, HasFragment};
use nom::branch::alt;
use nom::IResult;
use crate::grammar::ast::eq::ast_eq;

impl<'s> Expression<'s> {
    /// Parse an expression
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        alt((BinaryExpression::parse, binary_expression::base_primary))(input)
    }
}

impl<'s> HasFragment<'s> for Expression<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        use Expression::*;
        match self {
            NumLit(i) => i.get_fragment(),
            CharLit(i) => i.get_fragment(),
            StringLit(i) => i.get_fragment(),
            BooleanLit(i) => i.get_fragment(),
            Name(i) => i.get_fragment(),
            Parens(i) => i.get_fragment(),
            BinaryExpression(i) => i.get_fragment(),
            SelfLit(i) => i.get_fragment(),
            Block(i) => i.get_fragment(),
            UnaryExpression(i) => i.get_fragment(),
            Conditional(i) => i.get_fragment(),
            IndexExpression(i) => i.get_fragment(),
            FuncCall(i) => i.get_fragment(),
        }
    }
}

impl<'s> AstEq for Expression<'s> {
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
            (Name(a), Name(b)) => ast_eq(a, b),
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
