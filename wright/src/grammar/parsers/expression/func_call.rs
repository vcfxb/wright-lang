use crate::grammar::ast::eq::AstEq;
use crate::grammar::ast::{Expression, FuncCallExpression};
use crate::grammar::model::{Fragment, HasFragment};
use crate::grammar::parsers::expression::ToExpression;
use crate::grammar::parsers::whitespace::token_delimiter;
use crate::grammar::parsers::with_input;
use nom::character::complete::char as ch;
use nom::combinator::map;
use nom::multi::separated_list;
use nom::sequence::{delimited, pair, terminated};
use nom::IResult;

impl<'s> FuncCallExpression<'s> {
    /// Left parenthesis delimiting argument list. Probably should never change.
    pub const DELIMITER_LEFT: char = '(';

    /// Left parenthesis delimiting argument list. Probably should never change.
    pub const DELIMITER_RIGHT: char = ')';

    /// Comma separating arguments. Probably should never change.
    pub const ARG_SEPARATOR: char = ',';

    /// Parse an index expression in wright source code.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(
            with_input(pair(
                terminated(
                    Expression::parse,
                    pair(token_delimiter, ch(Self::DELIMITER_LEFT)),
                ),
                terminated(
                    separated_list(ch(Self::ARG_SEPARATOR), Expression::parse),
                    delimited(token_delimiter, ch(Self::DELIMITER_RIGHT), token_delimiter),
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

impl<'s> HasFragment<'s> for FuncCallExpression<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}

impl<'s> AstEq for FuncCallExpression<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        AstEq::ast_eq(&fst.func, &snd.func) && AstEq::ast_eq(&fst.args, &snd.args)
    }
}

impl<'s> ToExpression<'s> for FuncCallExpression<'s> {
    fn create_expr(self) -> Expression<'s> {
        Expression::FuncCall(self)
    }
}
