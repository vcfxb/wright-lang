//! Parser mapping utilities.

use crate::parser::state::ParserState;
use super::{BoxedParserFn, NodeParserResult};

/// Create a [Box]xed function (dyn [`Fn`]) that maps the output of a parser function through another function.
pub fn map<'src, PF, MF, O1, O2>(parser_function: PF, map_function: MF) -> BoxedParserFn<'src, O2>
where
    PF: (Fn(&mut ParserState<'src>) -> O1) + 'static,
    MF: (Fn(O1) -> O2) + 'static,
{
    Box::new(move |parser_state: &mut ParserState| (map_function)((parser_function)(parser_state)))
}

/// Map specifically the node produced by a parser function. 
pub fn map_node_type<'src, PF, MF, N1, N2>(
    parser_function: PF,
    map_function: MF,
) -> BoxedParserFn<'src, NodeParserResult<N2>>
where
    PF: (Fn(&mut ParserState<'src>) -> NodeParserResult<N1>) + 'src,
    MF: (Fn(N1) -> N2) + 'src,
{
    Box::new(move |parser_state: &mut ParserState<'_>| {
        // Run the parser and get the result.
        let parser_result = (parser_function)(parser_state);

        // Map the node type
        parser_result.map(|node| (map_function)(node))
    })
}
