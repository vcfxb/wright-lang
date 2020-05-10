use crate::grammar::ast::Block;
use crate::grammar::ast::{
    BinaryExpression, BinaryOp, BooleanLit, Conditional, Expression, NumLit, Parens, ScopedName,
    SelfLit, StringLit,
};
use crate::grammar::model::WrightInput;
use crate::grammar::parsers::expression::binary_expression::primary::{
    arithmetic::{arithmetic1, arithmetic2},
    bitshift::bitshift,
    bitwise::{bitwise_and, bitwise_or, bitwise_xor},
    equality::equality,
    logical::{logical_and, logical_or},
    range::range_expr,
    relational::relational,
};
use crate::grammar::parsers::whitespace::token_delimiter;
use crate::grammar::parsers::with_input;
use crate::grammar::tracing::parsers::map;
use crate::grammar::tracing::trace_result;
use crate::grammar::tracing::parsers::alt;
use nom::sequence::{delimited, pair, tuple};
use nom::IResult;

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

/// Since the Binary expression parser is actually a general expression
/// parser internally, which is verified to be a binary expression before
/// being returned, we can use the lowest precedence binary expression
/// parser as a general expression parser. This function is always just
/// a call to the lowest precedence binary expression parser.
pub fn parse_expr<I: WrightInput>(input: I) -> IResult<I, Expression<I>> {
    let trace = "Expr::parse_expr";
    trace_result(trace, range_expr(input.trace_start_clone(trace)))
}

/// Parser for the base expressions that can appear as a child in any binary
/// expression, down to the lowest node.
pub fn atom<I: WrightInput>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::atom";
    let res = alt((
        map(Parens::parse, to_expr),
        map(Block::parse, to_expr),
        map(NumLit::parse, to_expr),
        map(StringLit::parse, to_expr),
        map(BooleanLit::parse, to_expr),
        map(SelfLit::parse, to_expr),
        map(Conditional::parse, to_expr),
        //map(FuncCallExpression::parse, to_expr), // make sure we aren't causing recursion
        map(ScopedName::parse, to_expr),
    ))(input.trace_start_clone(trace));
    trace_result(trace, res)
}

/// Convert the result of a parser into an expression
#[inline]
pub(self) fn to_expr<I, E>(e: E) -> Expression<I>
where
    I: std::fmt::Debug + Clone,
    E: Into<Expression<I>>,
{
    e.into()
}

/// Return a parser for a precedence level of left associative operator.
/// Requires that the child parser succeed at least once.
/// Does not require that the produced output is a binary expression.
pub(self) fn parser_left<I: WrightInput>(
    child: fn(I) -> IResult<I, Expression<I>>,
    operator: fn(I) -> IResult<I, BinaryOp>,
) -> impl Fn(I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::parser_left";
    move |input| -> IResult<I, Expression<I>> {
        let source = input.trace_start_clone(trace);

        let first_res = child(source);

        if first_res.is_err() {
            return trace_result(trace, first_res);
        }

        let first = first_res.unwrap();

        let mut rem = first.0;
        let mut acc = first.1;

        while let Ok((new_rem, (op, right))) =
            pair(delimited(token_delimiter, operator, token_delimiter), child)(rem.clone())
        {
            rem = new_rem;
            let index = input.offset(&rem);
            let consumed = input.slice(..index);
            acc = BinaryExpression::new(consumed, acc, op, right).into();
        }

        Ok((rem.trace_end_clone(trace, true), acc))
    }
}

/// Parse a binary expression.
pub fn parse_binary_expr<I: WrightInput>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::parse_binary_expr";
    trace_result(
        trace,
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
        ))(input.trace_start_clone(trace)),
    )
}
