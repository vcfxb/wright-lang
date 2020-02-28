use crate::grammar::ast::{Block, Expression, Statement};
use crate::grammar::model::{HasFragment, Fragment};
use crate::grammar::parsers::with_input;
use nom::IResult;
use nom::character::complete::{char as ch, multispace0};
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded};

impl<'s> Block<'s> {
    fn inner(frag: Fragment<'s>) -> IResult<Fragment<'s>, (Vec<Statement<'s>>, Option<Expression<'s>>)> {
        delimited(
            ch('{'),
            pair(
                many0(
                    preceded(
                        multispace0,
                        Statement::parse,
                    ),
                ),
                delimited(
                    multispace0,
                    opt(Expression::parse),
                    multispace0,
                )
            ),
            ch('}'),
        )(frag)
    }

    /// Parse parentheses and the expression between them in source code. Will
    /// ignore any whitespace before and after.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(
            with_input(Self::inner),
            |(parse, (statements, expr))| {
                Block {
                    frag: parse,
                    statements,
                    result: expr.map(Box::new),
                }
            },
        )(input)
    }
}

impl<'s> HasFragment<'s> for Block<'s> {
    fn get_fragment(&self) -> Fragment<'s> {self.frag}
}
