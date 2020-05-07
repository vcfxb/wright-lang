use crate::grammar::ast::eq::AstEq;
use crate::grammar::ast::{Block, Expression, FuncCall, ScopedName, Parens};
use crate::grammar::model::{Fragment, HasFragment};
use crate::grammar::parsers::whitespace::token_delimiter;
use crate::grammar::parsers::with_input;
use nom::branch::alt;
use nom::character::complete::char as ch;
use nom::combinator::map;
use nom::multi::separated_list;
use nom::sequence::{delimited, pair, terminated};
use nom::IResult;

impl<'s> FuncCall<'s> {
    /// Left parenthesis delimiting argument list. Probably should never change.
    pub const DELIMITER_LEFT: char = '(';

    /// Left parenthesis delimiting argument list. Probably should never change.
    pub const DELIMITER_RIGHT: char = ')';

    /// Comma separating arguments. Probably should never change.
    pub const ARG_SEPARATOR: char = ',';

    fn func_call_primary(input: Fragment<'s>) -> IResult<Fragment<'s>, Expression> {
        alt((
            // map(IndexExpression::parse, Expression::IndexExpression),
            // commented out to avoid possible recursion.
            map(Block::parse, Block::into),
            map(Parens::parse, Parens::into),
            map(ScopedName::parse, ScopedName::into),
        ))(input)
    }

    /// Parse an index expression in wright source code.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(
            with_input(pair(
                terminated(Self::func_call_primary, token_delimiter),
                delimited(
                    ch(Self::DELIMITER_LEFT),
                    separated_list(
                        delimited(token_delimiter, ch(Self::ARG_SEPARATOR), token_delimiter),
                        Expression::parse,
                    ),
                    ch(Self::DELIMITER_RIGHT),
                ),
            )),
            move |(consumed, (func, args))| Self {
                frag: consumed,
                func: Box::new(func),
                args,
            },
        )(input)
    }
}

impl<'s> HasFragment<'s> for FuncCall<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}

impl<'s> AstEq for FuncCall<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        AstEq::ast_eq(&fst.func, &snd.func) && AstEq::ast_eq(&fst.args, &snd.args)
    }
}

impl<'s> Into<Expression<'s>> for FuncCall<'s> {
    fn into(self) -> Expression<'s> {
        Expression::FuncCall(self)
    }
}
