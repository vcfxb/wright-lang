use crate::grammar::ast::{BinaryExpression, BinaryOp, Conditional, Expression, Name, Parens, NumLit, StringLit, BooleanLit, SelfLit, FuncCallExpression};
use crate::grammar::model::Fragment;
use crate::grammar::parsers::whitespace::token_delimiter;
use nom::branch::alt;
use nom::combinator::{map, value};
use nom::multi::many1;
use nom::sequence::{delimited, pair, preceded, separated_pair};
use nom::IResult;
use nom::bytes::complete::tag;
use crate::grammar::parsers::expression::binary_expression::primary::{
    equality::equality,
    relational::relational,
    range::range_expr,
    logical::{logical_or, logical_and},
    bitwise::{bitwise_or, bitwise_xor, bitwise_and},
    bitshift::bitshift,
    arithmetic::{arithmetic1, arithmetic2},
};
use crate::grammar::ast::Block;

/// Module for parsing range expressions.
/// This includes Range and RangeTo operators.
pub mod range;

/// Module for parsing logical expressions, including
/// logical 'or', and logical 'and'.
pub(self) mod logical;

/// Module for parsing bitwise expressions, including
/// bitwise 'or', 'xor', and 'and' operations.
pub(self) mod bitwise;

/// Module for parsing equality expressions, including
/// 'equals' and 'not equals'.
pub(self) mod equality;

/// Module for parsing relational expressions.
/// These include all of the comparison operators.
pub(self) mod relational;

/// Module for parsing bitshift expressions.
pub(self) mod bitshift;

/// Module for parsing arithmetic operators.
pub(self) mod arithmetic;

/// Parser for the base expressions that can appear as a child in any binary
/// expression, down to the lowest node.
pub fn base_primary(input: Fragment) -> IResult<Fragment, Expression> {
    alt((
        map(Parens::parse, to_expr),
        map(Block::parse, to_expr),
        map(NumLit::parse, to_expr),
        map(StringLit::parse, to_expr),
        map(BooleanLit::parse, to_expr),
        map(SelfLit::parse, to_expr),
        map(Conditional::parse, to_expr),
        map(FuncCallExpression::parse, to_expr),
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

/// Parse a binary expression.
pub fn parse_binary_expr(input: Fragment) -> IResult<Fragment, Expression> {
    alt((
        range_expr,
        logical_or,
        logical_and,
        bitwise_or,
        bitwise_xor,
        bitwise_and,
        equality,
        relational,
        bitshift,
        arithmetic1,
        arithmetic2,
    ))(input)
}