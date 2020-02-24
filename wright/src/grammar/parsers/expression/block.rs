use crate::grammar::ast::{Block, Expression, Statement};
use crate::grammar::model::{HasFragment, Fragment};
use nom::IResult;
use nom::character::complete::{char as ch, multispace0};
use nom::combinator::{map, opt, recognize};
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded};
use crate::grammar::parsers::expression::ToExpression;

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
            recognize(Self::inner),
            |parse| {
                let (statements, expr) = Self::inner(parse).unwrap().1;
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

impl<'s> ToExpression<'s> for Block<'s> {
    fn create_expr(self) -> Expression<'s> {
        Expression::Block(self)
    }
}