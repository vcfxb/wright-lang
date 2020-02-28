use crate::grammar::model::Fragment;
use nom::character::complete::{char, multispace0, not_line_ending};
use nom::multi::{count, many0};
use nom::sequence::{preceded, terminated};
use nom::IResult;

/// Parses a Wright single line comment
pub fn line_comment(input: Fragment) -> IResult<Fragment, Fragment> {
    preceded(count(char('/'), 2), not_line_ending)(input)
}

/// Parses a sequence of adjacent whitespace and comments
/// Returns a vec containing the comment text lines
pub fn token_delimiter(input: Fragment) -> IResult<Fragment, Vec<Fragment>> {
    preceded(multispace0, many0(terminated(line_comment, multispace0)))(input)
}
