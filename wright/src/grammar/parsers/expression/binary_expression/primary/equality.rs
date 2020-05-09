use crate::grammar::ast::{
    Expression,
    BinaryOp
};
use crate::grammar::parsers::expression::binary_expression::primary::parser_left;
use crate::grammar::parsers::expression::binary_expression::primary::relational::{
    relational, relational_primary,
};
use nom::branch::alt;
use nom::IResult;
use crate::grammar::tracing::input::OptionallyTraceable;
use crate::grammar::tracing::trace_result;

pub fn equality_primary<I: OptionallyTraceable>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::equality_primary";
    trace_result(
        trace,
        alt((
            relational,
            relational_primary
        ))(input.trace_start_clone(trace))
    )
}

/// Parse equality expression.
pub fn equality<I: OptionallyTraceable>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::equality";
    trace_result(
        trace,
        parser_left(
            equality_primary,
            BinaryOp::parse_equality_operator
        )(input.trace_start_clone(trace))
    )
}
