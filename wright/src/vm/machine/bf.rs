use crate::vm::machine::VirtualMachine;

/// Instructions for the brainfuck virtual machine.
pub mod instructions;

/// A Brainfuck inspired virtual machine.
#[derive(Clone, Debug)]
pub struct BrainFuckVM {
    instr: Vec<<BrainFuckVM as VirtualMachine>::InstructionUnit>,
    read_head: usize,
    tape: Vec<u8>,
    index: usize,
}

impl VirtualMachine for BrainFuckVM {
    type OpCode = char;
    type InstructionUnit = char;

    fn halted(&self) -> bool {
        self.read_head == self.instr.len()
    }

    fn instructions(&self) -> Vec<char> {
        self.instr.clone()
    }
}