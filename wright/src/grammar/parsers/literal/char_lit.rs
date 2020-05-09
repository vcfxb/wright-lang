use crate::grammar::ast::{eq::AstEq, CharLit, Expression};
use crate::grammar::model::HasSourceReference;

use crate::grammar::parsers::with_input;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::{anychar, char as ch, one_of};
use nom::combinator::{map_opt, map_res, not, value};
use nom::error::context;
use nom::sequence::{delimited, preceded};
use nom::IResult;
use crate::grammar::tracing::{
    input::OptionallyTraceable,
    trace_result,
    parsers::map::map,
};
use std::fmt::Debug;

impl<T: Debug + Clone> CharLit<T> {
    /// The name of this parser in traces.
    pub const TRACE_NAME: &'static str = "CharLit";
}

impl<I: Debug + Clone + OptionallyTraceable> CharLit<I> {
    fn new(source: I, inner: char) -> Self {
        Self { source, inner }
    }

    /// Parse an unescaped unicode character.
    pub(super) fn unicode_char(source: I) -> IResult<I, char> {
        preceded(not(one_of("\\\t\n\r'")), anychar)(source)
    }

    /// Parse any escaped or unescaped character.
    pub(super) fn character_body(source: I) -> IResult<I, char> {
        let vch = move |c: char, v: char| move |s: I| value(v, ch(c))(s);
        let from_str_radix = |i: I| u32::from_str_radix(i.source(), 16);

        context(
            "expected character literal",
            alt((
                Self::unicode_char,
                preceded(
                    tag("\\"),
                    context(
                        "unrecognized escape sequence",
                        alt((
                            vch('\\', '\\'),
                            vch('\'', '\''),
                            vch('\"', '\"'),
                            vch('0', '\0'),
                            vch('n', '\n'),
                            vch('r', '\r'),
                            vch('t', '\t'),
                            map_opt(
                                alt((
                                    preceded(
                                        ch('x'),
                                        map_res(
                                            context(
                                                "expected exactly 2 hexadecimal digits",
                                                take_while_m_n(2, 2, |c: char| {
                                                    c.is_ascii_hexdigit()
                                                }),
                                            ),
                                            from_str_radix,
                                        ),
                                    ),
                                    delimited(
                                        tag("u{"),
                                        map_res(
                                            context(
                                                "expected between 1 and 6 hexadecimal digits",
                                                take_while_m_n(1, 6, |c: char| {
                                                    c.is_ascii_hexdigit()
                                                }),
                                            ),
                                            from_str_radix,
                                        ),
                                        ch('}'),
                                    ),
                                )),
                                std::char::from_u32,
                            ),
                        )),
                    ),
                ),
            )),
        )(source)
    }

    pub(super) fn character_wrapper(source: I) -> IResult<I, char> {
        delimited(tag("'"), Self::character_body, tag("'"))(source)
    }

    /// Parse a character literal.
    pub fn parse(input: I) -> IResult<I, Self> {
        let res = map(
            with_input(Self::character_wrapper),
            |(input, ch)| Self::new(input, ch)
        )(input.trace_start_clone(Self::TRACE_NAME));
        trace_result(Self::TRACE_NAME, res)
    }
}

impl<I: Debug + Clone> HasSourceReference<I> for CharLit<I> {
    fn get_source_ref(&self) -> &I {
        &self.source
    }
}

impl<I: Debug + Clone> Into<Expression<I>> for CharLit<I> {
    fn into(self) -> Expression<I> {
        Expression::CharLit(self)
    }
}

impl<I: Debug + Clone> AstEq for CharLit<I> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        fst.inner == snd.inner
    }
}
