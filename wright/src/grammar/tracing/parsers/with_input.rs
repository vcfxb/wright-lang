use nom::{IResult, Offset, Slice};
use crate::grammar::parsers::with_input_call;
use crate::grammar::tracing::input::OptionallyTraceable;
use nom::lib::std::ops::RangeTo;

/// This type represents the output of a parser and the input consumed to produce that output.
pub struct WithInputConsumed<I, T> {
    consumed: I,
    produced: T
}

impl<I, T> WithInputConsumed<I, T>
where I: OptionallyTraceable + Offset + Slice<RangeTo<usize>>
{
    /// Name used to represent this function in traces.
    pub const TRACE_NAME: &'static str = "with_input";

    /// constructor
    fn new(consumed: I, produced: T) -> Self {
        Self {consumed, produced}
    }

    // FIXME: link
    /// Call a parser and return a result with input.
    /// This is a more specific version of the generic [`with_input parser`]().
    pub fn with_input<F>(parser: F) -> impl Fn(I) -> IResult<I, Self>
    where F: Fn(I) -> IResult<I, T> {
        move |input| {
            let i = input.trace_start_clone(Self::TRACE_NAME);
            with_input_call(parser, i).map(|(rem, (cons, node))| {
                (
                    rem.trace_end_clone(Self::TRACE_NAME, true),
                    Self::new(cons.trace_end_clone(Self::TRACE_NAME, true), node)
                )
            }).map_err(|err|
                err.map_input(|i: I| i.trace_end_clone(Self::TRACE_NAME, false))
            )
        }
    }

    // FIXME: link
    /// Call the associated [with_input](#with_input) function.
    pub fn with_input_call<F>(parser: F, input: I) -> IResult<I, Self>
    where F: Fn(I) -> IResult<I, T> {
        Self::with_input(parser)(input)
    }
}

impl<I, T> Into<(I, T)> for WithInputConsumed<I, T> {
    fn into(self) -> (I, T) {
        (self.consumed, self.produced)
    }
}