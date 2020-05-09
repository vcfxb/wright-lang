use crate::grammar::ast::{eq::AstEq, Expression, StringLit};
use crate::grammar::model::{HasSourceReference, WrightInput};
use crate::grammar::parsers::with_input;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::{anychar, char as ch, multispace0, newline, one_of};
use nom::combinator::{map_res, not, value};
use nom::error::context;
use nom::multi::many0;
use nom::sequence::{delimited, preceded};
use nom::IResult;
use crate::grammar::tracing::{
    input::OptionallyTraceable,
    parsers::map::map,
    trace_result
};
use std::fmt::Debug;

impl<T: Debug + Clone> StringLit<T> {
    /// Name of this parser in parser tracing.
    pub const TRACE_NAME: &'static str = "StringLit";
}

impl<'a, I: WrightInput<'a>> StringLit<I> {
    fn new(source: I, inner: String) -> Self {
        Self { source, inner }
    }

    fn anych(input: I) -> IResult<I, Option<char>> {
        map(preceded(not(one_of("\\\"")), anychar), |c| Some(c))(input)
    }

    fn body(input: I) -> IResult<I, String> {
        let vch =
            move |c: char, v: char| move |source: I| value(Some(v), ch(c))(source);
        let from_str_radix = |str: I| u32::from_str_radix(&str.into(), 16);
        map(
            many0(alt((
                Self::anych,
                preceded(
                    ch('\\'),
                    alt((
                        vch('\\', '\\'),
                        vch('\'', '\''),
                        vch('\"', '\"'),
                        vch('0', '\0'),
                        vch('n', '\n'),
                        vch('r', '\r'),
                        vch('t', '\t'),
                        alt((
                            map(
                                preceded(
                                    ch('x'),
                                    map_res(
                                        context(
                                            "expected exactly 2 hexadecimal digits",
                                            take_while_m_n(2, 2, |c: char| c.is_ascii_hexdigit()),
                                        ),
                                        from_str_radix,
                                    ),
                                ),
                                std::char::from_u32,
                            ),
                            map(
                                delimited(
                                    tag("u{"),
                                    map_res(
                                        context(
                                            "expected between 1 and 6 hexadecimal digits",
                                            take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit()),
                                        ),
                                        from_str_radix,
                                    ),
                                    ch('}'),
                                ),
                                std::char::from_u32,
                            ),
                            value(None, preceded(newline, multispace0)),
                        )),
                    )),
                ),
            ))),
            |vec: Vec<Option<char>>| {
                vec.iter()
                    .map(|o| *o)
                    .filter(Option::is_some)
                    .map(Option::unwrap)
                    .collect::<String>()
            },
        )(input)
    }

    fn wrapper(i: I) -> IResult<I, String> {
        delimited(ch('\"'), Self::body, ch('\"'))(i)
    }

    /// Parse a string literal in source code.
    pub fn parse(input: I) -> IResult<I, Self> {
        let res = map(
            with_input(Self::wrapper),
            |(consumed, result)| {
                Self::new(consumed, result)
            }
        )(input.trace_start_clone(Self::TRACE_NAME));
        trace_result(Self::TRACE_NAME, res)
    }
}

impl<I: Debug + Clone> HasSourceReference<I> for StringLit<I> {
    fn get_source_ref(&self) -> &I {
        &self.source
    }
}

impl<I: Debug + Clone> Into<Expression<I>> for StringLit<I> {
    fn into(self) -> Expression<I> {
        Expression::StringLit(self)
    }
}

impl<I: Debug + Clone> AstEq for StringLit<I> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        fst.inner == snd.inner
    }
}
