use crate::grammar::ast::BinaryOp;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char as ch;
use nom::combinator::value;
use nom::{IResult, InputTake, Compare};
use crate::grammar::tracing::input::OptionallyTraceable;
use crate::grammar::tracing::trace_result;
use crate::grammar::model::WrightInput;

impl BinaryOp {
    /// Logical AND operator in long form.
    pub const LOGICAL_AND: &'static str = "and";

    /// Logical OR operator in long form.
    pub const LOGICAL_OR: &'static str = "or";

    /// Modulus operator in long form.
    pub const MOD: &'static str = "mod";


}

impl BinaryOp {
    /// Parse the short or long version of a binary operator.
    fn short_or_long<'a, I>(
        short: &'static str,
        long: &'static str,
        result: BinaryOp,
    ) -> impl Fn(I) -> IResult<I, BinaryOp>
    where
        I: OptionallyTraceable + std::fmt::Debug + Clone + InputTake + Compare<&'a str>
    {
        let trace = "BinaryOp::short_or_long";
        move |input| {
            trace_result(
                trace,
                value(
                    result,
                    alt((
                        tag(short),
                        tag(long))
                    ))(input.trace_start_clone(trace)
                )
            )
        }
    }


    /// Parse the logical AND operator. Currently matches on `&&` or
    /// the logical AND associated constant defined in BinaryOp.
    pub fn parse_logical_and<I: WrightInput>(input: I) -> IResult<I, BinaryOp> {
        let trace = "BinaryOp::parse_logical_and";
        trace_result(
            trace,
            Self::short_or_long(
                "&&",
                BinaryOp::LOGICAL_AND,
                BinaryOp::LogicalAnd
            )(input.trace_start_clone(trace))
        )
    }

    /// Parse the logical OR operator. Currently matches on `||` or
    /// the logical OR associated constant defined in BinaryOp.
    pub fn parse_logical_or<I: WrightInput>(input: I) -> IResult<I, BinaryOp> {
        let trace = "BinaryOp::parse_logical_or";
        trace_result(
            trace,
            Self::short_or_long(
                "||",
                BinaryOp::LOGICAL_OR,
                BinaryOp::LogicalOr
            )(input.trace_start_clone(trace))
        )
    }

    /// Parse a 'bitwise or' operator ('|').
    pub fn parse_or<I: WrightInput>(input: I) -> IResult<I, BinaryOp> {
        let trace = "BinaryOp::parse_or";
        trace_result(
            trace,
            value(
                BinaryOp::Or,
                ch('|')
            )(input.trace_start_clone(trace))
        )
    }

    /// Parse a 'bitwise xor' operator ('^').
    pub fn parse_xor<I: WrightInput>(input: I) -> IResult<I, BinaryOp> {
        let trace = "BinaryOp::parse_xor";
        trace_result(
            trace,
            value(
                BinaryOp::Xor,
                ch('^')
            )(input.trace_start_clone(trace))
        )
    }

    /// Parse a 'bitwise and' operator ('&').
    pub fn parse_and<I: WrightInput>(input: I) -> IResult<I, BinaryOp> {
        let trace = "BinaryOp::parse_and";
        trace_result(
            trace,
            value(
                BinaryOp::And,
                ch('&')
            )(input.trace_start_clone(trace))
        )
    }

    /// Parse an 'equals' (`==`) or 'not equals' (`!=`).
    pub fn parse_equality_operator<I: WrightInput>(input: I) -> IResult<I, BinaryOp> {
        let trace = "BinaryOp::parse_equality_operator";
        trace_result(trace, alt((
            value(BinaryOp::EqEq, tag("==")),
            value(BinaryOp::NotEq, tag("!=")),
        ))(input.trace_start_clone(trace)))
    }

    /// Parse a relational operator.
    /// Relational operators include greater than (`>`), less than (`<`),
    /// and their inclusive counterparts (`>=` and `<=` respectively).
    pub fn parse_relational_operator<I: WrightInput>(input: I) -> IResult<I, BinaryOp> {
        let trace = "BinaryOp::parse_relational_operator";
        trace_result(trace, alt((
            value(BinaryOp::Ge, tag(">=")),
            value(BinaryOp::Le, tag("<=")),
            value(BinaryOp::Gt, ch('>')),
            value(BinaryOp::Lt, ch('<')),
        ))(input.trace_start_clone(trace)))
    }

    /// Parse a bitshift operator.
    /// These include 'left shift' (`<<`), 'right shift' (`>>`),
    /// and 'unsigned right shift' (`>>>`).
    pub fn parse_bitshift_operator<I: WrightInput>(input: I) -> IResult<I, BinaryOp> {
        let trace = "BinaryOp::parse_bitshift_operator";
        trace_result(trace, alt((
            value(BinaryOp::LeftShift, tag("<<")),
            value(BinaryOp::UnsignedRightShift, tag(">>>")),
            value(BinaryOp::RightShift, tag(">>")),
        ))(input.trace_start_clone(trace)))
    }

    /// Parse an arithmetic operator of lower precedence.
    /// This is currently just addition and subtraction.
    pub fn parse_arithmetic_operator1<I: WrightInput>(input: I) -> IResult<I, BinaryOp> {
        let trace = "BinaryOp::parse_arithmetic_operator1";
        trace_result(
            trace,
            alt((
                value(
                    BinaryOp::Add,
                    ch('+')
                ),
                value(
                    BinaryOp::Sub,
                    ch('-')
                )
            ))(input.trace_start_clone(trace))
        )
    }

    /// Parse an arithmetic operator of higher precedence.
    /// This includes multiplication, division, and the modulus operator.
    pub fn parse_arithmetic_operator2<I: WrightInput>(input: I) -> IResult<I, BinaryOp> {
        let trace = "BinaryOp::parse_arithmetic_operator2";
        trace_result(trace, alt((
            value(BinaryOp::Mul, ch('*')),
            value(BinaryOp::Div, ch('/')),
            Self::short_or_long("%", BinaryOp::MOD, BinaryOp::Mod),
        ))(input.trace_start_clone(trace)))
    }
}
