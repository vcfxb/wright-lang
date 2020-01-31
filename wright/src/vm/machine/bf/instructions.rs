use crate::vm::instruction::Operation;
use crate::vm::machine::bf::BrainFuckVM;
use crate::vm::machine::VirtualMachine;

/// Move the index left.
#[derive(Copy, Clone, Debug)]
pub struct Left;

impl Operation<BrainFuckVM> for Left {
    const NAME: &'static str = "Left";
    const MNEMONIC: &'static str = ">";
}

/// Move the index right.
#[derive(Copy, Clone, Debug)]
pub struct Right;

/// Increment the cell at index.
#[derive(Copy, Clone, Debug)]
pub struct Incr;

/// Decrement the cell at index.
#[derive(Copy, Clone, Debug)]
pub struct Decr;

/// Get an ASCII character from the standard input and set the current cell to
/// that value.
#[derive(Copy, Clone, Debug)]
pub struct Get;

/// Print the value in the current cell to the standard output as an ASCII
/// value.
#[derive(Copy, Clone, Debug)]
pub struct Put;

/// Execute the code stored in this block if and until the value of the current
/// cell is 0.
#[derive(Clone, Debug)]
pub struct BasicBlock {
    inner: Vec<<BrainFuckVM as VirtualMachine>::InstructionUnit>,
}
