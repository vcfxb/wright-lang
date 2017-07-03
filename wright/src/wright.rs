use std::env;
use std::io::{self, Read, Write};
use std::ffi::CStr;
use std::os::raw::c_char;


#[no_mangle]        // Tell the compiler not to change the function's name
pub extern fn run_file(optimize_level : u8, contents : * const c_char, file_n : * const c_char) {
    let file_content;
    let file_name;
    // use unsafe here because this is passed in from python, and doing it without unsafe is hard
    unsafe {
        file_content = CStr::from_ptr(contents).to_str().unwrap();
        file_name = CStr::from_ptr(file_n).to_str().unwrap();
    }
    //println!("Optimize level {}: {}: \n{}", optimize_level, file_name, file_content);

    // todo: use file input etc.
}

#[no_mangle]        // Tell the compiler not to change the function's name
pub extern fn start_prompt() {
    let mut current_line :u64 = 1;
    loop {
        print!("{}<<< ", current_line);
        match io::stdout().flush(){
            Ok(a) => a,
            Err(e) => {
                println!("\nCould not write to standard output! (Fatal)\n");
                panic!(e);
            },
        };
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(a) => a,
            Err(e) => {
                println!("Input contained invalid characters!");
                panic!(e);
            },
        };
        print!("{}>>> {}", current_line, buffer.as_str());
        match io::stdout().flush() {
            Ok(a) => a,
            Err(e) => {
                println!("\nCould not write to standard output! (Fatal)\n");
                panic!(e);
            },
        };
        current_line += 1;
    }
}