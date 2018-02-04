use super::Position;

#[derive(Debug, Copy, Clone)]
/// Struct used for tracking spans of source code.
pub struct Span {
    first:  Position,
    second: Position,
}

impl Span {

}