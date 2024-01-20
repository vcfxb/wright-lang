//! The [`FirstSuccessful`] parse combinator takes a series of parsers and applies them in order until one is
//! successful.

use super::{BoxedParserFn, NodeParserOption};
use crate::parser::state::ParserState;

/// Parser combinator that takes a list of [`BoxedParserFn`]s and runs them in order on the input
/// until one of them succeeds, and returns that one. If none succeed, none is returned.
pub fn first_sucessful<'src, N: 'src>(
    parser_functions: Vec<BoxedParserFn<'src, NodeParserOption<N>>>,
) -> BoxedParserFn<'src, NodeParserOption<N>> {
    Box::new(move |parser_state: &mut ParserState<'_>| {
        parser_functions
            .iter()
            .find_map(|parser_function| {
                // Call the function on the clean clone.
                (parser_function)(parser_state)
            })
    })
}
