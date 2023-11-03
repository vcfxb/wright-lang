//! Structures used for representing expressions in wright source code.

use self::primary::Primary;

pub mod block;
pub mod literal;
pub mod parentheses;
pub mod primary;

/// Enumeration of all the different kinds of expression in wright.
#[derive(Debug)]
pub enum Expression<'src> {
    /// A literal in source code. 
    Primary(Primary<'src>),
    // Block(block::Block<'src>),
}
