//! Script that gets called by CI to install LLVM on windows.

use std::{fs, io, time::Duration};

use indicatif::{HumanBytes, ProgressBar};
use tar::Archive;
use url::Url;
use xz2::bufread::XzDecoder;

/// The URL of the LLVM 18.1.1 for Windows.
const WINDOWS_LLVM_URL: &str = "https://github.com/llvm/llvm-project/releases/download/llvmorg-18.1.1/clang+llvm-18.1.1-x86_64-pc-windows-msvc.tar.xz";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the dest dir.
    let dest_dir = std::env::args()
        .nth(1)
        .expect("Specify the installation destination");

    // Create a new spinner for getting the file.
    let download_spinner = ProgressBar::new_spinner().with_message("Downloading LLVM...");
    download_spinner.enable_steady_tick(Duration::from_millis(100));

    // Get the file from the url.
    let file_bytes = reqwest::get(WINDOWS_LLVM_URL).await?.bytes().await?;

    // The file downloaded, end the spinner.
    download_spinner.finish();
    println!("Got LLVM archive: {}", HumanBytes(file_bytes.len() as u64));

    // Make a decoder to strip the xz file extension.
    let xz_decoder: XzDecoder<&[u8]> = XzDecoder::new(file_bytes.as_ref());

    // Make a tar archive to write to disk.
    let mut tar_archive = Archive::new(xz_decoder);

    // Unpack it to disk.
    tar_archive
        .entries()?
        // Skip entries that error.
        .filter_map(Result::ok)
        .try_for_each(|mut entry| -> io::Result<()> {
            // Print that we're unpacking.
            println!(
                "Unpacking {}: {}",
                entry.path()?.display(),
                HumanBytes(entry.size())
            );
            // Unpack
            entry.unpack_in(".")?;

            Ok(())
        })?;

    let rename_folder = Url::parse(WINDOWS_LLVM_URL)?
        .path_segments()
        .expect("Path has segments")
        .last()
        .expect("Got last segment")
        .strip_suffix(".tar.xz")
        .expect("Stripped suffix")
        .to_owned();

    println!("Renamining {rename_folder} to {dest_dir}");

    fs::rename(rename_folder, &dest_dir)?;

    Ok(())
}
