use crate::grammar::model::Fragment;
use crate::grammar::ast::{BinaryExpression, BinaryOp};
use nom::IResult;
use nom::combinator::map;
use nom::sequence::{separated_pair, delimited};
use crate::grammar::parsers::whitespace::token_delimiter;
use crate::grammar::parsers::expression::binary_expression::operator::parse_logical_and;

/// A single operator parsing level.
fn single_operator_level<'s, O>(
    child: fn(Fragment<'s>) -> IResult<Fragment<'s>, BinaryExpression<'s>>,
    operator: O
) -> impl Fn(Fragment<'s>) -> IResult<Fragment<'s>, (BinaryExpression<'s>, BinaryExpression<'s>)>
where
    O: Fn(Fragment<'s>) -> IResult<Fragment<'s>, Fragment<'s>>,
{
    separated_pair(
        child,
        delimited(
            token_delimiter,
            operator,
            token_delimiter,
        ),
        child,
    )
}

fn logical_or_primary(input: Fragment) -> IResult<Fragment, BinaryExpression> {
    todo!()
}

/// 'boolean and' or 'logical and' is the lowest precedence binary operator.
pub fn logical_and_primary(input: Fragment) -> IResult<Fragment, BinaryExpression> {
    map(
        single_operator_level(logical_or_primary, parse_logical_and),
        |(left, right)| BinaryExpression::new_merge(left, BinaryOp::AndAnd, right)
    )(input)
}