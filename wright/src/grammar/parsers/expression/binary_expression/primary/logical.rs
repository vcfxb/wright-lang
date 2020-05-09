use crate::grammar::ast::{
    Expression,
    BinaryOp,
};
use crate::grammar::parsers::expression::binary_expression::primary::bitwise::{
    bitwise_or, bitwise_or_primary,
};
use crate::grammar::parsers::expression::binary_expression::primary::{parser_left, to_expr};
use nom::branch::alt;
use nom::IResult;
use crate::grammar::tracing::{
    input::OptionallyTraceable,
    parsers::map::map,
    trace_result
};


/// Parse possible children of a logical OR expression.
pub fn logical_or_primary<I: OptionallyTraceable + std::fmt::Debug + Clone>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::logical_or_primary";
    trace_result(
        trace,
        alt((
            map(logical_and, to_expr),
            logical_and_primary
        ))(input.trace_start_clone(trace))
    )
}

/// 'boolean or' or 'logical or' is the lowest precedence binary operator.
pub fn logical_or<I: OptionallyTraceable + std::fmt::Debug + Clone>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::logical_or";
    trace_result(
        trace,
        parser_left(
            logical_or_primary,
            BinaryOp::parse_logical_or
        )(input.trace_start_clone(trace))
    )
}

/// Parsers that can be the children of a 'logical and' expression.
fn logical_and_primary<I: OptionallyTraceable + std::fmt::Debug + Clone>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::logical_and_primary";
    trace_result(
        trace,
        alt((bitwise_or, bitwise_or_primary))(input.trace_start_clone(trace))
    )
}

/// Parse a 'logical and' expression.
pub fn logical_and<I: OptionallyTraceable + std::fmt::Debug + Clone>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::logical_and";
    trace_result(
        trace,
        parser_left(
            logical_and_primary,
            BinaryOp::parse_logical_and
        )(input.trace_start_clone(trace))
    )
}
