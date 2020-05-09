use crate::grammar::ast::{eq::AstEq, Expression, SelfLit};
use crate::grammar::model::HasSourceReference;
use nom::bytes::complete::tag;
use nom::IResult;
use crate::grammar::tracing::{
    input::OptionallyTraceable,
    parsers::map::map,
    trace_result
};

impl<T: std::fmt::Debug + Clone> SelfLit<T> {
    /// The trace name used in parser tracing.
    pub const TRACE_NAME: &'static str = "SelfLit";

    /// Literal self identifier.
    pub const SELF: &'static str = "self";
}

impl<I: OptionallyTraceable + std::fmt::Debug + Clone> SelfLit<I> {
    fn new(source: I) -> Self {
        Self { source }
    }

    /// Parse a self literal from input.
    pub fn parse(input: I) -> IResult<I, Self> {
        trace_result(
            Self::TRACE_NAME,
            map(
                tag(Self::SELF),
                Self::new
            )(input.trace_start_clone(Self::TRACE_NAME))
        )
    }
}

impl<I: std::fmt::Debug + Clone> Into<Expression<I>> for SelfLit<I> {
    fn into(self) -> Expression<I> {
        Expression::SelfLit(self)
    }
}

impl<I: std::fmt::Debug + Clone> HasSourceReference<I> for SelfLit<I> {
    fn get_source_ref(&self) -> &I {
        &self.source
    }
}

impl<I: std::fmt::Debug + Clone> AstEq for SelfLit<I> {
    #[inline]
    fn ast_eq(_: &Self, _: &Self) -> bool {
        true
    }
}
