use crate::grammar::model::WrightInput;
use crate::grammar::tracing::parsers::{alt, tag};
use crate::grammar::tracing::trace_result;
use nom::bytes::complete::take_until;
use nom::character::complete::{char, multispace0, not_line_ending};
use nom::combinator::value;
use nom::multi::{count, many0};
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::IResult;

/// Parses a Wright single line comment.
/// Wright single line comments start with `//` and will parse until a newline
/// character is reached. The returned value is the content of the comment.
pub fn line_comment<I: WrightInput>(input: I) -> IResult<I, I> {
    let trace = "line_comment";
    trace_result(
        trace,
        preceded(count(char('/'), 2), not_line_ending)(input.trace_start_clone(trace)),
    )
}

/// Parses a Wright multiline comment. Wright multiline comments are delimited
/// by `/*` and `*/`. They are not recursive. Wright has no concept of nested
/// comments, or any of the content within a comment for that matter.
pub fn multiline_comment<I: WrightInput>(input: I) -> IResult<I, I> {
    let trace = "multiline_comment";
    trace_result(
        trace,
        delimited(tag("/*"), take_until("*/"), tag("*/"))(input.trace_start_clone(trace)),
    )
}

/// Parses a sequence of adjacent whitespace and comments,
/// and discards the result.
pub fn token_delimiter<I: WrightInput>(input: I) -> IResult<I, ()> {
    let trace = "token_delimiter";
    trace_result(
        trace,
        preceded(
            multispace0,
            value(
                (),
                many0(pair(
                    // FIXME: regression of alt
                    alt((line_comment, multiline_comment)),
                    multispace0,
                )),
            ),
        )(input.trace_start_clone(trace)),
    )
}
