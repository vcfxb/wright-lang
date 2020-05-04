use crate::grammar::ast::{BinaryExpression, BinaryOp};
use crate::grammar::model::Fragment;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::IResult;

impl BinaryOp {
    /// Logical AND operator in long form.
    pub const LOGICAL_AND: &'static str = "and";

    /// Logical OR operator in long form.
    pub const LOGICAL_OR: &'static str = "or";
}

/// Parse the short or long version of a binary operator.
fn short_or_long<'s>(
    short: &'static str,
    long: &'static str,
) -> impl Fn(Fragment<'s>) -> IResult<Fragment<'s>, Fragment<'s>> {
    alt((tag(short), tag(long)))
}

/// Parse the logical AND operator. Currently matches on `&&` or
/// the logical AND associated constant defined in BinaryOp.
pub fn parse_logical_and(input: Fragment) -> IResult<Fragment, Fragment> {
    short_or_long("&&", BinaryOp::LOGICAL_AND)(input)
}

/// Parse the logical OR operator. Currently matches on `||` or
/// the logical OR associated constant defined in BinaryOp.
pub fn parse_logical_or(input: Fragment) -> IResult<Fragment, Fragment> {
    short_or_long("||", BinaryOp::LOGICAL_OR)(input)
}
