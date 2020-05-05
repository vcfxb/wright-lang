use crate::grammar::ast::{BinaryOp, Expression};
use crate::grammar::model::Fragment;
use crate::grammar::parsers::expression::binary_expression::operator::parse_logical_and;
use crate::grammar::parsers::expression::binary_expression::primary::{base_primary, to_expr, parser_left};
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;
use crate::grammar::parsers::expression::binary_expression::primary::bitwise_or::bitwise_or;

/// Parsers that can be the children of a 'logical and' expression.
pub(super) fn logical_and_primary(input: Fragment) -> IResult<Fragment, Expression> {
    alt((map(bitwise_or, to_expr), base_primary))(input)
}

/// Parse a 'logical and' expression.
pub(super) fn logical_and(input: Fragment) -> IResult<Fragment, Expression> {
    parser_left(logical_and_primary, parse_logical_and)(input)
}
