/// Wright identifier parser.
pub(crate) mod identifier;

/// Underscore expression parser.
pub(crate) mod underscore;

/// Binary Expression parser.
pub(crate) mod binary_expression;

/// Parentheses parser.
pub(crate) mod parens;

/// Block parser.
pub(crate) mod block;

#[cfg(test)]
mod expression_tests;

#[cfg(test)]
mod binary_expression_tests;

#[cfg(test)]
mod block_tests;

use crate::grammar::ast::{
    BinaryExpression, Block, BooleanLit, CharLit, Expression, Identifier, NumLit, Parens,
    StringLit, Underscore,
};
use crate::grammar::model::{Fragment, HasFragment};
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;

impl<'s> Expression<'s> {
    /// Parse a primary expression. A primary expression may always
    /// be the leading or trailing part of a binary expression.
    pub fn primary(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        alt((
            Block::expr_parse,
            Parens::expr_parse,
            NumLit::expr_parse,
            CharLit::expr_parse,
            StringLit::expr_parse,
            BooleanLit::expr_parse,
            Underscore::expr_parse,
            Identifier::expr_parse,
        ))(input)
    }

    /// Parse an expression.
    #[rustfmt::skip] // allow easy addition of new expressions
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        alt((
            BinaryExpression::expr_parse,
            Expression::primary,
        ))(input)
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
            Identifier(i) => i.get_fragment(),
            Underscore(i) => i.get_fragment(),
            Parens(i) => i.get_fragment(),
            BinaryExpression(i) => i.get_fragment(),
            Block(i) => i.get_fragment(),
        }
    }
}

/// Trait implemented by all members of the
/// `Expression` node in an AST.
pub(crate) trait ToExpression<'s>: Sized {
    /// Construct an `Expression` from this object.
    fn create_expr(self) -> Expression<'s>;

    /// Parse self from input. Generally just an alias to a public `parse`
    /// function.
    fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self>;

    /// Parse self from input and immediately convert to an `Expression`.
    fn expr_parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Expression<'s>> {
        map(Self::parse, Self::create_expr)(input)
    }
}
