extern crate wright;

use std::{env, process};
use wright::argparser;
use wright::run;


fn main() {
    if let Some(i) = argparser::argparse(env::args()) {
        if i.input == argparser::InputMode::Interactive {
            process::exit(run::interactive());
        } else if i.input == argparser::InputMode::File {
            process::exit(run::interpret(i.file.unwrap()));
        }
    } else {
        process::exit(0);
    }
}
