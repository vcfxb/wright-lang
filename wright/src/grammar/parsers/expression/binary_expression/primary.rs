use crate::grammar::model::Fragment;
use crate::grammar::ast::{BinaryExpression, BinaryOp, Expression, Name, Parens};
use nom::IResult;
use nom::combinator::map;
use nom::sequence::{separated_pair, delimited, pair, preceded};
use crate::grammar::parsers::whitespace::token_delimiter;
use nom::multi::many1;
use nom::branch::alt;

/// Module for parsing logical or expressions.
pub mod logical_or;

/// Module for parsing logical and expressions.
pub(self) mod logical_and;

/// A single operator parsing level.
pub(self) fn single_operator_level<'s, O>(
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

/// Parser for the base expressions that can appear as a child in any binary
/// expression, down to the lowest node.
pub fn base_primary(input: Fragment) -> IResult<Fragment, Expression> {
    alt((
        map(Parens::parse, to_expr),
        map(Name::parse, to_expr),
    ))(input)
}

/// Convert the result of a parser into an expression
pub(self) fn to_expr<'s, E: Into<Expression<'s>>>(e: E) -> Expression<'s> {e.into()}

pub(super) fn bitwise_or(input: Fragment) -> IResult<Fragment, Expression> {
    todo!()
}
