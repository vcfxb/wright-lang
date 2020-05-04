use crate::grammar::ast::{eq::AstEq, Block, Conditional, Expression};
use crate::grammar::model::{Fragment, HasFragment};
use crate::grammar::parsers::whitespace::token_delimiter;
use crate::grammar::parsers::with_input;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded, separated_pair, tuple};
use nom::IResult;

impl<'s> Conditional<'s> {
    /// `if` token in source code. This constant is unlikely to change.
    pub const IF: &'static str = "if";

    /// `else` token in source code. This constant is unlikely to change.
    pub const ELSE: &'static str = "else";

    // parse an expression followed by a block.
    fn parse_branch(input: Fragment<'s>) -> IResult<Fragment<'s>, (Expression<'s>, Block<'s>)> {
        separated_pair(Expression::parse, token_delimiter, Block::parse)(input)
    }

    // discards the output of the prefix (T is never used)
    fn parse_branch_prefixed<T>(
        prefix: impl Fn(Fragment<'s>) -> IResult<Fragment<'s>, T>,
    ) -> impl Fn(Fragment<'s>) -> IResult<Fragment<'s>, (Expression<'s>, Block<'s>)> {
        preceded(prefix, Self::parse_branch)
    }

    // parse the if branch of a conditional expression.
    fn parse_if(input: Fragment<'s>) -> IResult<Fragment<'s>, (Expression<'s>, Block<'s>)> {
        Self::parse_branch_prefixed(pair(tag(Self::IF), token_delimiter))(input)
    }

    // parse an `else if` branch of a conditional expression.
    fn parse_elif(input: Fragment<'s>) -> IResult<Fragment<'s>, (Expression<'s>, Block<'s>)> {
        Self::parse_branch_prefixed(pair(
            tag(Self::IF),
            delimited(token_delimiter, tag(Self::ELSE), token_delimiter),
        ))(input)
    }

    // parse all the elifs in a conditional statement
    fn parse_elifs(input: Fragment<'s>) -> IResult<Fragment<'s>, Vec<(Expression<'s>, Block<'s>)>> {
        many0(preceded(token_delimiter, Self::parse_elif))(input)
    }

    // parse the else branch.
    fn parse_else(input: Fragment<'s>) -> IResult<Fragment<'s>, Block<'s>> {
        preceded(pair(tag(Self::ELSE), token_delimiter), Block::parse)(input)
    }

    /// Parse a conditional expression in source code.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(
            with_input(tuple((
                Self::parse_if,
                Self::parse_elifs,
                preceded(token_delimiter, opt(Self::parse_else)),
            ))),
            |(consumed, ((pa, pb), elifs, terminal))| Self {
                frag: consumed,
                primary: (Box::new(pa), pb),
                elifs,
                default: terminal,
            },
        )(input)
    }
}

impl<'s> HasFragment<'s> for Conditional<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}

impl<'s> AstEq for Conditional<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        AstEq::ast_eq(&fst.default, &snd.default)
            && AstEq::ast_eq(&fst.elifs, &snd.elifs)
            && AstEq::ast_eq(&fst.primary, &snd.primary)
    }
}

impl<'s> Into<Expression<'s>> for Conditional<'s> {
    fn into(self) -> Expression<'s> {Expression::Conditional(self)}
}