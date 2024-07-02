//! The wright programming language crate. This is being re-written from the ground up as of September 2022.

// We want to enforce good stuff by default.
#![deny(missing_copy_implementations, missing_debug_implementations)]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(missing_docs)]
// Compiler directive to get docs.rs (which uses the nightly version of the rust compiler) to show
// info about featurer required for various modules and functionality.
//
// See: https://stackoverflow.com/a/70914430.
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]

// We cannot use memory mapped files on architectures that do not support memmap2.
#[cfg(all(
    feature = "file_memmap",
    any(target_arch = "wasm32", target_arch = "wasm64")
))]
compile_error!("Memory mapped files not available on WASM targets");

/// The version of this copy of Wright.
pub const WRIGHT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(feature = "reporting")]
pub mod reporting;

#[cfg(feature = "source_tracking")]
pub mod source_tracking;
// pub mod parser;
// pub mod repl;
// pub mod solver;
