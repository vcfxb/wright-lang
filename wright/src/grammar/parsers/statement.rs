/// Expression statement parser.
pub mod expression_statement;

// Tests

#[cfg(test)]
mod expression_statement_tests;

use crate::grammar::ast::{ExpressionSt, Statement};
use crate::grammar::model::Fragment;
use nom::combinator::map;
use nom::IResult;

impl<'s> Statement<'s> {
    /// Parse a statement of any type.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(ExpressionSt::parse, Statement::Expression)(input)
    }
}
