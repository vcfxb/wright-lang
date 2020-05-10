use crate::grammar::tracing::input::OptionallyTraceable;
use nom::{IResult, Offset, Slice};
use std::ops::RangeTo;

/// Wright literal value parsers.
pub(self) mod literal;

/// Wright expression parsers.
pub(self) mod expression;
#[cfg(test)]
mod expression_tests;

/// Wright statement parsers.
pub(crate) mod statement;

/// Wright pattern parsers.
pub(crate) mod pattern;

/// Wright type parsers.
pub(crate) mod types;

/// Wright comment and whitespace parsers.
pub mod whitespace;

/// Whitespace module tests
#[cfg(test)]
mod whitespace_tests;

/// Call a parser and on success, return the input consumed as well as the
/// result.
/// This function essentially returns a copy of the parser that returns
/// `(I, (I, O))`, or `(remaining input, (input consumed, output))`
pub fn with_input<F, I, O>(parser: F) -> impl Fn(I) -> IResult<I, (I, O)>
where
    I: Clone + Offset + Slice<RangeTo<usize>> + OptionallyTraceable,
    F: Fn(I) -> IResult<I, O>,
{
    move |i: I| -> IResult<I, (I, O)> {
        let input: I = i.trace_start_clone("with_input");
        match parser(input.clone()) {
            Ok((remaining, result)) => {
                let index = input.offset(&remaining);
                Ok((
                    remaining.trace_end_clone("with_input", true),
                    (input.slice(..index), result),
                ))
            }
            Err(e) => Err(e.map_input(|i: I| i.trace_end_clone("with_input", false))),
        }
    }
}

// FIXME: test link
/// Call [`with_input`](fn.with_input.html) on a given input fragmen
pub fn with_input_call<F, I, O>(parser: F, input: I) -> IResult<I, (I, O)>
where
    I: Clone + Offset + Slice<RangeTo<usize>> + OptionallyTraceable,
    F: Fn(I) -> IResult<I, O>,
{
    with_input(parser)(input)
}

#[cfg(test)]
mod with_input_test {
    use crate::grammar::parsers::with_input;
    use crate::grammar::tracing::parsers::tag;

    #[test]
    fn test_with_input() {
        let parser = tag("abc");
        match with_input(parser)("abcdef") {
            Ok((rem, (consumed, res))) => {
                assert_eq!(rem, "def");
                assert_eq!(consumed, "abc");
                assert_eq!(res, "abc");
            }
            Err(e) => panic!(e),
        }
    }
}
