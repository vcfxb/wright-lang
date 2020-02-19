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
mod block_tests;

use crate::grammar::ast::Expression;
use crate::grammar::model::{Fragment, HasFragment};

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
