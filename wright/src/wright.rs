extern crate wright;
use std::{env, process};
use wright::argparser;
use wright::run;

fn main() {
    if let Some(i) = argparser::argparse(env::args()) {
        process::exit(run::interpret(i));
    } else {
        process::exit(0);
    }
}
