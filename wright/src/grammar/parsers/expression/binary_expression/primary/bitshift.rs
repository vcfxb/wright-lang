use crate::grammar::ast::{
    Expression,
    BinaryOp
};
use crate::grammar::parsers::expression::binary_expression::primary::arithmetic::{
    arithmetic1, arithmetic1_primary,
};
use crate::grammar::parsers::expression::binary_expression::primary::parser_left;
use nom::branch::alt;
use nom::IResult;
use crate::grammar::tracing::input::OptionallyTraceable;
use crate::grammar::tracing::trace_result;

/// Subexpressions of a bitshift expression.
pub fn bitshift_primary<I: OptionallyTraceable + std::fmt::Debug + Clone>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::bitshift_primary";
    trace_result(
        trace,
        alt((
            arithmetic1,
            arithmetic1_primary
        ))(input.trace_start_clone(trace))
    )
}

/// Bitshift expression.
pub fn bitshift<I: OptionallyTraceable + std::fmt::Debug + Clone>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::bitshift";
    trace_result(
        trace,
        parser_left(
            bitshift_primary,
            BinaryOp::parse_bitshift_operator
        )(input.trace_start_clone(trace))
    )
}
