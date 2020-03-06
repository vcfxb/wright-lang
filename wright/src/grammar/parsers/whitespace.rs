use crate::grammar::model::Fragment;
use nom::character::complete::{char as ch, multispace0, not_line_ending};
use nom::combinator::value;
use nom::multi::{count, many0};
use nom::sequence::{preceded, terminated};
use nom::IResult;

/// Parses a Wright single line comment.
/// Wright single line comments start with `//` and will parse until a newline
/// character is reached. The returned value is the content of the comment.
pub fn line_comment(input: Fragment) -> IResult<Fragment, Fragment> {
    preceded(count(ch('/'), 2), not_line_ending)(input)
}

/// Parses a sequence of adjacent whitespace and comments,
/// and discards the result.
pub fn token_delimiter(input: Fragment) -> IResult<Fragment, ()> {
    preceded(
        multispace0,
        value((), many0(terminated(line_comment, multispace0))),
    )(input)
}
