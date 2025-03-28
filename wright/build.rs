//! Build script for wright.
//! This is used for capturing build environment info which is used at runtime.

use rustc_version::{Channel, version_meta};

fn main() {
    // Set a cfg flag if we're on the nightly channel.

    println!("cargo::rustc-check-cfg=cfg(CHANNEL_NIGHTLY)");
    if version_meta().unwrap().channel == Channel::Nightly {
        println!("cargo:rustc-cfg=CHANNEL_NIGHTLY");
    }

    // Save build info.
    // See https://docs.rs/built/0.7.4/built/index.html.
    built::write_built_file().expect("Failed to acquire build-time information");
}
