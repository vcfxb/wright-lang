use crate::grammar::ast::{eq::AstEq, Block, Expression, Statement};
use crate::grammar::model::{Fragment, HasFragment};
use crate::grammar::parsers::expression::ToExpression;
use nom::IResult;
use nom::combinator::{map, opt};
use crate::grammar::parsers::with_input;
use nom::sequence::{pair, terminated, delimited};
use nom::bytes::complete::tag;
use crate::grammar::parsers::whitespace::token_delimiter;
use nom::multi::many0;

impl<'s> Block<'s> {
    /// Block delimiters. Should not change.
    pub const DELIMITERS: (&'static str, &'static str) = ("{", "}");

    fn inner(input: Fragment<'s>) -> IResult<Fragment<'s>, (Vec<Statement<'s>>, Option<Box<Expression<'s>>>)> {
        pair(
            many0(terminated(Statement::parse, token_delimiter)),
            opt(map(Expression::parse, |e| Box::new(e)))
        )(input)
    }

    /// Parse a block in source code.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(with_input(delimited(
                pair(tag(Self::DELIMITERS.0), token_delimiter),
                Self::inner,
                pair(token_delimiter, tag(Self::DELIMITERS.1))
            )),
            |(consumed, (statements, terminal))| {
                Self {
                    frag: consumed,
                    statements,
                    result: terminal
                }
        })(input)
    }
}

impl<'s> HasFragment<'s> for Block<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}

impl<'s> ToExpression<'s> for Block<'s> {
    fn create_expr(self) -> Expression<'s> {
        Expression::Block(self)
    }
}

impl<'s> AstEq for Block<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        AstEq::ast_eq(&fst.result, &snd.result) && AstEq::ast_eq(&fst.statements, &snd.statements)
    }
}
