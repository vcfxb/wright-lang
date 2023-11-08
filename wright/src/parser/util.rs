//! Parsing utility functions used throughout the parser to make the process of parsing easier.

use super::{error::ParserError, state::ParserState};

pub mod discard_error;
pub mod first_successful;
pub mod map;
pub mod erase;

/// The output of a successful parse always includes the updated [`ParserState`] after parsing and the
/// parsed AST node.
#[derive(Debug)]
pub struct ParserSuccess<'src, Node> {
    /// The updated state of the parser.
    pub updated_parser_state: ParserState<'src>,

    /// The produced AST node.
    pub ast_node: Node,
}

/// A [`Result`] returned from an AST node parser.
pub type NodeParserResult<'src, Node, Error = ParserError> =
    Result<ParserSuccess<'src, Node>, Error>;

/// An [`Option`] returned from an AST node parser.
pub type NodeParserOption<'src, Node> = Option<ParserSuccess<'src, Node>>;

/// Type alias used to apease the borrow/lifetime checker complaining about HKTs and stuff.
pub type BoxedParserFn<'src, Output> = Box<dyn Fn(ParserState<'src>) -> Output + 'src>;
