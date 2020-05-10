use nom::bytes::complete::tag as nom_tag;
use nom::{InputTake, Compare, InputLength, IResult};
use crate::grammar::tracing::input::OptionallyTraceable;
use crate::grammar::tracing::trace_result;

/// Traced version of nom's
/// [tag](https://docs.rs/nom/5.1.1/nom/bytes/complete/fn.tag.html)
/// parser.
pub fn tag<T, Input>(
    tag: T
) -> impl Fn(Input) -> IResult<Input, Input>
where
    Input: InputTake + Compare<T> + OptionallyTraceable,
    T: InputLength + Clone
{
    let trace = "tag";
    move |input: Input| {
        let i = input.trace_start_clone(trace);
        let res = nom_tag(tag.clone())(i);
        trace_result(trace, res)
    }
}