use std::env;

pub fn argparse(args : env::Args) -> Option<CourseOfAction> {
    let mut all_args : Vec<String> = args.collect();
    if (all_args.contains(&"-h".to_string())) || (all_args.contains(&"--help".to_string()))  {
        help();
        return None;
    } else if (all_args.contains(&"-v".to_string())) || (all_args.contains(&"--version".to_string()))  {
        version();
        return None;
    } else {
        return None; // todo: secondary arg parsing
    }
}

pub struct CourseOfAction {
    pub input: InputMode,
    pub mode: ProcessingMode,
    pub file: Option<String>,
    pub optimization: u8,

}

#[derive(PartialEq)]
pub enum InputMode {
    File,
    Interactive,
}

pub enum ProcessingMode {
    TreeWalk,
    LLVM,
    RadonBytecode,
    InterpretedBytecode,
}

fn version() {
    println!("Wright language version 0.1");
}

fn help() {
    println!(
"
wright [OPTIONS] [INPUT]

Options:
    -h, --help          Display this message.
    -v, --version       Display version information
    -I, --interactive   Run in interactive interpreted mode.
"
);
}