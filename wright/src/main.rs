extern crate wright;

use std::env;
use wright::argparse;

fn main() {
    // todo
    if let Some(i) = argparse::argparse(env::args()) {
        if i.input == argparse::InputMode::Interactive {

        }
    }
}