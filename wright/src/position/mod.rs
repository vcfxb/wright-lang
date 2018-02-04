//! Module used by parser and lexer for tracking position in source.

/// Span is a class used to store pairs of Positions.
pub mod span;

#[derive(Debug, Copy, Clone)]
/// Position of the reading head in the file, indexed starting at 1.
/// Mainly for user interfacing.
pub struct Position {
    line: usize,
    col: usize,
}

impl Default for Position {
    /// Defaults to initial location.
    fn default() -> Self { Position::new() }
}

impl Position {
    /// Constructor, setting read-head at beginning of file.
    pub fn new() -> Self { Position { line: 1, col: 1, } }
    /// Increments internal line counter.
    pub fn increment_line(&mut self) {
        self.line += 1;
        self.col = 1;
    }
    /// Increments column.
    pub fn increment_column(&mut self) { self.col += 1; }
    /// Increments column by a certain amount.
    pub fn increment_column_by(&mut self, amount: usize) {self.col += amount;}
    /// Increments line by a certain amount.
    pub fn increment_line_by(&mut self, amount: usize) {self.line += amount; self.col = 1;}
    /// Decrements column.
    /// Will not let column fall below 1.
    pub fn decrement_column(&mut self) { if self.col > 1 {self.col -= 1;}}
    /// Decrements column by a certain amount.
    /// Will not let column fall below 1.
    pub fn decrement_column_by(&mut self, amount: usize) {if self.col-amount>=1{self.col-=amount;}}
    /// Returns line number.
    pub fn get_line(&self) -> usize { return self.line; }
    /// Returns column number.
    pub fn get_col(&self) -> usize { return self.col; }

}