//! The wright programming language crate. This is being re-written from the ground up as of September 2022.

// Compile without the standard library if the user chooses to do so.
#![cfg_attr(not(any(feature = "std", test)), no_std)]
// We want to enforce good stuff by default.
#![deny(missing_copy_implementations, missing_debug_implementations)]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(missing_docs)]
// Compiler directive to get docs.rs (which uses the nightly version of the rust compiler) to show
// info about feature required for various modules and functionality.
//
// See: https://stackoverflow.com/a/70914430.
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]

// We cannot use memory mapped files on architectures that do not support memmap2.
#[cfg(all(
    feature = "file_memmap",
    any(target_arch = "wasm32", target_arch = "wasm64")
))]
compile_error!("Memory mapped files not available on WASM targets");

// If the "none" feature is enabled, make sure the user has no other features enabled.
//
// Currently all of the features besides "none" depend on "std" so if both "none" and "std"
// are present, raise an error at compile time.
//
// Make sure to keep this updated as more features are added.
#[cfg(all(feature = "none", feature = "std"))]
compile_error!("feature \"none\" is enabled, which restricts the usage of any other features including \"std\".");

/// The version of this copy of Wright.
pub const WRIGHT_VERSION: &str = build_info::PKG_VERSION;

/// Build information about this copy of wright, provided using <https://crates.io/crates/built>.
pub mod build_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[cfg(feature = "source-tracking")]
pub mod source_tracking;

#[cfg(feature = "reporting")]
pub mod reporting;

#[cfg(feature = "lexer")]
pub mod lexer;

#[cfg(feature = "ast-models")]
pub mod ast;

#[cfg(feature = "parser")]
pub mod parser;

// pub mod repl;
