//! Structure and implementation relating to file names used throughout the wright compiler and tooling.

use derive_more::Display;
use std::path::PathBuf;

/// Used to represent different file names used throughout this crate.
#[derive(Debug, Display, Clone)]
pub enum FileName {
    /// A real file on the user's computer.
    #[display("{}", "_0.display()")]
    Real(PathBuf),
    /// A named test-case in this crate's source code.
    Test(&'static str),
    // /// The interactive Wright repl.
    // #[display(fmt = "REPL:{}", line_number)]
    // Repl { line_number: usize },
    /// An un-named test case in this crate's source code.
    #[display("<NO_NAME>")]
    None,
}
