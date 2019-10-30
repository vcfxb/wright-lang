pub use codespan::{Span};
use codespan::{ByteIndex, RawIndex};

pub mod tokens;
pub mod ast;
pub mod parser;

#[derive(Debug, Copy, Clone, Default)]
pub struct Properties;