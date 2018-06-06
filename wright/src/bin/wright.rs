extern crate wright;
extern crate regex;
extern crate clap;

use std::process::exit;
use clap::{Arg, App, AppSettings};
use wright::version::VERSION;
use wright::target::Target;
use wright::interpreter::{Interpreter, OptimizationLevel, Emit};
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
        .arg(Arg::with_name("TARGET")
            .short("t")
            .long("target")
            .help("The byte-code format to target")
            .long_help("Tell wright what byte-code format to target. When not specified, and -r or --run \
                is used, will use tree-walk style interpreter. The default when compiling is jvm.")
            .takes_value(true)
            .possible_values(&["jvm"])
            .multiple(false)
            .conflicts_with_all(&["INTERACTIVE"])
            .requires("INPUT")
        )
        .arg(Arg::with_name("OPTIMIZE")
            .short("O")
            .long("opt")
            .help("Set Optimization Level. Default is debug.")
            .takes_value(true)
            .possible_values(&["debug", "release"])
            .multiple(false)
            .conflicts_with_all(&["RUN", "INTERACTIVE"])
            .requires("INPUT")
        )
        .get_matches();
    if !matches.is_present("INTERACTIVE") {
        let file = matches.value_of("INPUT").unwrap();
        let out = matches.value_of("OUTPUT");
        let mut emits: Vec<Emit> = Vec::with_capacity(3);
        if matches.is_present("EMIT") {
            for v in matches.values_of("EMIT").unwrap() {
                emits.push(match v {
                    "tokens"  => Emit::Tokens,
                    "ast"     => Emit::AbstractSyntaxTree,
                    other => panic!("{} should not be a possible emit option.", other),
                });
            }
        }
        let target: Target;
        let run: bool = matches.is_present("RUN");
        if matches.is_present("TARGET") {
            target = match matches.value_of("TARGET").unwrap() {
                //"wasm" => Target::WASM,
                "jvm" => Target::JVM,
                //"bf" => Target::BrainFuck,
                other => panic!("{} is not a possible target!", other),
            }
        }  else {
            exit(match Interpreter::treewalker(file, emits) {Some(i) => i.run(), _ => 1});
        }
        let level = match matches.value_of("OPTIMIZE") {
            Some("release") => OptimizationLevel::Release,
            _ => OptimizationLevel::Debug
        };

        exit(match Interpreter::new(file, level, emits, Some(target), out, run) {
            Some(i) => i.run(),
            _ => 1
        });

    } else {
        let interp = Interpreter::Interactive;
        exit(interp.run());
    }
}

fn is_valid_wright_file_name(s: String) -> Result<(), String> {
    let re: Regex = Regex::new(r"[[:word:]]+.wr$|[[:word:]]+.wright$").unwrap(); 
    match re.is_match(s.as_str()) {
        true  => Ok(()),
        false => Err(format!("Wright file names can only contain alphanumerics and underscores and \
        must end with .wr or .wright. ({} does not.)", s)),
    }
}
