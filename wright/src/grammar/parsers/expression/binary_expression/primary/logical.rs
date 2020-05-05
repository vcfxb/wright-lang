use crate::grammar::ast::Expression;
use crate::grammar::model::Fragment;
use crate::grammar::parsers::expression::binary_expression::operator::{
    parse_logical_and, parse_logical_or,
};
use crate::grammar::parsers::expression::binary_expression::primary::bitwise::{
    bitwise_or, bitwise_or_primary,
};
use crate::grammar::parsers::expression::binary_expression::primary::{parser_left, to_expr};
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;

/// Parse possible children of a logical OR expression.
pub fn logical_or_primary(input: Fragment) -> IResult<Fragment, Expression> {
    alt((map(logical_and, to_expr), logical_and_primary))(input)
}

/// 'boolean or' or 'logical or' is the lowest precedence binary operator.
pub fn logical_or(input: Fragment) -> IResult<Fragment, Expression> {
    parser_left(logical_or_primary, parse_logical_or)(input)
}

/// Parsers that can be the children of a 'logical and' expression.
fn logical_and_primary(input: Fragment) -> IResult<Fragment, Expression> {
    alt((bitwise_or, bitwise_or_primary))(input)
}

/// Parse a 'logical and' expression.
pub fn logical_and(input: Fragment) -> IResult<Fragment, Expression> {
    parser_left(logical_and_primary, parse_logical_and)(input)
}
