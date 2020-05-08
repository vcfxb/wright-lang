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
//#[cfg(test)]
//mod block_tests;

#[cfg(test)]
mod expression_tests;

use crate::grammar::ast::eq::ast_eq;
use crate::grammar::ast::{eq::AstEq, BinaryExpression, Expression};
use crate::grammar::model::{Fragment, HasFragment};
use nom::branch::alt;
use nom::IResult;

impl<'s> Expression<'s> {
    /// Parse an expression
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        println!("Expr::parse");
        assert!(false);
        alt((BinaryExpression::parse, binary_expression::base_primary))(input)
    }
}

impl<'s> HasFragment<'s> for Expression<'s> {
    fn get_fragment_reference(&self) -> &Fragment<'s> {
        use Expression::*;
        match self {
            NumLit(i) => i.get_fragment_reference(),
            CharLit(i) => i.get_fragment_reference(),
            StringLit(i) => i.get_fragment_reference(),
            BooleanLit(i) => i.get_fragment_reference(),
            ScopedName(i) => i.get_fragment_reference(),
            Parens(i) => i.get_fragment_reference(),
            BinaryExpression(i) => i.get_fragment_reference(),
            SelfLit(i) => i.get_fragment_reference(),
            Block(i) => i.get_fragment_reference(),
            UnaryExpression(i) => i.get_fragment_reference(),
            Conditional(i) => i.get_fragment_reference(),
            IndexExpression(i) => i.get_fragment_reference(),
            FuncCall(i) => i.get_fragment_reference(),
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
