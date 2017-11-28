#[derive(Debug, Copy, Clone)]
/// Position of the reading head in the file, indexed starting at 1.
/// Mainly for user interfacing.
pub struct Position {
    pub line: usize,
    pub col: usize,
}

impl Default for Position {
    /// defaults to initial location
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
    /// Decrements column.
    /// Will not let column fall below 1.
    pub fn decrement_column(&mut self) { if self.col > 1 {self.col -= 1;}}

}