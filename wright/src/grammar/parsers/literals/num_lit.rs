use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take_while1, take_while_m_n},
    combinator::{map, map_res, peek},
    sequence::preceded,
    IResult,
};

use crate::grammar::{ast::NumLit, model::Fragment};

use crate::grammar::ast::Expression;
use crate::grammar::model::HasFragment;
use crate::grammar::parsers::expression::ToExpression;
use crate::grammar::parsers::with_input;
use std::num::ParseIntError;

impl<'s> NumLit<'s> {
    fn new(frag: Fragment<'s>, num: u128) -> Self {
        Self { frag, inner: num }
    }

    fn from_hex(input: &str) -> Result<u128, std::num::ParseIntError> {
        u128::from_str_radix(input, 16)
    }

    pub(super) fn from_dec(input: &str) -> Result<u128, std::num::ParseIntError> {
        u128::from_str_radix(input, 10)
    }

    fn from_bin(input: &str) -> Result<u128, std::num::ParseIntError> {
        u128::from_str_radix(input, 2)
    }

    fn clear_underscores(input: &str) -> String {
        let res = input.replace("_", "");
        res
    }

    fn hex_primary(input: Fragment) -> IResult<Fragment, u128> {
        map_res(
            preceded(
                tag("0x"),
                preceded(
                    peek(take_while_m_n(1, 1, |c: char| c.is_ascii_hexdigit())),
                    take_while1(|c: char| c.is_ascii_hexdigit() || c == '_'),
                ),
            ),
            |frag: Fragment| -> Result<u128, ParseIntError> {
                let mut s = String::from(frag.source());
                s = Self::clear_underscores(&s);
                Self::from_hex(&s)
            },
        )(input)
    }

    fn bin_primary(input: Fragment) -> IResult<Fragment, u128> {
        map_res(
            preceded(
                tag("0b"),
                preceded(
                    peek(take_while_m_n(1, 1, |c: char| c == '1' || c == '0')),
                    is_a("10_"),
                ),
            ),
            |frag: Fragment| -> Result<u128, ParseIntError> {
                let mut s = String::from(frag.source());
                s = Self::clear_underscores(&s);
                Self::from_bin(&s)
            },
        )(input)
    }

    pub(super) fn dec_primary(input: Fragment) -> IResult<Fragment, u128> {
        map_res(
            preceded(
                peek(take_while_m_n(1, 1, |c: char| c.is_ascii_digit())),
                take_while1(|c: char| c.is_ascii_digit() || c == '_'),
            ),
            |frag: Fragment| -> Result<u128, ParseIntError> {
                //dbg!(frag);
                let mut s = String::from(frag.source());
                s = Self::clear_underscores(&s);
                Self::from_dec(&s)
            },
        )(input)
    }

    /// Parse a numerical literal to a value.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        let constructor = |(frag, val)| Self::new(frag, val);
        alt((
            map(with_input(Self::bin_primary), constructor),
            map(with_input(Self::hex_primary), constructor),
            map(with_input(Self::dec_primary), constructor),
        ))(input)
    }
}

impl<'s> HasFragment<'s> for NumLit<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}

impl<'s> ToExpression<'s> for NumLit<'s> {
    fn create_expr(self) -> Expression<'s> {
        Expression::NumLit(self)
    }
}
