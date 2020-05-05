use crate::grammar::model::Fragment;
use nom::IResult;
use crate::grammar::ast::{Expression};
use nom::branch::alt;
use crate::grammar::parsers::expression::binary_expression::primary::{parser_left};
use crate::grammar::parsers::expression::binary_expression::operator::{parse_or, parse_xor, parse_and};
use crate::grammar::parsers::expression::binary_expression::primary::equality::{equality, equality_primary};

/// A child expression under a 'bitwise or' expression.
pub fn bitwise_or_primary(input: Fragment) -> IResult<Fragment, Expression> {
    alt((
        bitwise_xor,
        bitwise_xor_primary,
    ))(input)
}

/// A child expression under a 'bitwise xor' expression.
fn bitwise_xor_primary(input: Fragment) -> IResult<Fragment, Expression> {
    alt((
        bitwise_and,
        bitwise_and_primary
    ))(input)
}

/// A child expression under a 'bitwise or' expression.
fn bitwise_and_primary(input: Fragment) -> IResult<Fragment, Expression> {
    alt((
        equality,
        equality_primary
    ))(input)
}

/// Parse a 'bitwise or' binary expression.
pub fn bitwise_or(input: Fragment) -> IResult<Fragment, Expression> {
    parser_left(bitwise_or_primary, parse_or)(input)
}

/// Parse a 'bitwise xor' binary expression.
pub fn bitwise_xor(input: Fragment) -> IResult<Fragment, Expression> {
    parser_left(bitwise_xor_primary, parse_xor)(input)
}

/// Parse a 'bitwise and' binary expression.
pub fn bitwise_and(input: Fragment) -> IResult<Fragment, Expression> {
    parser_left(bitwise_and_primary, parse_and)(input)
}

