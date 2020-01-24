/// Wright virtual machine opcodes. Modeled off of MIPS instruction
/// reference.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[allow(missing_docs)]
#[allow(bad_style)]
pub enum OpCode {
    // 0x0x -- general operations.
    /// No operation.
    NOP = 0x00,
    SYSCALL,

    // 0x1x -- Register operations.
    /// Store word.
    SW = 0x10,
    /// Load word.
    LW,
    /// Store byte.
    SB,
    /// Load byte.
    LB,
    /// Move.
    MOV,
    /// Move from $lo register.
    MFLO,
    /// Move from $hi register.
    MFHI,

    // 0x2x -- Unsigned arithmetic operations.
    /// Add unsigned.
    ADDU = 0x20,
    /// Add unsigned with immediate.
    ADDUI,
    SUBU,
    SUBUI,
    MULU,
    MULUI,
    DIVU,
    DIVUI,

    // 0x3x -- Bitwise operations.
    SLL = 0x30,
    SLLV,
    SRL,
    SRLV,
    AND,
    ANDI,
    OR,
    ORI,
    XOR,
    XORI,

    // 0x4x -- Signed arithmetic operations.
    ADDS = 0x40,
    ADDSI,
    SUBS,
    SUBSI,
    MULS,
    MULSI,
    DIVS,
    DIVSI,
    MODS,
    MODSI,
    SRA,
    SRAV,

    // 0x5x -- Conditional Branching operations.
    BEQ = 0x50,
    BNE,
    BEQZ,
    BNEZ,
    BGTZ,
    BGEZ,
    BLTZ,
    BLEZ,

    // 0x6x -- Jump operations.
    JMP = 0x60,
    JAL,
    JR,
    JALR,

    // 0x7x -- Set operations.
    SLT = 0x70,
    SLTI,
    SLTU,
    SLTUI,
}
