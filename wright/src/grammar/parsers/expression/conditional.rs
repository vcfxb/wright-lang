use crate::grammar::ast::{Conditional, eq::ASTEq, Expression, Block};
use crate::grammar::model::{Fragment, HasFragment};
use nom::IResult;
use nom::bytes::complete::tag;
use nom::combinator::map;
use crate::grammar::parsers::with_input;
use nom::sequence::{preceded, pair, delimited, terminated};
use crate::grammar::parsers::whitespace::token_delimiter;

impl<'s> Conditional<'s> {
    /// `if` token in source code. This constant is unlikely to change.
    pub const IF: &'static str = "if";

    /// `else` token in source code. This constant is unlikely to change.
    pub const ELSE: &'static str = "else";

    // parse an expression followed by a block.
    fn parse_branch(input: Fragment<'s>) -> IResult<Fragment<'s>, (Expression<'s>, Block<'s>)> {
        pair(
            terminated(
                    Expression::parse,
                    token_delimiter
            ),
            Block::parse
        )(input)
    }

    // parse the if branch of a conditional expression.
    fn parse_if(input: Fragment<'s>) -> IResult<Fragment<'s>, (Expression<'s>, Block<'s>)> {
        preceded(
            pair(tag(Self::IF), token_delimiter),
            Self::parse_branch
        )(input)
    }

    

    /// Parse a conditional expression in source code.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(
            with_input(
                pair(
                    preceded(
                        preceded(tag(Self::IF), token_delimiter),
                        pair(
                            Expression::parse,
                            Block::parse
                        )
                    ),

                ),
            || {

        })(input)
    }
}

impl<'s> HasFragment<'s> for Conditional<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}

impl<'s> ASTEq for Conditional<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        ASTEq::ast_eq(&fst.default, &snd.default);
        todo!()
    }
}