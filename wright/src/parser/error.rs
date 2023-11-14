//! Parser error handling.

use std::ops::Range;

/// An error that can occur during parsing.
#[derive(Debug)]
pub struct ParserError {
    /// The byte index range of the offending line in the file being parsed.
    pub byte_range: Range<usize>,
    /// The type of error.
    pub ty: ParserErrorVariant,
}

/// Different types of errors that can be generated duruing parsing.
#[derive(Debug, Clone, Copy)]
pub enum ParserErrorVariant {
    /// Something was expected and wasn't there.
    Expected(&'static str),

    /// Encountered unterminated multi-line comment.
    UnterminatedMultilineComment,
}
