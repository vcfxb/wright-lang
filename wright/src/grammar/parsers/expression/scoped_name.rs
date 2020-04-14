use crate::grammar::ast::{eq::AstEq, Expression, Identifier, ScopedName};
use crate::grammar::model::{Fragment, HasFragment};
use crate::grammar::parsers::expression::ToExpression;
use crate::grammar::parsers::whitespace::token_delimiter;
use crate::grammar::parsers::with_input;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::separated_nonempty_list;
use nom::sequence::delimited;
use nom::IResult;

impl<'s> ScopedName<'s> {
    /// The scope separator string.
    pub const SEPARATOR: &'static str = "::";

    /// Parses a ScopedName from the given input fragment.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(
            with_input(separated_nonempty_list(
                delimited(token_delimiter, tag(Self::SEPARATOR), token_delimiter),
                Identifier::parse,
            )),
            |(frag, names)| Self {
                frag,
                path: names[..names.len() - 1].to_vec(),
                name: names[names.len() - 1],
            },
        )(input)
    }
}

impl<'s> HasFragment<'s> for ScopedName<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}

impl<'s> ToExpression<'s> for ScopedName<'s> {
    fn create_expr(self) -> Expression<'s> {
        Expression::ScopedName(self)
    }
}

impl<'s> AstEq for ScopedName<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        AstEq::ast_eq(&fst.path, &snd.path) && AstEq::ast_eq(&fst.name, &snd.name)
    }
}
