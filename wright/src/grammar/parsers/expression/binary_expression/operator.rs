use crate::grammar::ast::{BinaryExpression, BinaryOp, Expression};
use crate::grammar::model::Fragment;
use nom::branch::alt;
use nom::character::complete::char as ch;
use nom::IResult;
use nom::combinator::value;
use nom::bytes::complete::tag;

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
    result: BinaryOp,
) -> impl Fn(Fragment<'s>) -> IResult<Fragment<'s>, BinaryOp> {
    value(result, alt((tag(short), tag(long))))
}

/// Parse the logical AND operator. Currently matches on `&&` or
/// the logical AND associated constant defined in BinaryOp.
pub fn parse_logical_and(input: Fragment) -> IResult<Fragment, BinaryOp> {
    short_or_long(
        "&&",
        BinaryOp::LOGICAL_AND,
        BinaryOp::LogicalAnd
    )(input)
}

/// Parse the logical OR operator. Currently matches on `||` or
/// the logical OR associated constant defined in BinaryOp.
pub fn parse_logical_or(input: Fragment) -> IResult<Fragment, BinaryOp> {
    short_or_long(
        "||",
        BinaryOp::LOGICAL_OR,
        BinaryOp::LogicalOr
    )(input)
}

/// Parse a 'bitwise or' operator ('|').
pub fn parse_or(input: Fragment) -> IResult<Fragment, BinaryOp> {
    value(BinaryOp::Or, ch('|'))(input)
}

/// Parse a 'bitwise xor' operator ('^').
pub fn parse_xor(input: Fragment) -> IResult<Fragment, BinaryOp> {
    value(BinaryOp::Xor, ch('^'))(input)
}

/// Parse a 'bitwise and' operator ('&').
pub fn parse_and(input: Fragment) -> IResult<Fragment, BinaryOp> {
    value(BinaryOp::And, ch('&'))(input)
}

/// Parse an 'equals' (`==`) or 'not equals' (`!=`).
pub fn parse_equality_operator(input: Fragment) -> IResult<Fragment, BinaryOp> {
    alt((
        value(BinaryOp::EqEq, tag("==")),
        value(BinaryOp::NotEq, tag("!="))
    ))(input)
}

/// Parse a relational operator.
/// Relational operators include greater than (`>`), less than (`<`),
/// and their inclusive counterparts (`>=` and `<=` respectively).
pub fn parse_relational_operator(input: Fragment) -> IResult<Fragment, BinaryOp> {
    alt((
        value(BinaryOp::Ge, tag(">=")),
        value(BinaryOp::Le, tag("<=")),
        value(BinaryOp::Gt, ch('>')),
        value(BinaryOp::Lt, ch('<')),
    ))(input)
}

/// Parse a bitshift expression.
/// These include 'left shift' (`<<`), 'right shift' (`>>`),
/// and 'unsigned right shift' (`>>>`).
pub fn parse_bitshift_operator(input: Fragment) -> IResult<Fragment, BinaryOp> {
    alt((
        value(BinaryOp::LeftShift, tag("<<")),
        value(BinaryOp::UnsignedRightShift, tag(">>>")),
        value(BinaryOp::RightShift, tag(">>")),
    ))(input)
}