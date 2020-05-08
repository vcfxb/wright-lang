use crate::grammar::ast::{eq::AstEq, Expression, Identifier, ScopedName};
use crate::grammar::model::{Fragment, HasFragment};
use crate::grammar::parsers::whitespace::token_delimiter;
use nom::bytes::complete::tag;
use nom::multi::{many0};
use nom::sequence::{delimited, pair, terminated};
use nom::{IResult, Err};
use crate::grammar::tracing::{input::OptionallyTraceable, parsers::{
    with_input::WithInputConsumed,
    map::map
}, trace_result};

impl<'s> ScopedName<'s> {
    /// The scope separator string.
    pub const SEPARATOR: &'static str = "::";

    /// The name this will appear under when tracing a parse.
    pub const TRACE_NAME: &'static str = "ScopedName";

    /// Parses a ScopedName from the given input fragment.
    pub fn parse<I: OptionallyTraceable>(input: I) -> IResult<I, Self> {
        let mut i: I = input.clone();
        i.trace_start(Self::TRACE_NAME);
        let res: IResult<I, Self> = map(
            WithInputConsumed::with_input(pair(
                many0(terminated(
                    Identifier::parse,
                    delimited(token_delimiter,
                        tag(Self::SEPARATOR),
                        token_delimiter
                    )
                )),
                Identifier::parse,
            )),
            |wi| {
                let (consumed, (path, name)) = wi.into();
                Self {
                    frag: consumed,
                    path,
                    name,
                }
            },
        )(i);

        trace_result(Self::TRACE_NAME, res)
    }
}

impl<'s> HasFragment<'s> for ScopedName<'s> {
    fn get_fragment_reference(&self) -> &Fragment<'s> {
        &self.frag
    }
}

impl<I> Into<Expression<I>> for ScopedName<I> {
    fn into(self) -> Expression<I> {
        Expression::ScopedName(self)
    }
}

impl<T> AstEq for ScopedName<T> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        AstEq::ast_eq(&fst.path, &snd.path) && AstEq::ast_eq(&fst.name, &snd.name)
    }
}
