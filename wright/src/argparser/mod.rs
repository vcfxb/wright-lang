//! The command line argument parser for Wright
extern crate regex;
use self::regex::Regex;
use version;

/// Parse arguments passed to the wright command.
/// Returns `None` if there is no action to take after parsing arguments.
/// Otherwise, it will return `Some(file)` where `file` is a String filename to be interpreted.
pub fn argparse(all_args: Vec<String>) -> Option<String> {
    if all_args.len() == 1 {
        println!("Wright requires a file to interpret.");
        help();
        None
    } else if (all_args.contains(&"-h".to_string())) || (all_args.contains(&"--help".to_string()))  {
        version();
        help();
        None
    } else if (all_args.contains(&"-v".to_string())) || (all_args.contains(&"--version".to_string())) {
        version();
        None
    } else if all_args.len() == 2 {
        let re: Regex = Regex::new(r"[[:alpha:]].wr|[[:alpha:]].wright").unwrap();
        if !re.is_match(all_args[1].clone().as_str()) {
            println!("File must end with .wr or .wright. Consider renaming {f} to {f}.wr",
                f = all_args[0].clone()
            );
            return None;
        }
        Some(all_args[1].clone())
    } else {
        println!("Unrecognized arguments. \n\n");
        version();
        help();
        None
    }
}

/// Prints version string for wright.
/// Should be identical to cargo version information.
pub fn version() { println!("Wright language version {}", version::get_version().as_str()); }

/// Prints help information for wright
pub fn help() {
    println!("
    wright [OPTIONS] <INPUT>

    Input:
        Files are valid if they have a \".wr\" or \".wright\" extension.

    Options:
        -h, --help                  Display this message.
        -v, --version               Display version information
    "
    );
}