//! Build script for wright. Currently this is just used to get docs.rs to show feature/cfg info.
//!
//! See: https://stackoverflow.com/a/70914430.

use rustc_version::{version_meta, Channel};

fn main() {
    // Set cfg flags depending on release channel
    let channel = match version_meta().unwrap().channel {
        Channel::Stable => "CHANNEL_STABLE",
        Channel::Beta => "CHANNEL_BETA",
        Channel::Nightly => "CHANNEL_NIGHTLY",
        Channel::Dev => "CHANNEL_DEV",
    };

    println!("cargo:rustc-cfg={}", channel)
}
