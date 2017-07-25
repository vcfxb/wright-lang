extern crate wright;

use std::{env, process};
use wright::argparse;

fn main() {
    // todo
    if let Some(i) = argparse::argparse(env::args()) {
        println!("{:?}", i);
    } else {
        process::exit(0);
    }
}