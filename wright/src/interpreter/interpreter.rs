/// Module for interpreting files via bytecode.

use interpreter::{Emit,OptimizationLevel};
use target::Target;

/// Function to interpret a file using a bytecode interpreter. Currently unimplemented.
pub fn interpret(
    emits: Vec<Emit>,
    target: Target,
    opt: OptimizationLevel,
    file_name: &str,
    input: String
) -> i32 {
    unimplemented!()
}
