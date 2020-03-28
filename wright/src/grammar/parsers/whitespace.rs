use crate::grammar::model::Fragment;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{char, multispace0, not_line_ending};
use nom::combinator::value;
use nom::combinator::{map, not, recognize};
use nom::multi::{count, many0};
use nom::sequence::{delimited, preceded, terminated};
use nom::IResult;

/// Parses a Wright single line comment.
/// Wright single line comments start with `//` and will parse until a newline
/// character is reached. The returned value is the content of the comment.
pub fn line_comment(input: Fragment) -> IResult<Fragment, Fragment> {
    preceded(count(char('/'), 2), not_line_ending)(input)
}

/// Parses a Wright multiline comment
pub fn multiline_comment(input: Fragment) -> IResult<Fragment, Fragment> {
    delimited(tag("/*"), take_until("*/"), tag("*/"))(input)
}

/// Parses a sequence of adjacent whitespace and comments,
/// and discards the result.
pub fn token_delimiter(input: Fragment) -> IResult<Fragment, ()> {
    preceded(
        multispace0,
        value((), many0(terminated(line_comment, multispace0))),
    )(input)
}
