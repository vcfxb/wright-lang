use crate::grammar::model::{HasFragment, Fragment};
use crate::grammar::ast::{ExpressionStatement, Expression};
use crate::grammar::ast::eq::ASTEq;
use nom::IResult;
use nom::sequence::{terminated, pair};
use nom::combinator::map;
use nom::character::complete::char as ch;
use crate::grammar::parsers::with_input;
use crate::grammar::ast::Statement;
use crate::grammar::parsers::whitespace::token_delimiter;

impl<'s> ExpressionStatement<'s> {
    /// Parse an expression followed by a semicolon in source code.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(
            with_input(
                terminated(
                    Expression::parse,
                    pair(token_delimiter,ch(Statement::SEMICOLON)))
            ),
            move |(consumed, result)| Self {
                frag: consumed,
                inner: Box::new(result),
            }
        )(input)
    }
}

impl<'s> HasFragment<'s> for ExpressionStatement<'s> {
    #[inline]
    fn get_fragment(&self) -> Fragment<'s> {self.frag}
}

impl<'s> ASTEq for ExpressionStatement<'s> {
    #[inline]
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        ASTEq::ast_eq(&fst.inner, &snd.inner)
    }
}