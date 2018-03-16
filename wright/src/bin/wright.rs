extern crate wright;
extern crate regex;
extern crate clap;

use std::process;
use clap::{Arg, App, AppSettings};
use wright::version::VERSION;
use wright::interpreter::{Interpreter, OptimizationLevel};
use regex::Regex;

fn main() {
    let matches = App::new("Wright")
        .setting(AppSettings::ColorAlways)
        .version(VERSION)
        .author("Antonia Calia-Bogan (github.com/Alfriadox)")
        .about("The Wright programming language interpreter and compiler.")
        .arg(Arg::with_name("INTERACTIVE")
            .short("i")
            .long("interactive")
            .help("Run Wright in interactive mode.")
        )
        .arg(Arg::with_name("INPUT")
            .required_unless("INTERACTIVE")
            .help("Input wright file.")
            .validator(is_valid_wright_file_name)
        )
        .arg(Arg::with_name("RUN")
            .requires("INPUT")
            .help("Runs input wright file rather than compiling it.")
            .case_insensitive(true)
            .short("r")
            .long("run")
        )
        .arg(Arg::with_name("OUTPUT")
            .conflicts_with_all(&["RUN", "INTERACTIVE"])
            .help("Output file to be written. Will overwrite if it exists.")
            .short("o")
            .long("output")
            .takes_value(true)
        )
        .arg(Arg::with_name("OPTIMIZE")
            .short("O")
            .long("opt")
            .help("Set Optimization Level. Default is debug.")
            .takes_value(true)
            .possible_values(&["debug", "release", "supercompiler"])
            .use_delimiter(false)
            .conflicts_with_all(&["RUN", "INTERACTIVE"])
            .requires("INPUT")
        )
        .get_matches();
    if !matches.is_present("INTERACTIVE") {
        let file = matches.value_of("INPUT").unwrap();
        let mut interp = match Interpreter::new(file, OptimizationLevel::Debug, None) {
            Some(i) => i,
            None    => {process::exit(1)},
        };
        if matches.is_present("OUTPUT") {
            interp = Interpreter::new(file, OptimizationLevel::Debug, Some(matches.value_of("OUTPUT").unwrap())).unwrap();
        }
        if matches.is_present("OPTIMIZE") {
            interp.level = match matches.value_of("OPTIMIZE").unwrap() {
                "debug"         => OptimizationLevel::Debug,
                "release"       => OptimizationLevel::Release,
                "supercompiler" => OptimizationLevel::SuperCompiler,
                _               => panic!("Bad opt argument!"),
            }
        }
        process::exit(interp.run());
    } else {
        let mut interp = Interpreter::new_interactive();
        process::exit(interp.run());
    }
}

fn is_valid_wright_file_name(s: String) -> Result<(), String> {
    let re: Regex = Regex::new(r"[[:word:]]+.wr$|[[:word:]]+.wright$").unwrap(); 
    match re.is_match(s.as_str()) {
        true  => Ok(()),
        false => Err(format!("{} doesn't end in .wr or .wright as all wright files should.", s)),
    }
}