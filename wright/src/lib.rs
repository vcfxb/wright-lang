#![warn(missing_copy_implementations)]
#![warn(missing_docs)]

//! The Wright programming language crate.
//!

pub mod version;
pub mod grammar;
pub mod cli;

use codespan::CodeMap;

/// Enum of possible intermediate representations which can be emited.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Emit {
    /// Tokens, or Lexemes. [see wikipedia](https://en.wikipedia.org/wiki/Lexical_analysis)
    Tokens,
    /// The Abstract Syntax Tree. [see wikipedia](https://en.wikipedia.org/wiki/Abstract_syntax_tree)
    AbstractSyntaxTree,
}

/// Returns exit code.
pub fn call_files(filenames: Vec<&str>, run: bool, emits: Vec<Emit>, verbose: bool) -> i32 {
    let mut map = CodeMap::new();
    for file in filenames {
        if map.add_filemap_from_disk(file).is_err() {
            println!("Could not load file: {}", file);
            return 1;
        }
    }
    call(map, run, emits, verbose)
}

/// Returns exit code.
pub fn call(codemap: CodeMap, run: bool, emits: Vec<Emit>, verbose: bool) -> i32 {
    unimplemented!()
}