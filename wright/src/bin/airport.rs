// Wright's version of cargo
extern crate wright;
use std::process::exit;

use wright::version::VERSION;

fn main() {
    println!("Wright's airport utility is not yet implemented as of version {}.", VERSION);
    exit(1);
}