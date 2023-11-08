//! Parser mapping utilities.

use crate::parser::state::ParserState;

use super::{BoxedParserFn, NodeParserResult, ParserSuccess};

/// Create a [Box]xed function (dyn [`Fn`]) that maps the output of a parser function through another function.
pub fn map<'src, PF, MF, O1, O2>(parser_function: PF, map_function: MF) -> BoxedParserFn<'src, O2>
where
    PF: (Fn(ParserState<'src>) -> O1) + 'static,
    MF: (Fn(O1) -> O2) + 'static,
{
    Box::new(move |parser_state: ParserState| (map_function)((parser_function)(parser_state)))
}

/// Map specifically the node produced by a parser function. 
pub fn map_node_type<'src, PF, MF, N1, N2>(
    parser_function: PF,
    map_function: MF,
) -> BoxedParserFn<'src, NodeParserResult<'src, N2>>
where
    PF: (Fn(ParserState<'src>) -> NodeParserResult<N1>) + 'static,
    MF: (Fn(N1) -> N2) + 'static,
{
    Box::new(move |parse_state: ParserState<'_>| {
        // Run the parser and get the result.
        let parse_result = (parser_function)(parse_state);

        // Map the node type
        parse_result.map(| ParserSuccess { updated_parser_state, ast_node} | {
                ParserSuccess {
                    updated_parser_state,
                    ast_node: ((map_function)(ast_node)),
                }
            },
        )
    })
}
