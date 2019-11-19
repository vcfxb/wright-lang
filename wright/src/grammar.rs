use codespan::{
    ByteIndex,
    Span,
    Files,
    FileId
};

pub mod ast;
pub mod parser;

/// Module for Wright's lexer. Parts of wright's grammar are defined in here, namely:
///
/// - `# ...` is a wright single line comment.
/// - `#* ... *#` is a wright multiline comment.
/// - `#? ...` is a wright doc comment.
/// - `## ... ##` is a wright doc comment over multiple lines.
/// - `#! ...` is a wright doc comment for the module. (like `//!` in rust.)
///
pub mod lexer;
