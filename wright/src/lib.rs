//! The wright programming language crate. This is being re-written from the ground up as of September 2022.

// We want to enforce good stuff by default.
#![deny(missing_copy_implementations, missing_debug_implementations)]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(missing_docs)]

/// The version of this copy of Wright.
pub const WRIGHT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod reporting;
pub mod source_tracking;
// pub mod parser;
// pub mod repl;
// pub mod solver;
