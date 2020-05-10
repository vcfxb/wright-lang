use crate::grammar::tracing::input::OptionallyTraceable;
use crate::grammar::tracing::trace_result;
use nom::IResult;

/// A traced version of nom's
/// [`map`](https://docs.rs/nom/5.1.1/nom/combinator/fn.map.html)
/// combinator.
pub fn map<I, P, F, O1, O2>(parser: P, f: F) -> impl Fn(I) -> IResult<I, O2>
where
    I: OptionallyTraceable,
    P: Fn(I) -> IResult<I, O1>,
    F: Fn(O1) -> O2,
{
    move |input: I| {
        let trace = "map";
        let i = input.trace_start_clone(trace);
        trace_result(trace, parser(i).map(|(rem, o)| (rem, f(o))))
    }
}
