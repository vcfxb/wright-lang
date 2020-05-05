use crate::grammar::ast::{eq::AstEq, Block, Expression, Statement};
use crate::grammar::model::{Fragment, HasFragment};
use crate::grammar::parsers::whitespace::token_delimiter;
use crate::grammar::parsers::with_input;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, pair, terminated};
use nom::IResult;

impl<'s> Block<'s> {
    /// Start of block in source code.
    pub const START_DELIMITER: &'static str = "{";
    /// End of a block in source code.
    pub const END_DELIMITER: &'static str = "}";

    fn inner(
        input: Fragment<'s>,
    ) -> IResult<Fragment<'s>, (Vec<Statement<'s>>, Option<Box<Expression<'s>>>)> {
        pair(
            many0(terminated(Statement::parse, token_delimiter)),
            opt(map(Expression::parse, |e| Box::new(e))),
        )(input)
    }

    /// Parse a block in source code. A block is delimited by `{` and `}`.
    /// Blocks contain a series of statements, and optionally an expression at
    /// the end. There may be no statements, in which case it is considered to
    /// be a series of length 0.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(
            with_input(delimited(
                pair(tag(Self::START_DELIMITER), token_delimiter),
                Self::inner,
                pair(token_delimiter, tag(Self::END_DELIMITER)),
            )),
            |(consumed, (statements, terminal))| Self {
                frag: consumed,
                statements,
                result: terminal,
            },
        )(input)
    }
}

impl<'s> HasFragment<'s> for Block<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}

impl<'s> Into<Expression<'s>> for Block<'s> {
    fn into(self) -> Expression<'s> {
        Expression::Block(self)
    }
}

impl<'s> AstEq for Block<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        AstEq::ast_eq(&fst.result, &snd.result) && AstEq::ast_eq(&fst.statements, &snd.statements)
    }
}
