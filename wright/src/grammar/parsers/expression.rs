/// Wright identifier parser.
pub(crate) mod identifier;

/// Underscore expression parser.
pub(crate) mod underscore;

/// Binary Expression parser.
pub(crate) mod binary_expression;

/// Parentheses parser.
pub(crate) mod parens;

#[cfg(test)]
mod expression_tests;

use crate::grammar::ast::{
    BinaryExpression, BinaryOp, BooleanLit, CharLit, Expression, Identifier, NumLit, Parens,
    StringLit, Underscore,
};
use crate::grammar::model::{Fragment, HasFragment};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::combinator::opt;
use nom::error::context;
use nom::multi::fold_many0;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::IResult;

impl<'s> Expression<'s> {
    /// Parse an expression
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        todo!("Expression::parse is unimplemented")
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
        }
    }
}

/// Trait implemented by all members of the
/// `Expression` node in an AST.
pub(crate) trait ToExpression<'s> {
    /// Construct an `Expression` from this object.
    fn create_expr(self) -> Expression<'s>;
}
