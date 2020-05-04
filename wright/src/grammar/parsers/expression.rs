use std::mem::discriminant;

/// Binary expression parser and utilities.
pub mod binary_expression;

/// Unary expression parser.
pub(crate) mod unary_expression;

/// Index expression parsers.
pub(crate) mod index_expression;

/// Function call expression parser.
pub(crate) mod func_call;
// TODO: uncomment this when expression parser is done (it hits a `todo!()` otherwise)
//#[cfg(test)]
//mod func_call_tests;

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
use nom::IResult;

impl<'s> Expression<'s> {
    /// Parse an expression
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        // temporary: call BinaryExpression::parse when implemented
        BinaryExpression::primary(input)
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

/// Trait implemented by all members of the
/// `Expression` node in an AST.
pub(crate) trait ToExpression<'s>: HasFragment<'s> + AstEq {
    /// Construct an `Expression` from this object.
    fn create_expr(self) -> Expression<'s>;
}

impl<'s> ToExpression<'s> for Expression<'s> {
    #[inline]
    fn create_expr(self) -> Expression<'s> {
        self
    }
}

impl<'s> AstEq for Expression<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        use Expression::*;
        // shorthand fn
        fn aeq<T: AstEq>(a: T, b: T) -> bool {
            AstEq::ast_eq(&a, &b)
        }

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
            (NumLit(a), NumLit(b)) => aeq(a, b),
            (CharLit(a), CharLit(b)) => aeq(a, b),
            (StringLit(a), StringLit(b)) => aeq(a, b),
            (BooleanLit(a), BooleanLit(b)) => aeq(a, b),
            (Name(a), Name(b)) => aeq(a, b),
            (Parens(a), Parens(b)) => aeq(a, b),
            (BinaryExpression(a), BinaryExpression(b)) => aeq(a, b),
            (SelfLit(a), SelfLit(b)) => aeq(a, b),
            (Block(a), Block(b)) => aeq(a, b),
            (UnaryExpression(a), UnaryExpression(b)) => aeq(a, b),
            (Conditional(a), Conditional(b)) => aeq(a, b),
            (IndexExpression(a), IndexExpression(b)) => aeq(a, b),
            (_, _) => unimplemented!(),
        }
    }
}
