
/// A [brainfuck](https://esolangs.org/wiki/Brainfuck) inspired virtual machine
/// implementation for wright.
pub mod bf;

/// A [MIPS](https://en.wikipedia.org/wiki/MIPS_architecture) inspired virtual
/// machine implementation for wright.
pub mod mips;

/// This trait defines the types used in defining a virtual machine and is
/// implemented by virtual machines that wright targets.
pub trait VirtualMachine {
    /// The type associated with operation codes for this virtual machine. Also
    /// defines the maximum number of instructions that can be used with this
    /// virtual machine based on the size of this type.
    type OpCode;

    /// The type used to store instructions. Usually `u8` (bytes).
    type InstructionUnit;

    /// Check if this virtual machine has halted.
    fn halted(&self) -> bool;


}