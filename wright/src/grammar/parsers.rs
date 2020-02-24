use nom::{IResult, Offset, Slice};
use std::ops::RangeTo;

/// Wright literal value parsers.
pub(crate) mod literals;

/// Wright expression parsers.
pub(crate) mod expression;

/// Wright comment and whitespace parsers.
pub(crate) mod whitespace;

/// Whitespace module tests
mod whitespace_tests;

/// Call a parser and on success, return the input consumed as well as the
/// result.
/// This function essentially returns a copy of the parser that returns
/// `(I, (I, O))`, or `(remaining input, (input consumed, output))`
pub fn with_input<F, I, O>(parser: F) -> impl Fn(I) -> IResult<I, (I, O)>
    where
        I: Clone + Offset + Slice<RangeTo<usize>>,
        F: Fn(I) -> IResult<I, O> {
    move |input: I| -> IResult<I, (I, O)> {
        let i = input.clone();
        match parser(i) {
            Ok((remaining, result)) => {
                let index = input.offset(&remaining);
                Ok((remaining, (input.slice(..index), result)))
            },
            Err(e) => Err(e)
        }
    }
}