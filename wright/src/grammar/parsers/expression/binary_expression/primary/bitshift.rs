use crate::grammar::ast::{BinaryOp, Expression};
use crate::grammar::model::WrightInput;
use crate::grammar::parsers::expression::binary_expression::primary::arithmetic::{
    arithmetic1, arithmetic1_primary,
};
use crate::grammar::parsers::expression::binary_expression::primary::parser_left;
use crate::grammar::tracing::trace_result;
use nom::branch::alt;
use nom::IResult;

/// Subexpressions of a bitshift expression.
pub fn bitshift_primary<I: WrightInput>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::bitshift_primary";
    trace_result(
        trace,
        alt((arithmetic1, arithmetic1_primary))(input.trace_start_clone(trace)),
    )
}

/// Bitshift expression.
pub fn bitshift<I: WrightInput>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::bitshift";
    trace_result(
        trace,
        parser_left(bitshift_primary, BinaryOp::parse_bitshift_operator)(
            input.trace_start_clone(trace),
        ),
    )
}
