//! Representation for literal expressions in wright source code.

use crate::parser::{
    state::ParserState,
    util::{map::map_node_type, BoxedParserFn, NodeParserOption, NodeParserResult, first_successful::first_sucessful, discard_error::discard_errors, erase::erase},
};

use self::{
    boolean::{parse_boolean_literal, BooleanLiteral},
    integer::{parse_integer_literal, IntegerLiteral},
};

pub mod boolean;
pub mod integer;
// pub mod string;
// pub mod character;

#[derive(Debug)]
pub enum Literal<'src> {
    /// An integer literal in source code.
    Integer(IntegerLiteral<'src>),
    /// A boolean literal in source code.
    Boolean(BooleanLiteral<'src>),
}

/// Convenience function for converting a child parser to one that is erased and generates
///  [`Literal`]s in [`NodeParserOption`]s. 
fn convert_to_literal_parser<'src, PF, LC, N>(parser_function: PF, literal_conversion: LC) -> BoxedParserFn<'src, NodeParserOption<Literal<'src>>> 
where 
    PF: (Fn(&mut ParserState<'src>) -> NodeParserResult<N>) + 'src,
    LC: (Fn(N) -> Literal<'src>) + 'src,
{
    erase(discard_errors(map_node_type(parser_function, literal_conversion)))
}

/// Parse a literal from source code.
pub fn parse_literal<'src>(
    parser_state: &mut ParserState<'src>,
) -> NodeParserOption<Literal<'src>> 
{
    // Make a parser that finds the first successfull literal parse. 
    let parser = first_sucessful(vec![
        convert_to_literal_parser(parse_integer_literal, Literal::Integer),
        convert_to_literal_parser(parse_boolean_literal, Literal::Boolean),
    ]);

    // Call that parser. 
    (parser)(parser_state)
}
