use crate::grammar::ast::eq::ASTEq;
use crate::grammar::ast::{Expression, IndexExpression};
use crate::grammar::model::{Fragment, HasFragment};
use crate::grammar::parsers::expression::ToExpression;
use crate::grammar::parsers::whitespace::token_delimiter;
use crate::grammar::parsers::with_input;
use nom::character::complete::char as ch;
use nom::combinator::map;
use nom::sequence::{delimited, pair, terminated};
use nom::IResult;

impl<'s> IndexExpression<'s> {
    /// Square brace characters. Probably should never change.
    pub const DELIMITERS: (char, char) = ('[', ']');

    /// Parse an index expression in wright source code.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(
            with_input(pair(
                terminated(
                    Expression::parse,
                    pair(token_delimiter, ch(Self::DELIMITERS.0)),
                ),
                terminated(
                    Expression::parse,
                    delimited(token_delimiter, ch(Self::DELIMITERS.1), token_delimiter),
                ),
            )),
            move |(consumed, (subject, object))| Self {
                frag: consumed,
                subject: Box::new(subject),
                object: Box::new(object),
            },
        )(input)
    }
}

impl<'s> HasFragment<'s> for IndexExpression<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}

impl<'s> ASTEq for IndexExpression<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        ASTEq::ast_eq(&*fst.subject, &*snd.subject) && ASTEq::ast_eq(&*fst.object, &*snd.object)
    }
}

impl<'s> ToExpression<'s> for IndexExpression<'s> {
    fn create_expr(self) -> Expression<'s> {
        Expression::IndexExpression(self)
    }
}
