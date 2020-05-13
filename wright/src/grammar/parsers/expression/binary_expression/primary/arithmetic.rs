use crate::grammar::ast::{BinaryOp, Expression};
use crate::grammar::model::WrightInput;
use crate::grammar::parsers::expression::binary_expression::primary::{atom, parser_left};
use crate::grammar::tracing::parsers::alt;
use crate::grammar::tracing::trace_result;
use nom::IResult;

/// Parse child of a lower precedence arithmetic operator.
pub fn arithmetic1_primary<I: WrightInput>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::arithmetic1_primary";
    trace_result(
        trace,
        alt((arithmetic2, atom))(input.trace_start_clone(trace)),
    )
}

/// Parse lower precedence arithmetic expression.
pub fn arithmetic1<I: WrightInput>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::arithmetic1";
    trace_result(
        trace,
        parser_left(arithmetic1_primary, BinaryOp::parse_arithmetic_operator1)(
            input.trace_start_clone(trace),
        ),
    )
}

/// Parse higher precedence arithmetic expression.
pub fn arithmetic2<I: WrightInput>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::arithmetic2";
    trace_result(
        trace,
        parser_left(atom, BinaryOp::parse_arithmetic_operator2)(input.trace_start_clone(trace)),
    )
}
