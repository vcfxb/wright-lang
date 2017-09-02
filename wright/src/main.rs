extern crate wright;

use std::{env, process};
use wright::argparse;
use wright::run;

fn main() {
    if let Some(i) = argparse::argparse(env::args()) {
        //println!("{:?}", i);
        if i.input == argparse::InputMode::Interactive {
            process::exit(run::interactive());
        } else if i.run {
            process::exit(run::interpret(i.mode, i.file.unwrap(), i.optimization));
        } else if let Some(out) = i.output {
            process::exit(run::compile(i.mode, i.file.unwrap(), i.optimization, out));
        }
    } else {
        process::exit(0);
    }
}
