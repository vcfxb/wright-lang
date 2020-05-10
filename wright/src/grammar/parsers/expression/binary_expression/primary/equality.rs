use crate::grammar::ast::{BinaryOp, Expression};
use crate::grammar::model::WrightInput;
use crate::grammar::parsers::expression::binary_expression::primary::parser_left;
use crate::grammar::parsers::expression::binary_expression::primary::relational::{
    relational, relational_primary,
};
use crate::grammar::tracing::trace_result;
use nom::branch::alt;
use nom::IResult;

pub fn equality_primary<I: WrightInput>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::equality_primary";
    trace_result(
        trace,
        alt((relational, relational_primary))(input.trace_start_clone(trace)),
    )
}

/// Parse equality expression.
pub fn equality<I: WrightInput>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::equality";
    trace_result(
        trace,
        parser_left(equality_primary, BinaryOp::parse_equality_operator)(
            input.trace_start_clone(trace),
        ),
    )
}
