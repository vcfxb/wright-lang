use nom::branch::Alt;
use nom::error::{ParseError, ErrorKind};
use nom::IResult;
use crate::grammar::tracing::input::OptionallyTraceable;
use crate::grammar::tracing::trace_result;


/// A traced version of nom's
/// [`alt`](https://docs.rs/nom/5.1.1/nom/branch/fn.alt.html)
/// combinator.
pub fn alt<I, O, List>(l: List) -> impl Fn(I) -> IResult<I, O, (I, ErrorKind)>
where
    I: Clone + OptionallyTraceable,
    List: Alt<I, O, (I, ErrorKind)>
{
    let trace = "alt";
    move |input: I| {
        let input= input.trace_start_clone(trace);
        let res= l.choice(input);
        trace_result(trace, res)
    }
}