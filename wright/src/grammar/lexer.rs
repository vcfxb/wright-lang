use codespan::{Span, ByteIndex};

/// The lexer model. Abstract data types that are used in the lexer.
pub mod model;
pub use model::*;

/// Wright language literal tokens types.
pub mod literals;

/// Parsers for single characters in source code.
pub mod single;

/// Utilities for parsing source code.
pub mod keyword;

#[derive(Debug)]
/// Token type, representing a token in source code.
pub struct Token {
    span: Span,
    rule: &'static str,
    state: TokenState
}

impl Token {
    /// Construct a new token.
    pub fn new(start: ByteIndex, end: ByteIndex, rule: &'static str, state: TokenState) -> Self {
        Self {span: Span::new(start, end), rule, state}
    }

    /// Construct a new token with the None state.
    pub fn new_stateless(start: ByteIndex, end: ByteIndex, rule: &'static str) -> Self {
        Self::new(start, end, rule, TokenState::None)
    }

    /// Retrieve this token's span.
    pub fn get_span(&self) -> Span {self.span}

    /// Get the type of this token.
    pub fn get_rule(&self) -> &'static str {self.rule }

    /// Get the state stored with this token.
    pub fn get_state(&self) -> &TokenState {&self.state}
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}: bytes: [{}, {}). state: {:?}]",
               self.rule,
               self.span.start(),
               self.span.end(), self.state)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
/// State that can be stored with a token of wright source code.
pub enum TokenState {
    /// String state, usually used for string literals.
    Str(String),
    /// Num state, usually used for numeric literals.
    Num(u128),
    /// Bool state, usually used for boolean literals.
    Bool(bool),
    /// Char state, usually used for char literals.
    Char(char),
    /// None or empty state.
    None,
}