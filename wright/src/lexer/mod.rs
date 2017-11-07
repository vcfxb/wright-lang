//! Lexer Module.
use std::fmt;
/// Module used for tracking read-head position in file.
pub mod position;
use lexer::position::*;
/// Module of functions for checking characters.
pub mod char_tests;
use lexer::char_tests::*;

#[derive(Debug, Clone)]
// todo: Docs
///
pub struct Lexer {
    current_position: Position,
    pub source: String,
    pub tokens: Vec<String>,
}

impl Lexer {
    /// Constructor
    pub fn new(content: String) -> Self {
        Lexer {
            current_position: Position::new(),
            source: content,
            tokens: vec![],
        }
    }
    /// Tokenizes `self.source` and stores to `self.tokens`.
    pub fn lex(&mut self) -> Result<(), LexerError> {
        let mut current_token = String::new();
        let chars = self.source.chars();

        Ok(())
    }

}

#[derive(Debug, Clone)]
/// Structure for lexer errors.
pub struct LexerError {
    pub position: Position,
    pub info: String,
    pub line: String,
    // not pub; access only in this mod.
    arrow_str: String
}

impl LexerError {
    /// Constructor.
    pub fn new(arg_position: Position, current_line: String) -> Self {
        LexerError {
            position: arg_position,
            info: String::new(),
            line: current_line,
            arrow_str: String::new()
        }
    }
    /// Sets info string based on an expected character and the character that was found.
    /// Auto-generates error message.
    pub fn set_info(&mut self, expected: char, found: char) {
        self.info = format!("Expected {:?} found {:?}. ", expected, found);
        let current_line_borrow = self.line.clone();
        for c in current_line_borrow.chars().take(self.position.col-1) {
            match c {
                '\t' => self.arrow_str.push('\t'),
                _ =>  self.arrow_str.push(' '),
            }
        }
        self.arrow_str.push('^');
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LexerError: line {}, column {}: {}\n\t{}\n\t{}",
                self.position.line,
                self.position.col,
                self.info,
                self.line,
                self.arrow_str)
    }
}