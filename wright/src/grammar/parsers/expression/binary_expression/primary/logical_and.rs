use crate::grammar::ast::{BinaryOp, Expression};
use crate::grammar::model::Fragment;
use crate::grammar::parsers::expression::binary_expression::operator::parse_logical_and;
use crate::grammar::parsers::expression::binary_expression::primary::{
    base_primary, bitwise_or, fold_left, single_operator_level, to_expr,
};
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;

/// Parsers that can be the children of a
pub(super) fn logical_and_primary(input: Fragment) -> IResult<Fragment, Expression> {
    alt((map(bitwise_or, to_expr), base_primary))(input)
}

pub(super) fn logical_and(input: Fragment) -> IResult<Fragment, Expression> {
    map(
        single_operator_level(logical_and_primary, parse_logical_and),
        |(first, following)| fold_left(first, following, BinaryOp::AndAnd),
    )(input)
}
