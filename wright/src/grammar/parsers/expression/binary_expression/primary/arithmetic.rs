use crate::grammar::model::Fragment;
use nom::IResult;
use crate::grammar::ast::Expression;
use crate::grammar::parsers::expression::binary_expression::primary::{parser_left, base_primary};
use crate::grammar::parsers::expression::binary_expression::operator::{parse_arithmetic_operator1, parse_arithmetic_operator2};
use nom::branch::alt;

/// Parse child of a lower precedence arithmetic operator.
pub fn arithmetic1_primary(input: Fragment) -> IResult<Fragment, Expression> {
    alt((arithmetic2, base_primary))(input)
}

/// Parse lower precedence arithmetic expression.
pub fn arithmetic1(input: Fragment) -> IResult<Fragment, Expression> {
    parser_left(arithmetic1_primary, parse_arithmetic_operator1)(input)
}

/// Parse higher precedence arithmetic expression.
pub fn arithmetic2(input: Fragment) -> IResult<Fragment, Expression> {
    parser_left(base_primary, parse_arithmetic_operator2)(input)
}