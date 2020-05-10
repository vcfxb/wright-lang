use crate::grammar::ast::{BinaryExpression, BinaryOp, Expression};
use crate::grammar::model::WrightInput;
use crate::grammar::parsers::expression::binary_expression::primary::logical::{
    logical_or, logical_or_primary,
};
use crate::grammar::parsers::whitespace::token_delimiter;
use crate::grammar::parsers::with_input;
use crate::grammar::tracing::parsers::tag;
use crate::grammar::tracing::{parsers::map, trace_result};
use nom::branch::alt;
use nom::combinator::value;
use nom::sequence::{delimited, tuple};
use nom::IResult;

/// Parse a child node in a range expression.
pub fn range_primary<I: WrightInput>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::range_primary";
    trace_result(
        trace,
        alt((logical_or, logical_or_primary))(input.trace_start_clone(trace)),
    )
}

/// Parse a complete range expression in source code.
pub fn range_expr<I: WrightInput>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::range_expr";
    trace_result(
        trace,
        map(
            with_input(tuple((
                range_primary,
                delimited(
                    token_delimiter,
                    alt((
                        value(BinaryOp::Range, tag("..")),
                        value(BinaryOp::RangeInclusive, tag("..=")),
                    )),
                    token_delimiter,
                ),
                range_primary,
            ))),
            |(consumed, (l, op, r))| BinaryExpression::new(consumed, l, op, r).into(),
        )(input.trace_start_clone(trace)),
    )
}
