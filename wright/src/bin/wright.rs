extern crate wright;
extern crate regex;
extern crate clap;

use std::process::exit;
use clap::{Arg, App, AppSettings};
use wright::version::VERSION;
use regex::Regex;

fn main() {
    let matches = App::new("Wright")
        .setting(AppSettings::ColorAlways)
        .version(VERSION)
        .author("Antonia Calia-Bogan (github.com/Alfriadox)")
        .about("The Wright programming language interpreter and compiler.")
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
        .arg(Arg::with_name("EMIT")
            .short("e")
            .long("emit")
            .help("Prints intermediate representation(s).")
            .takes_value(true)
            .possible_values(&["tokens", "ast"])
            .use_delimiter(true)
            .multiple(true)
            .conflicts_with("INTERACTIVE")
            .requires("INPUT")
        )
        .get_matches();
//    let file = matches.value_of("INPUT").unwrap();
//    let mut emits: Vec<Emit> = Vec::with_capacity(3);
//    if matches.is_present("EMIT") {
//        for v in matches.values_of("EMIT").unwrap() {
//            emits.push(match v {
//                "tokens"  => Emit::Tokens,
//                "ast"     => Emit::AbstractSyntaxTree,
//                other => panic!("{} should not be a possible emit option.", other),
//            });
//        }
//    }
//    exit(match Interpreter::treewalker(file, emits) {Some(i) => i.run(), _ => 1});
}

fn is_valid_wright_file_name(s: String) -> Result<(), String> {
    let re: Regex = Regex::new(r"[[:word:]]+.wr$|[[:word:]]+.wright$").unwrap(); 
    match re.is_match(s.as_str()) {
        true  => Ok(()),
        false => Err(format!("Wright file names can only contain alphanumerics and underscores and \
        must end with .wr or .wright. ({} does not.)", s)),
    }
}
