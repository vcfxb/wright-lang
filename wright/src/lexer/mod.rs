//! Lexer Module.

#[derive(Default,Debug)]
/// Position of the reading head in the file, indexed starting at 1.
/// Mainly for user interfacing.
pub struct Position {
    line: u64,
    col: u64,
}

impl Position {
    pub fn initial() -> Position {
        Position { line: 1, col: 1, }
    }
    pub fn increment_line(&mut self) {
        self.line += 1;
        self.col = 1;
    }
    pub fn increment_column(&mut self) {
        self.col += 1;
    }
    pub fn get_line(&self) -> u64 { self.line }
    pub fn get_col(&self) -> u64 { self.col }
}

/// Checks if a char is a digit
pub fn is_digit(c: char) -> bool { c >= '0' && c <= '9' }

/// Checks if a char is a hexadecimal digit.
pub fn is_hex_digit(c: char) -> bool {
    is_digit(c) || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F')
}

/// Checks if a char is a binary digit.
pub fn is_bin_digit(c: char) -> bool { c == '0' || c == '1' }

/// Checks if a char is in the alphabet.
pub fn is_alpha(c: char) -> bool { (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') }

/// Checks if a char is alphnumeric.
pub fn is_alphanumeric(c: char) -> bool { is_digit(c) || is_alpha(c) }

/// Checks if a char is whitespace.
pub fn is_whitespace(c: char) -> bool {
    match c {
        ' ' | '\r' | '\t' | '\n' => true,
        _ => false,
    }
}

#[derive(Debug)]
// todo: Docs
///
pub struct Lexer {
    current_position: Position,
    current_lexeme: String,
    source: String
}