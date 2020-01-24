use crate::grammar::ast::{Identifier, BooleanLit, Underscore};
use crate::grammar::model::Fragment;
use nom::IResult;
use nom::combinator::{recognize, verify, map};
use nom::sequence::pair;
use nom::bytes::complete::take_while;
use nom::error::context;

impl<'s> Identifier<'s> {
    /// Reserved words that an identifier must not match.
    pub const RESERVED_WORDS: [&'static str; 3] = [
        BooleanLit::FALSE, BooleanLit::TRUE, Underscore::UNDERSCORE,
    ];

    fn new(frag: Fragment<'s>) -> Self {
        Self { frag }
    }

    fn raw_ident(input: Fragment<'s>) -> IResult<Fragment<'s>, Fragment<'s>> {
        verify(
            recognize(
                pair(
                    take_while(|c: char| c.is_ascii_alphabetic() || c == '_'),
                    take_while(|c: char| c.is_ascii_alphanumeric() || c == '_')
                )
            ),
            |fr: &Fragment<'s>| Self::RESERVED_WORDS.iter()
                .all(|s: &&str| *s != fr.source())
        )(input)
    }

    /// Parse an identifier from source code. Identifiers may include
    /// ASCII alphanumerics and underscores, but must not start with a number.
    /// An Identifier also must not be a reserved word.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        context("expected identifier", map(Self::raw_ident, Self::new))(input)
    }
}