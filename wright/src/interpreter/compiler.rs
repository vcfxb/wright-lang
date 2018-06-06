//! Module for compiling files to a specified output.

use interpreter::{Emit, OptimizationLevel};
use target::Target;

use std::io::{Read, Write};
use std::fs::File;

/// Function for compiling a wright source file. Currently unimplemented.
pub fn compile<T: Write>(
    emits: Vec<Emit>,
    target: Target,
    optimization: OptimizationLevel,
    file_name: &str,
    input: String,
    output: T
) -> i32 {
    unimplemented!()
}
