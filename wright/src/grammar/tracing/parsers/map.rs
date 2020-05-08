use crate::grammar::model::Fragment;
use nom::IResult;
use crate::grammar::tracing::input::OptionallyTraceable;

/// A traced version of nom's
/// [`map` function](https://docs.rs/nom/5.1.1/nom/combinator/fn.map.html).
pub fn map<I, P, F, O1, O2>(
    parser: P,
    f: F
) -> impl Fn(I) -> IResult<I, O2>
    where
        I: OptionallyTraceable,
        P: Fn(I) -> IResult<I, O1>,
        F: Fn(O1) -> O2,
{
    move |input: I| {
        let mut i = input.clone();
        i.trace_start("map");
        parser(i).map(|(r, o)| (r, f(o)))
            .map(|(rem, o)| {
                let mut r = rem.clone();
                r.trace_end("map", true);
                (r, o)
            })
    }
}