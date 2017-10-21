//! The command line argument parser for Wright

extern crate regex;
use std::env;
use std::path::Path;
use self::regex::Regex;
use version;


// doc comments -> documentation
/**
    Parse arguments passed to the wright command.
    Files must end with a `.wright` or `.wr` extension.
    Returns `None` if there is no action to take after parsing arguments.
    Otherwise, it will return `Some(n)` where `n` is a `CourseOfAction` to be taken.
    Defaults to interactive "REPL" input when there is not a file argument.
    Otherwise wright will interpret the file.
*/
pub fn argparse(args: env::Args) -> Option<CourseOfAction> {
    let all_args : Vec<String> = args.collect();
    if all_args.len() == 1 {
        Some(CourseOfAction {input: InputMode::Interactive, file: None })
    } else if (all_args.contains(&"-h".to_string())) || (all_args.contains(&"--help".to_string()))  {
        version();
        help();
        None
    } else if (all_args.contains(&"-v".to_string())) || (all_args.contains(&"--version".to_string())) {
        version();
        None
    } else if all_args.len() == 2 {
        Some(CourseOfAction {input: InputMode::File, file: Some(all_args[2].clone()) })
    } else {
        println!("Unrecognized arguments. \n\n");
        version();
        help();
        None
    }
}

/// Determines the course of action for the wright interpreter to take,
/// whether that's reading a file and evaluating its contents or
/// reading from standard input.
#[derive(Debug)] // for debugging (of course)
pub struct CourseOfAction {
    pub input: InputMode,
    pub file: Option<String>,
}

/// Two possible input modes: `File` for interpreting and running wright files
/// and `Interactive` for running wright in interactive mode
#[derive(PartialEq, Debug)]
pub enum InputMode {
    File,
    Interactive,
}

/// Prints version string for wright.
/// Should be identical to cargo version information.
pub fn version() {
    println!("Wright language version {}", version::get_version().as_str());
}

/// Prints help information for wright
pub fn help() {
    println!("
    wright [OPTIONS] [INPUT]

    Input:
        Files are valid if they have a \".wr\" or \".wright\" extension.

    Options:
        run                         Runs file immediately, using Radon byte-code.
        -h, --help                  Display this message.
        -v, --version               Display version information

            If no input file is given, wright will default into an interactive mode.
        Otherwise, the input file will be run by the wright interpreter.
    "
    );
}