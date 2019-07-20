pub use codespan::{ByteSpan, CodeMap};
use codespan::{ByteIndex, RawIndex};

pub mod tokens;
pub mod ast;
pub mod parser;

#[derive(Debug, Copy, Clone, Default)]
pub struct Properties {
    bytespan: ByteSpan,
}

impl Properties {
    fn new_span(span: ByteSpan) -> Self {
        Self { bytespan: span }
    }
    /// Create new Properties instance.
    pub fn new(start: usize, end: usize) -> Self {
        Self::new_span(ByteSpan::new(ByteIndex(start as RawIndex),
                                     ByteIndex(end as RawIndex)))
    }
    /// Return the ByteSpan of this element.
    pub fn span(&self) -> ByteSpan {self.bytespan}
}