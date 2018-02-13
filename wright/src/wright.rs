extern crate wright;
extern crate clap;
use clap::App;
use wright::run;
use wright::version;

fn main() {

}


/// Prints version string for wright.
/// Should be identical to cargo version information.
pub fn version() { println!("Wright language version {}", version::get_version().as_str()); }

/// Prints help information for wright
pub fn help() {
    println!("
    wright [OPTIONS] <INPUT>

    Input:
        Files are valid if they have a \".wr\" or \".wright\" extension.

    Options:
        -h, --help                  Display this message.
        -v, --version               Display version information
    "
    );
}