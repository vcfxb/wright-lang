use crate::vm::machine::VirtualMachine;
use nom::lib::std::fmt::Debug;

/// An operation on a virtual machine.
pub trait Operation<M: VirtualMachine>: Debug + Clone {
    /// The name of this operation.
    const NAME: &'static str;

    /// The mnemonic of this operation.
    const MNEMONIC: &'static str;
}

/// An alias or macro for another operation on the same machine.
pub trait Alias<M: VirtualMachine>: Operation<M> {
    /// The type of the actual instruction represented by this alias.
    type Actual: Instruction<M>;

    /// Resolve this macro into its represented operation.
    fn resolve(self) -> Self::Actual;
}

/// Trait implemented by instructions in virtual machines.
pub trait Instruction<M: VirtualMachine>: Operation<M> {
    /// The OpCode on the associated virtual machine.
    const CODE: M::OpCode;

    /// Converts an instance of the implemented instruction type to the units
    /// associated with the associated virtual machine.
    fn render(self) -> Vec<M::InstructionUnit>;

    /// The function to execute this instruction on the given machine. This is
    /// called after the op code has been read, but before any arguments. This
    /// function allows the instruction to choose how many arguments to read,
    /// and how.
    fn execute(machine: &mut M);
}