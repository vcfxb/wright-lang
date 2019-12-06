
use codespan::Span;

use nom::{
    character::{

    },
    bytes::complete::{
        tag,
    },
    IResult,
};

/// Numerical literal in wright source code.
/// i.e. `10`, `0xCa1a0`, `0b0101_0101`, `100_000`
#[derive(Copy, Clone, Debug)]
pub struct NumLit {
    span: Span,
    inner: u128,
}

impl NumLit {
    fn new(sp: Span, num: u128) -> Self {
        Self {span: sp, inner: num}
    }

    /// Get the span associated with this numerical literal.
    pub fn span(&self) -> Span {self.span}

    /// Get the value of this numerical literal.
    pub fn get(&self) -> u128 {self.inner}

    fn from_hex(input: &str) -> Result<u128, std::num::ParseIntError> {
        u128::from_str_radix(input, 16)
    }

    fn from_dec(input: &str) -> Result<u128, std::num::ParseIntError> {
        u128::from_str_radix(input, 10)
    }

    fn from_bin(input: &str) -> Result<u128, std::num::ParseIntError> {
        u128::from_str_radix(input, 2)
    }

    fn clear_underscores(input: &str) -> String {
        input.replace("_", "")
    }

    fn hex_primary(input: &str) -> IResult<&str, u128> {
        unimplemented!()
    }

    /// Parse a numerical literal to a value.
    pub fn parse(input: &str) -> IResult<&str, u128> {
        unimplemented!()
    }
}