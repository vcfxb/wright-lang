use nom::IResult;
use crate::grammar::model::Fragment;
use crate::grammar::ast::{Expression, BinaryOp};
use crate::grammar::parsers::expression::binary_expression::primary::{single_operator_level, fold_left, to_expr, bitwise_or, base_primary};
use nom::combinator::map;
use crate::grammar::parsers::expression::binary_expression::operator::parse_logical_and;
use nom::branch::alt;

/// Parsers that can be the children of a
pub(super) fn logical_and_primary(input: Fragment) -> IResult<Fragment, Expression> {
    alt((
        map(bitwise_or, to_expr),
        base_primary,
    ))(input)
}

pub(super) fn logical_and(input: Fragment) -> IResult<Fragment, Expression> {
    map(
        single_operator_level(
            logical_and_primary,
            parse_logical_and
        ),
        |(first, following)| {
            fold_left(first, following, BinaryOp::AndAnd)
        }
    )(input)
}
