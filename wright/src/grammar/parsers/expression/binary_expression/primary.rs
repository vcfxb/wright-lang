use crate::grammar::ast::{BinaryExpression, BinaryOp, Conditional, Expression, Name, Parens};
use crate::grammar::model::Fragment;
use crate::grammar::parsers::whitespace::token_delimiter;
use nom::branch::alt;
use nom::combinator::{map, value};
use nom::multi::many1;
use nom::sequence::{delimited, pair, preceded, separated_pair};
use nom::IResult;
use nom::bytes::complete::tag;

/// Module for parsing range expressions.
/// This includes Range and RangeTo operators.
pub mod range;

/// Module for parsing 'logical or' expressions.
pub(self) mod logical_or;

/// Module for parsing 'logical and' expressions.
pub(self) mod logical_and;

/// Module for parsing 'bitwise or' expressions.
pub(self) mod bitwise_or;

/// Parser for the base expressions that can appear as a child in any binary
/// expression, down to the lowest node.
pub fn base_primary(input: Fragment) -> IResult<Fragment, Expression> {
    alt((
        map(Parens::parse, to_expr),
        map(Conditional::parse, to_expr),
        map(Name::parse, to_expr),
    ))(input)
}

/// Convert the result of a parser into an expression
pub(self) fn to_expr<'s, E: Into<Expression<'s>>>(e: E) -> Expression<'s> {
    e.into()
}

/// Return a parser for a precedence level of left associative operator.
pub(self) fn parser_left<'s, >(
    child: fn(Fragment<'s>) -> IResult<Fragment<'s>, Expression<'s>>,
    operator: fn(Fragment<'s>) -> IResult<Fragment<'s>, BinaryOp>,
) -> impl Fn(Fragment<'s>) -> IResult<Fragment<'s>, Expression<'s>> {
    move |input: Fragment<'s>| {
        map(
            pair(
                child,
                many1(pair(
                    delimited(
                        token_delimiter,
                        operator.clone(),
                        token_delimiter
                    ),
                    child
                ))
            ),
            |(fst, following)| {
                let mut acc = fst;
                let mut stack = following;
                stack.reverse();
                while !stack.is_empty() {
                    let (op, right) = stack.pop().unwrap();
                    acc = BinaryExpression::new_merge(acc, op, right).into();
                }
                acc
            }
        )(input)
    }
}