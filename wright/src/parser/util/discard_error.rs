//! Parser function combinator useful for discarding errors. This can transform a parser from returning a [`Result`]
//! to returning an [`Option`].

use crate::parser::state::ParserState;
use super::{BoxedParserFn, NodeParserOption, NodeParserResult};

/// Return a [Box]xed parser function that returns an [`Option`] rather than a [`Result`].
pub fn discard_errors<'src, PF, O>(
    parser_function: PF,
) -> BoxedParserFn<'src, NodeParserOption<'src, O>>
where
    PF: (Fn(ParserState<'src>) -> NodeParserResult<O>) + 'src,
{
    Box::new(move |parser_state: ParserState<'_>| ((parser_function)(parser_state)).ok())
}
