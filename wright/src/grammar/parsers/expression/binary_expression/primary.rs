use crate::grammar::model::Fragment;
use crate::grammar::ast::{BinaryExpression, BinaryOp, Expression, Name};
use nom::IResult;
use nom::combinator::map;
use nom::sequence::{separated_pair, delimited, pair, preceded};
use crate::grammar::parsers::whitespace::token_delimiter;
use crate::grammar::parsers::expression::binary_expression::operator::parse_logical_and;
use nom::multi::many1;
use nom::branch::alt;

/// A single operator parsing level.
fn single_operator_level<'s, O>(
    child: fn(Fragment<'s>) -> IResult<Fragment<'s>, Expression<'s>>,
    operator: O
) -> impl Fn(Fragment<'s>) -> IResult<Fragment<'s>, (Expression<'s>, Vec<Expression<'s>>)>
where
    O: Fn(Fragment<'s>) -> IResult<Fragment<'s>, Fragment<'s>>,
{
    pair(
        child,
        many1(preceded(
            delimited(
                token_delimiter,
                operator,
                token_delimiter,
            ),
            child,
        ))
    )
}

/// Fold a beginning expression and a following list of expressions into a single
/// expression left-associatively. Connect them all using the same operator.
fn fold_left<'s>(first: Expression<'s>, list: Vec<Expression<'s>>, op: BinaryOp) -> Expression<'s> {
    let mut stack = list;
    stack.reverse();
    let mut acc = first;
    while !stack.is_empty() {
        acc =
            BinaryExpression::new_merge(
                acc,
                op,
                stack.pop().unwrap())
                .into();
    }
    acc
}

fn logical_or(input: Fragment) -> IResult<Fragment, BinaryExpression> {
    todo!()
}

fn logical_and_primary(input: Fragment) -> IResult<Fragment, Expression> {
    fn expr<'s, E: Into<Expression<'s>>>(e: E) -> Expression<'s> {e.into()}
    alt((
        map(logical_or, expr),
        map(Name::parse, expr)
    ))(input)
}

/// 'boolean and' or 'logical and' is the lowest precedence binary operator.
pub fn logical_and(input: Fragment) -> IResult<Fragment, Expression> {
    map(
        single_operator_level(
            logical_and_primary, parse_logical_and),
        |(first, list)|
            fold_left(first, list, BinaryOp::AndAnd)
    )(input)
}