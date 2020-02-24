use crate::grammar::ast::{CharLit, Expression};
use crate::grammar::model::{Fragment, HasFragment};

use crate::grammar::parsers::expression::ToExpression;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::{anychar, char as ch, one_of};
use nom::combinator::{map, map_opt, map_res, not, recognize, value};
use nom::error::context;
use nom::sequence::{preceded, terminated};
use nom::IResult;

impl<'s> CharLit<'s> {
    fn new(frag: Fragment<'s>, inner: char) -> Self {
        CharLit { frag, inner }
    }

    pub(super) fn unicode_char(frag: Fragment<'s>) -> IResult<Fragment<'s>, char> {
        preceded(not(one_of("\\\t\n\r'")), anychar)(frag)
    }

    pub(super) fn character_body(frag: Fragment<'s>) -> IResult<Fragment, char> {
        let vch = move |c: char, v: char| move |fragment: Fragment<'s>| value(v, ch(c))(fragment);
        let from_str_radix = |f: Fragment<'s>| u32::from_str_radix(f.source(), 16);

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
                                    preceded(
                                        tag("u{"),
                                        terminated(
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
                                    ),
                                )),
                                std::char::from_u32,
                            ),
                        )),
                    ),
                ),
            )),
        )(frag)
    }

    pub(super) fn character_wrapper(frag: Fragment<'s>) -> IResult<Fragment, char> {
        preceded(tag("'"), terminated(Self::character_body, tag("'")))(frag)
    }

    /// Parse a character literal.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment, Self> {
        map(recognize(Self::character_wrapper), |frag| {
            Self::new(frag, Self::character_wrapper(frag).unwrap().1)
        })(input)
    }
}

impl<'s> HasFragment<'s> for CharLit<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}

impl<'s> ToExpression<'s> for CharLit<'s> {
    fn create_expr(self) -> Expression<'s> {
        Expression::CharLit(self)
    }
    #[inline]
    fn parse_self(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {Self::parse(input)}
}
