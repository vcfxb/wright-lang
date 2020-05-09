use crate::grammar::ast::eq::AstEq;
use crate::grammar::ast::Statement;
use crate::grammar::ast::{Expression, ExpressionStatement};
use crate::grammar::model::{Fragment, HasSourceReference};
use crate::grammar::parsers::whitespace::token_delimiter;
use crate::grammar::parsers::with_input;
use nom::character::complete::char as ch;
use nom::combinator::map;
use nom::sequence::{pair, terminated};
use nom::IResult;

impl<'s> ExpressionStatement<'s> {
    /// Parse an expression followed by a semicolon in source code.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(
            with_input(terminated(
                Expression::parse,
                pair(token_delimiter, ch(Statement::TERMINATOR)),
            )),
            move |(consumed, result)| Self {
                source: consumed,
                inner: Box::new(result),
            },
        )(input)
    }
}

impl<'s> HasSourceReference<'s> for ExpressionStatement<'s> {
    #[inline]
    fn get_source_ref(&self) -> &Fragment<'s> {
        &self.source
    }
}

impl<'s> AstEq for ExpressionStatement<'s> {
    #[inline]
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        AstEq::ast_eq(&fst.inner, &snd.inner)
    }
}
