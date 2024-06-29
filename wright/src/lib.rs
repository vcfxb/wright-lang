//! The wright programming language crate. This is being re-written from the ground up as of September 2022.

// We want to enforce good stuff by default.
#![deny(missing_copy_implementations, missing_debug_implementations)]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(missing_docs)]

// We cannot use memory mapped files on architectures that do not support memmap2. 
#[cfg(all(feature = "file_memmap", any(target_arch = "wasm32", target_arch = "wasm64")))]
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
