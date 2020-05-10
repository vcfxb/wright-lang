use crate::grammar::ast::{eq::AstEq, Expression, Identifier, ScopedName};
use crate::grammar::model::{HasSourceReference, WrightInput};
use crate::grammar::parsers::whitespace::token_delimiter;
use crate::grammar::parsers::with_input;
use crate::grammar::tracing::parsers::tag;
use crate::grammar::tracing::{parsers::map, trace_result};
use nom::multi::many0;
use nom::sequence::{delimited, pair, terminated, preceded};
use nom::IResult;

impl<T: std::fmt::Debug + Clone> ScopedName<T> {
    /// The scope separator string.
    pub const SEPARATOR: &'static str = "::";

    /// The name this will appear under when tracing a parse.
    pub const TRACE_NAME: &'static str = "ScopedName";
}

impl<I: WrightInput> ScopedName<I> {
    /// Parses a ScopedName from the given input fragment.
    pub fn parse(input: I) -> IResult<I, Self> {
        let res: IResult<I, Self> = map(
            with_input(pair(
                Identifier::parse,
                many0(preceded(
                    delimited(token_delimiter, tag(Self::SEPARATOR), token_delimiter),
                    Identifier::parse,
                )),
            )),
            |(consumed, (fst, following))| {
                let mut list = following.clone();
                list.insert(0, fst);
                let index = list.len() -1;
                Self {
                    source: consumed,
                    path: (&list[..index]).into(),
                    name: list[index].clone(),
                }
            },
        )(input.trace_start_clone(Self::TRACE_NAME));

        trace_result(Self::TRACE_NAME, res)
    }
}

impl<I: std::fmt::Debug + Clone> HasSourceReference<I> for ScopedName<I> {
    fn get_source_ref(&self) -> &I {
        &self.source
    }
}

impl<I: std::fmt::Debug + Clone> Into<Expression<I>> for ScopedName<I> {
    fn into(self) -> Expression<I> {
        Expression::ScopedName(self)
    }
}

impl<T: std::fmt::Debug + Clone + PartialEq> AstEq for ScopedName<T> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        AstEq::ast_eq(&fst.path, &snd.path) && AstEq::ast_eq(&fst.name, &snd.name)
    }
}
