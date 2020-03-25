use crate::grammar::ast::eq::ASTEq;
use crate::grammar::ast::Statement;
use crate::grammar::ast::{Expression, ExpressionStatement};
use crate::grammar::model::{Fragment, HasFragment};
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
                frag: consumed,
                inner: Box::new(result),
            },
        )(input)
    }
}

impl<'s> HasFragment<'s> for ExpressionStatement<'s> {
    #[inline]
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}

impl<'s> ASTEq for ExpressionStatement<'s> {
    #[inline]
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        ASTEq::ast_eq(&fst.inner, &snd.inner)
    }
}
