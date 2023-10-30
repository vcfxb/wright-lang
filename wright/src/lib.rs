#![warn(missing_copy_implementations)]

//! The wright programming language crate. This is being re-written from the ground up as of September 2022.

/// The version of this copy of Wright. 
pub const WRIGHT_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub mod filemap;
pub mod parser;
pub mod solver;
pub mod repl;
