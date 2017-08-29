extern crate regex;

use std::env;
use std::path::Path;
use self::regex::Regex;
use version;

pub fn argparse(args : env::Args) -> Option<CourseOfAction> {
    let all_args : Vec<String> = args.collect();
    if (all_args.contains(&"-h".to_string())) || (all_args.contains(&"--help".to_string()))  {
        version();
        help();
        None
    } else if (all_args.contains(&"-v".to_string())) || (all_args.contains(&"--version".to_string())) {
        version();
        None
    } else if all_args.contains(&"-I".to_string()) || (all_args.contains(&"--interactive".to_string())) {
        Some(CourseOfAction { input: InputMode::Interactive, mode: ProcessingMode::TreeWalk, file: None, optimization: 0, run: true, output: None})
    } else {
        // variables to tell the argparser what to look for / expect in each arg.
        let mut o_now = false;
        let mut contains_file = false;
        let mut file_name = None;
        let mut use_llvm = false;
        let mut llvm_ir = false;
        let mut r = false;
        let mut o: Option<String> = None;
        let mut out_now = false;
        let mut opt = 0;
        let mut radon = false;
        let re = Regex::new(r"[[:alnum:]].wright").unwrap();
        let alt_re = Regex::new(r"[[:alnum:]].wr").unwrap();
        let o_re = Regex::new(r"[[:alnum:]].radon").unwrap();
        for a in all_args {
            if o_now {
                opt = if let Ok(s) = a.to_string().parse::<u8>() {
                    if s > 2 {
                        println!("Optimization defaulting to 2, as {} is not a number between 0 and 3", a);
                        2
                    }
                    else {
                        s
                    }
                } else {
                    println!("Optimization defaulting to 0, as {} is not a number between 0 and 3", a);
                    0
                };
                o_now = false;
            }
            else if out_now {
                if o_re.is_match(&a) {
                    o = Some(a.clone().to_string());
                    out_now = false;
                } else {
                    let mut t = a.clone().to_string();
                    t.push_str(".radon");
                    o = Some(t);
                    out_now = false;
                }
            }
            else if re.is_match(&a) || alt_re.is_match(&a) {
                contains_file = true;
                file_name = Some(a.clone());
            }
            else if a == "--llvm".to_string() {
                use_llvm = true;
            }
            else if a == "--emit-llvm-ir".to_string() {
                llvm_ir = true;
            }
            else if a == "run".to_string() {
                r = true;
                o = None;
            }
            else if a == "-O".to_string() || a == "--optimize".to_string() {
                o_now = true;
                continue;
            }
            else if a == "-o".to_string() || a == "--output".to_string() {
                out_now = true;
                if r {
                    println!("\nPlease note that running your file will not compile to an output file.\n");
                }
                continue;
            }
            else if a == "--radon".to_string() {
                radon = true;
            }
        }
        if contains_file {
            if o == None {
                let mut t = file_name.clone().unwrap();
                let t_temp = t.clone();
                let p = Path::new(t_temp.as_str());
                t = p.file_stem().unwrap().to_str().unwrap().to_string();       // weird string munge to change file extension
                t.push_str(".radon");
                o = Some(t);
            }
            if use_llvm {
                Some(CourseOfAction {input : InputMode::File, mode: ProcessingMode::LLVM, file: file_name, optimization: opt , run: false, output: o})
            }
            else if llvm_ir {
                Some(CourseOfAction {input : InputMode::File, mode: ProcessingMode::LLVM_IR, file: file_name, optimization: opt , run: false, output: o})
            }
            else if r {
                if radon {
                    Some(CourseOfAction {
                        input : InputMode::File,
                        mode: ProcessingMode::RadonBytecode,
                        file: file_name,
                        optimization: opt,
                        run: true,
                        output: None
                    })
                } else {
                    Some(CourseOfAction {
                        input : InputMode::File,
                        mode: ProcessingMode::TreeWalk,
                        file: file_name, optimization: 0,
                        run: true,
                        output: None
                    })
                }
            }
            else {
                Some(CourseOfAction {input : InputMode::File, mode: ProcessingMode::RadonBytecode, file: file_name, optimization: opt, run: false, output: o})
            }
        }
        else {
            println!("Please specify a file or interactive mode.");
            help();
            None
        }

    }
}

#[derive(Debug)] // for debugging (of course)
pub struct CourseOfAction {
    pub input: InputMode,
    pub mode: ProcessingMode,
    pub file: Option<String>,
    pub optimization: u8,
    pub run: bool,
    pub output: Option<String>,

}

#[derive(PartialEq, Debug)]
pub enum InputMode {
    File,
    Interactive,
}

#[derive(Debug)]
pub enum ProcessingMode {
    TreeWalk,
    LLVM,
    LLVM_IR,
    RadonBytecode,
}

fn version() {
    println!("Wright language version {}", version::get_version().as_str());
}

fn help() {
    println!("
wright [OPTIONS] [INPUT]

Input:
    Files are valid if they have a \".wr\" or \".wright\" extension.

Options:
    run                         Runs file immediately, using Radon byte-code.
    -h, --help                  Display this message.
    -v, --version               Display version information
    -I, --interactive           Run in interactive interpreted mode. Will automatically use a tree-walk interpreter,
                                    with optimization set to 0.
    -o, --output [FILE]         Specifies output filename.
    -O, --optimize [NUMBER]     Optimized compilation based on number 0, 1 or 2. Defaults to 0.
        --llvm                  Use LLVM.
        --emit-llvm-ir          Emits LLVM IR code. Implies --llvm.
        --radon                 Use Radon byte-code, emitting a .radon file. Radon is the default for compiling,
                                    but is not the default wen using 'run'. Specify if you would like to use it
                                    otherwise running the file just defaults to a tree-walk interpreter with
                                    optimization set to 0.
"
);
}