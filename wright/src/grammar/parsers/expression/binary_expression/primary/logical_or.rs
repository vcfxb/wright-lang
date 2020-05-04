use crate::grammar::ast::{BinaryOp, Expression};
use crate::grammar::model::Fragment;
use crate::grammar::parsers::expression::binary_expression::operator::parse_logical_or;
use crate::grammar::parsers::expression::binary_expression::primary::logical_and::{
    logical_and, logical_and_primary,
};
use crate::grammar::parsers::expression::binary_expression::primary::{
    fold_left, single_operator_level, to_expr,
};
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;

/// Parse possible children of a logical OR expression.
pub(super) fn logical_or_primary(input: Fragment) -> IResult<Fragment, Expression> {
    alt((map(logical_and, to_expr), logical_and_primary))(input)
}

/// 'boolean or' or 'logical or' is the lowest precedence binary operator.
pub(super) fn logical_or(input: Fragment) -> IResult<Fragment, Expression> {
    map(
        single_operator_level(logical_or_primary, parse_logical_or),
        |(first, list)| fold_left(first, list, BinaryOp::OrOr),
    )(input)
}
