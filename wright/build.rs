//! Build script for wright.
//! This is used for capturing build environment info which is used at runtime.

use rustc_version::{version_meta, Channel};

fn main() {
    // Set cfg flags depending on release channel.
    // See: https://stackoverflow.com/a/70914430.
    let channel = match version_meta().unwrap().channel {
        Channel::Stable => "CHANNEL_STABLE",
        Channel::Beta => "CHANNEL_BETA",
        Channel::Nightly => "CHANNEL_NIGHTLY",
        Channel::Dev => "CHANNEL_DEV",
    };

    println!("cargo:rustc-cfg={}", channel);

    // Save build info.
    // See https://docs.rs/built/0.7.4/built/index.html.
    built::write_built_file().expect("Failed to acquire build-time information");
}
