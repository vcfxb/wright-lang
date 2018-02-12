use super::Position;

#[derive(Debug, Copy, Clone)]
/// Struct used for tracking spans of source code.
pub struct Span {
    /// Starting Position of the span.
    pub start:  Position,
    /// Ending Position of the span.
    pub end: Position,
}

impl Span {
    /// Gets the starting position in the span.
    pub fn get_start(&self) -> Position {self.start}
    /// Gets the ending position in the span.
    pub fn get_end(&self) -> Position {self.end}
    /// Checks if the ending position is after the starting position.
    pub fn is_valid(&self) -> bool {
        self.start.get_line() <= self.end.get_line() && self.start.get_col() <= self.end.get_col()
    }
    /// Checks if the span is more than one line long.
    pub fn is_multiple_lines(&self) -> bool {
        self.start.get_line() < self.end.get_line()
    }
}