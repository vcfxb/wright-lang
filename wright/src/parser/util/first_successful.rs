//! The [`FirstSuccessful`] parse combinator takes a series of parsers and applies them in order until one is
//! successful.

use crate::parser::state::ParserState;
use super::{BoxedParserFn, NodeParserOption};

/// Parser combinator that takes a list of [`BoxedParserFn`]s and runs them in order on clean clones of the 
/// imput until one of them succeeds, and returns that one. If none succeed, none is returned. 
pub fn first_sucessful<'src, N: 'src>(parser_functions: Vec<BoxedParserFn<'src, NodeParserOption<'src, N>>>) -> BoxedParserFn<'src, NodeParserOption<'src, N>> {
    Box::new(move |parser_state: ParserState<'_>| {
        parser_functions.iter()
            .filter_map(|parser_function| {
                // Make a clean clone of the parser state. 
                let clean_clone = parser_state.clone();
                // Call the function on the clean clone. 
                (parser_function)(clean_clone)            
            })
            .next()
    })
}
