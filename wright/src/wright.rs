extern crate wright;
extern crate regex;
extern crate clap;
use clap::{Arg, App, AppSettings, SubCommand};
use wright::version::VERSION;
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
        .subcommand(SubCommand::with_name("run")
            .about("Compiles and runs given Wright file.")
            .arg(Arg::with_name("INPUT")
                .required(true)
                .help("Input wright file.")
            )
        )
        .subcommand(SubCommand::with_name("build")
            .about("Compiles the given Wright file.")
            .arg(Arg::with_name("INPUT")
                .required(true)
                .help("Input wright file.")
            )
            .arg(Arg::with_name("")
            )
        )
        .get_matches();
    if let Some(sub_run_matches) = matches.subcommand_matches("run") {
        if let Some(file) = sub_run_matches.value_of("INPUT") {
            unimplemented!()
        }
    }
}
