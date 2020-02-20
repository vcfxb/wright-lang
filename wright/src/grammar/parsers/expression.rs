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
    BinaryExpression,
    Block,
    BooleanLit,
    CharLit,
    Expression,
    Identifier,
    NumLit,
    Parens,
    StringLit,
    Underscore,
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
            map(Block::parse, Expression::Block),
            map(Parens::parse, Expression::Parens),
            map(NumLit::parse, Expression::NumLit),
            map(CharLit::parse, Expression::CharLit),
            map(StringLit::parse, Expression::StringLit),
            map(BooleanLit::parse, Expression::BooleanLit),
            map(Underscore::parse, Expression::Underscore),
            map(Identifier::parse, Expression::Identifier),
        ))(input)
    }

    /// Parse an expression.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        alt((
            BinaryExpression::parse,
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
pub(crate) trait ToExpression<'s> {
    /// Construct an `Expression` from this object.
    fn create_expr(self) -> Expression<'s>;
}
