use crate::grammar::model::Fragment;
use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use crate::grammar::ast::{BinaryExpression, BinaryOp};

impl BinaryOp {
    /// Logical AND operator in long form.
    pub const LOGICAL_AND: &'static str = "and";

    /// Logical OR operator in long form.
    pub const LOGICAL_OR: &'static str = "or";
}

/// Parse the logical and operator. Currently matches on `&&` or
/// [the logical and constant](consts.LOGICAL_AND.html).
pub fn parse_logical_and(input: Fragment) -> IResult<Fragment, Fragment> {
    alt((
        tag("&&"),
        tag(BinaryOp::LOGICAL_AND)
    ))(input)
}