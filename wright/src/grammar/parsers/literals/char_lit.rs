use crate::grammar::ast::{CharLit, Expression, eq::ASTEq};
use crate::grammar::model::{Fragment, HasFragment};

use crate::grammar::parsers::expression::ToExpression;
use crate::grammar::parsers::with_input;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::{anychar, char as ch, one_of};
use nom::combinator::{map, map_opt, map_res, not, value};
use nom::error::context;
use nom::sequence::{delimited, preceded};
use nom::IResult;

impl<'s> CharLit<'s> {
    fn new(frag: Fragment<'s>, inner: char) -> Self {
        CharLit { frag, inner }
    }

    pub(crate) fn unicode_char(frag: Fragment<'s>) -> IResult<Fragment<'s>, char> {
        preceded(not(one_of("\\\t\n\r'")), anychar)(frag)
    }

    pub(crate) fn character_body(frag: Fragment<'s>) -> IResult<Fragment, char> {
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
        )(frag)
    }

    pub(super) fn character_wrapper(frag: Fragment<'s>) -> IResult<Fragment, char> {
        delimited(tag("'"), Self::character_body, tag("'"))(frag)
    }

    /// Parse a character literal.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment, Self> {
        map(with_input(Self::character_wrapper), |(frag, ch)| {
            Self::new(frag, ch)
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
}

impl<'s> ASTEq for CharLit<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {fst.inner == snd.inner}
}