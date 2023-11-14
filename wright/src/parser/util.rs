//! Parsing utility functions used throughout the parser to make the process of parsing easier.

use super::{error::ParserError, state::ParserState};

pub mod discard_error;
pub mod first_successful;
pub mod map;
pub mod erase;
pub mod ignore;

/// A [`Result`] returned from an AST node parser.
pub type NodeParserResult<Node, Error = ParserError> = Result<Node, Error>;

/// An [`Option`] returned from an AST node parser.
pub type NodeParserOption<Node> = Option<Node>;

/// Type alias used to apease the borrow/lifetime checker complaining about HKTs and stuff.
pub type BoxedParserFn<'src, Output> = Box<dyn Fn(&mut ParserState<'src>) -> Output + 'src>;
