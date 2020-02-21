use crate::grammar::model::Fragment;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::combinator::{map, not, recognize};
use nom::character::complete::{char, multispace0, not_line_ending};
use nom::multi::{count, many0};
use nom::sequence::{delimited, preceded, terminated};
use nom::IResult;

/// Parses a Wright single line comment
pub fn line_comment(input: Fragment) -> IResult<Fragment, Fragment> {
    preceded(
        count(char('/'), 2),
        not_line_ending,
    )(input)
}

/// Parses a Wright multiline comment
pub fn multiline_comment(input: Fragment) -> IResult<Fragment, Fragment> {
    delimited(
        tag("/*"),
        take_until("*/"),
        tag("*/"),
    )(input)
}

/// Parses a sequence of adjacent whitespace and comments
/// Returns a vec containing the text of each comment.
pub fn token_delimiter(input: Fragment) -> IResult<Fragment, Vec<Fragment>> {
    preceded(
        multispace0,
        many0(
            terminated(
                alt((
                    line_comment,
                    multiline_comment,
                )),
                multispace0,
            ),
        ),
    )(input)
}
