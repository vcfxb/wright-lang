use crate::grammar::ast::{
    Expression,
    BinaryOp
};
use crate::grammar::parsers::expression::binary_expression::primary::bitshift::{
    bitshift, bitshift_primary,
};
use crate::grammar::parsers::expression::binary_expression::primary::parser_left;
use nom::branch::alt;
use nom::IResult;
use crate::grammar::tracing::trace_result;
use crate::grammar::model::WrightInput;

/// Parser for sub expressions of a relational expression.
pub fn relational_primary<I: WrightInput>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::relational_primary";
    trace_result(
        trace,
        alt((
            bitshift,
            bitshift_primary
        ))(input.trace_start_clone(trace))
    )
}

/// Parse a relational expression.
pub fn relational<I: WrightInput>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::relational";
    trace_result(
        trace,
        parser_left(
            relational_primary,
            BinaryOp::parse_relational_operator
        )(input.trace_start_clone(trace))
    )
}
