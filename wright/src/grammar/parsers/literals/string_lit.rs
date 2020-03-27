use crate::grammar::ast::{eq::ASTEq, Expression, StringLit};
use crate::grammar::model::{Fragment, HasFragment};
use crate::grammar::parsers::expression::ToExpression;
use crate::grammar::parsers::with_input;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::{anychar, char as ch, multispace0, newline, one_of};
use nom::combinator::{map, map_res, not, value};
use nom::error::context;
use nom::multi::many0;
use nom::sequence::{delimited, preceded};
use nom::IResult;

impl<'s> StringLit<'s> {
    fn new(frag: Fragment<'s>, inner: String) -> Self {
        Self { frag, inner }
    }

    fn anych(input: Fragment<'s>) -> IResult<Fragment<'s>, Option<char>> {
        map(preceded(not(one_of("\\\"")), anychar), |c| Some(c))(input)
    }

    fn body(input: Fragment<'s>) -> IResult<Fragment<'s>, String> {
        let vch =
            move |c: char, v: char| move |fragment: Fragment<'s>| value(Some(v), ch(c))(fragment);
        let from_str_radix = |f: Fragment<'s>| u32::from_str_radix(f.source(), 16);
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

    fn wrapper(i: Fragment<'s>) -> IResult<Fragment<'s>, String> {
        delimited(ch('\"'), Self::body, ch('\"'))(i)
    }

    /// Parse a string literal in source code.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(with_input(Self::wrapper), |(consumed, result)| {
            Self::new(consumed, result)
        })(input)
    }
}

impl<'s> HasFragment<'s> for StringLit<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}

impl<'s> ToExpression<'s> for StringLit<'s> {
    fn create_expr(self) -> Expression<'s> {
        Expression::StringLit(self)
    }
}

impl<'s> ASTEq for StringLit<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        fst.inner == snd.inner
    }
}
