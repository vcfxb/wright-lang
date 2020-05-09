use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take_while1, take_while_m_n},
    combinator::{map_res, peek},
    sequence::preceded,
    IResult
};

use crate::grammar::{ast::NumLit};

use crate::grammar::ast::{eq::AstEq, Expression};
use crate::grammar::model::{HasSourceReference, WrightInput};
use crate::grammar::tracing::parsers::map::map;
use std::num::ParseIntError;
use crate::grammar::tracing::input::OptionallyTraceable;
use crate::grammar::tracing::trace_result;
use crate::grammar::parsers::with_input;
use std::fmt::Debug;

impl<T: Debug + Clone> NumLit<T> {
    /// Name used to refer to this parser in traces.
    pub const TRACE_NAME: &'static str = "NumLit";
}

impl<'a, I: WrightInput<'a>> NumLit<I> {
    fn new(source: I, num: u128) -> Self {
        Self { source, inner: num }
    }

    /// Convert a number from a string using base 16.
    fn from_hex(input: &str) -> Result<u128, std::num::ParseIntError> {
        u128::from_str_radix(input, 16)
    }

    /// Convert a number from a string using base 10.
    pub(super) fn from_dec(input: &str) -> Result<u128, std::num::ParseIntError> {
        u128::from_str_radix(input, 10)
    }

    /// Convert a number from a string using base 2.
    fn from_bin(input: &str) -> Result<u128, std::num::ParseIntError> {
        u128::from_str_radix(input, 2)
    }

    /// Remove all underscores from a string.
    fn clear_underscores(input: &str) -> String {
        let res = input.replace("_", "");
        res
    }

    /// Parse a properly formatted hexadecimal number.
    fn hex_primary(input: I) -> IResult<I, u128> {
        map_res(
            preceded(
                tag("0x"),
                preceded(
                    peek(take_while_m_n(1, 1, |c: char| c.is_ascii_hexdigit())),
                    take_while1(|c: char| c.is_ascii_hexdigit() || c == '_'),
                ),
            ),
            |source: I| -> Result<u128, ParseIntError> {
                let mut s = String::from(source.into());
                s = Self::clear_underscores(&s);
                Self::from_hex(&s)
            },
        )(input)
    }

    /// Parse a properly formatted binary number.
    fn bin_primary(input: I) -> IResult<I, u128> {
        map_res(
            preceded(
                tag("0b"),
                preceded(
                    peek(take_while_m_n(1, 1, |c: char| c == '1' || c == '0')),
                    is_a("10_"),
                ),
            ),
            |source: I| -> Result<u128, ParseIntError> {
                let mut s: String = source.into();
                s = Self::clear_underscores(&s);
                Self::from_bin(&s)
            },
        )(input)
    }

    /// Parse a properly formatted positive decimal integer.
    pub(super) fn dec_primary(input: I) -> IResult<I, u128> {
        map_res(
            preceded(
                peek(take_while_m_n(1, 1, |c: char| c.is_ascii_digit())),
                take_while1(|c: char| c.is_ascii_digit() || c == '_'),
            ),
            |source: I| -> Result<u128, ParseIntError> {
                let mut s = source.into();
                s = Self::clear_underscores(&s);
                Self::from_dec(&s)
            },
        )(input)
    }

    /// Parse a numerical literal to a value.
    pub fn parse(input: I) -> IResult<I, Self> {
        let constructor = |(source, num)| Self::new(source, num);
        let res = alt((
            map(with_input(Self::bin_primary), constructor),
            map(with_input(Self::hex_primary), constructor),
            map(with_input(Self::dec_primary), constructor),
        ))(input.trace_start_clone(Self::TRACE_NAME));
        trace_result(Self::TRACE_NAME, res)
    }
}

impl<I: Debug + Clone> HasSourceReference<I> for NumLit<I> {
    fn get_source_ref(&self) -> &I {
        &self.source
    }
}

impl<I: Debug + Clone> Into<Expression<I>> for NumLit<I> {
    fn into(self) -> Expression<I> {
        Expression::NumLit(self)
    }
}

impl<I: Debug + Clone> AstEq for NumLit<I> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        fst.inner == snd.inner
    }
}
