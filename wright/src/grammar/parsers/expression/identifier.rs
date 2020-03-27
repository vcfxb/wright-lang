use crate::grammar::ast::{eq::AstEq, Expression, SelfLit};
use crate::grammar::ast::{BooleanLit, Identifier};
use crate::grammar::model::{Fragment, HasFragment};
use crate::grammar::parsers::expression::ToExpression;
use nom::bytes::complete::{take_while, take_while1};
use nom::combinator::{map, recognize, verify};
use nom::error::context;
use nom::sequence::pair;
use nom::IResult;

impl<'s> Identifier<'s> {
    /// Reserved words that an identifier must not match.
    pub const RESERVED_WORDS: [&'static str; 3] =
        [BooleanLit::FALSE, BooleanLit::TRUE, SelfLit::SELF];

    fn new(frag: Fragment<'s>) -> Self {
        Self { frag }
    }

    fn raw_ident(input: Fragment<'s>) -> IResult<Fragment<'s>, Fragment<'s>> {
        verify(
            recognize(pair(
                take_while1(|c: char| c.is_ascii_alphabetic() || c == '_'),
                take_while(|c: char| c.is_ascii_alphanumeric() || c == '_'),
            )),
            |fr: &Fragment<'s>| {
                Self::RESERVED_WORDS
                    .iter()
                    .all(|s: &&str| *s != fr.source())
            },
        )(input)
    }

    /// Parse an identifier from source code. Identifiers may include
    /// ASCII alphanumerics and underscores, but must not start with a number.
    /// An Identifier also must not be a reserved word.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        context("expected identifier", map(Self::raw_ident, Self::new))(input)
    }
}

impl<'s> HasFragment<'s> for Identifier<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}

impl<'s> ToExpression<'s> for Identifier<'s> {
    fn create_expr(self) -> Expression<'s> {
        Expression::Identifier(self)
    }
}

impl<'s> AstEq for Identifier<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        fst.frag.source() == snd.frag.source()
    }
}
