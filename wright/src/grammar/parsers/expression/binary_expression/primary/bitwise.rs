use crate::grammar::ast::{BinaryOp, Expression};
use crate::grammar::model::WrightInput;
use crate::grammar::parsers::expression::binary_expression::primary::equality::{
    equality, equality_primary,
};
use crate::grammar::parsers::expression::binary_expression::primary::parser_left;
use crate::grammar::tracing::trace_result;
use crate::grammar::tracing::parsers::alt;
use nom::IResult;

/// A child expression under a 'bitwise or' expression.
pub fn bitwise_or_primary<I: WrightInput>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::bitwise_or_primary";
    trace_result(
        trace,
        alt((bitwise_xor, bitwise_xor_primary))(input.trace_start_clone(trace)),
    )
}

/// A child expression under a 'bitwise xor' expression.
fn bitwise_xor_primary<I: WrightInput>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::bitwise_xor_primary";
    trace_result(
        trace,
        alt((bitwise_and, bitwise_and_primary))(input.trace_start_clone(trace)),
    )
}

/// A child expression under a 'bitwise or' expression.
fn bitwise_and_primary<I: WrightInput>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::bitwise_and_primary";
    trace_result(
        trace,
        alt((equality, equality_primary))(input.trace_start_clone(trace)),
    )
}

/// Parse a 'bitwise or' binary expression.
pub fn bitwise_or<I: WrightInput>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::bitwise_or";
    trace_result(
        trace,
        parser_left(bitwise_or_primary, BinaryOp::parse_or)(input.trace_start_clone(trace)),
    )
}

/// Parse a 'bitwise xor' binary expression.
pub fn bitwise_xor<I: WrightInput>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::bitwise_xor";
    trace_result(
        trace,
        parser_left(bitwise_xor_primary, BinaryOp::parse_xor)(input.trace_start_clone(trace)),
    )
}

/// Parse a 'bitwise and' binary expression.
pub fn bitwise_and<I: WrightInput>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::bitwise_and";
    trace_result(
        trace,
        parser_left(bitwise_and_primary, BinaryOp::parse_and)(input.trace_start_clone(trace)),
    )
}
