//! Representation for literal expressions in wright source code.

use crate::parser::{state::ParserState, util::{NodeParserResult, map::map_node_type, NodeParserOption, ParserSuccess, BoxedParserFn}};

use self::{boolean::{BooleanLiteral, parse_boolean_literal}, integer::{IntegerLiteral, parse_integer_literal}};


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

/// Parse a literal from source code. 
pub fn parse_literal<'src>(parser_state: ParserState<'src>) -> NodeParserOption<'src, Literal<'src>> {
    // Make a list of the parsers to attempt in order on fresh clones of the parser state. 
    // Map each parser to the enum constructor to normalize types. 
    let literal_parsers: [BoxedParserFn<'src, NodeParserResult<'src, Literal<'src>>>; 2] = [
        map_node_type(parse_integer_literal, Literal::Integer),
        map_node_type(parse_boolean_literal, Literal::Boolean),
    ];

    for parser_function in literal_parsers.into_iter() {
        // Make a clean state to pass to the child parser. 
        let clean_state = parser_state.clone();

        // Call the parser and handle the result. 
        match (parser_function)(clean_state) {
            // Ignore/handle the output. 
            // Go to the next parser on errors. 
            Err(_) => continue,
            ok @ Ok(_) => return ok.ok(),
        }
    }

    // If none return a sucessful result, return None.
    None
}
