//! Responsible for keeping track of different files added to the Wright build system.

use codespan_reporting::files::{Files, SimpleFiles};
use derive_more::Display;
use std::path::PathBuf;

/// Used to represent different file names used throughout this crate.
#[derive(Debug, Display, Clone)]
pub enum FileName {
    /// A real file on the user's computer.
    #[display(fmt = "{}", "_0.display()")]
    Real(PathBuf),
    /// A named test-case in this crate's source code.
    Test(&'static str),
    /// An un-named test case in this crate's source code.
    #[display(fmt = "<NO_NAME>")]
    None,
}

/// The file map that we use throughout the rest of this crate.
pub type FileMap = SimpleFiles<FileName, String>;

/// The file id type used to refer to files in the file map.
pub type FileId = <FileMap as Files<'static>>::FileId;
