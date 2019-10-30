
use wright::{Emit, call_files};
use wright::cli::get_wright_app;
use std::process::exit;

fn main() {
    let matches = get_wright_app().get_matches();
    let filenames = matches.values_of("INPUT").unwrap().collect();
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
    exit(call_files(filenames, matches.is_present("RUN"), emits, matches.is_present("VERBOSE")))
}
