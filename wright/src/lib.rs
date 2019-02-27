#![warn(missing_copy_implementations)]
#![warn(missing_docs)]
//! The Wright programming language crate.
//!

pub mod codemap;
use codemap::CodeMap;

pub mod version;
pub mod grammar;
pub mod cli;

/// Enum of possible intermediate representations which can be emited.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Emit {
    /// Tokens, or Lexemes. [see wikipedia](https://en.wikipedia.org/wiki/Lexical_analysis)
    Tokens,
    /// The Abstract Syntax Tree. [see wikipedia](https://en.wikipedia.org/wiki/Abstract_syntax_tree)
    AbstractSyntaxTree,
}

pub fn call_files(filenames: Vec<&str>, run: bool, emits: Vec<Emit>, verbose: bool) -> i32 {
    unimplemented!();
}
