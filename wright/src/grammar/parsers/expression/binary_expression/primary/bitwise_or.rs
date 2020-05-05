use crate::grammar::model::Fragment;
use nom::IResult;
use crate::grammar::ast::{Expression, BinaryOp};
use nom::branch::alt;
use crate::grammar::parsers::expression::binary_expression::primary::{parser_left, base_primary};
use nom::combinator::value;
use nom::bytes::complete::tag;
use crate::grammar::parsers::expression::binary_expression::operator::parse_or;

// /// A child expression under a 'bitwise or' expression.
// pub fn bitwise_or_primary(input: Fragment) -> IResult<Fragment, Expression> {
//     alt((
//
//     ))(input)
// }

/// Parse a 'bitwise or' binary expression.
pub fn bitwise_or(input: Fragment) -> IResult<Fragment, Expression> {
    parser_left(base_primary, parse_or)(input)
}