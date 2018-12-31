//! Opcode

/// Raw Opcode in an instruction[6:0].
enum_from_primitive! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Opcode {
        Load     = 0b000_0011,
        MiscMem  = 0b000_1111,
        OpImm    = 0b001_0011,
        Store    = 0b010_0011,
        Op       = 0b011_0011,
        Branch   = 0b110_0011,
        Jal      = 0b110_1111,
        OpSystem = 0b111_0011,
    }
}

/// Opcode for ALU
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AluOp {
    ADD,
    OR,
    SLT,
    SLTU,
    AND,
    XOR,
    SLL,
    SRL,
    SRA,
}

/// Branch type
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum BranchType {
    //NONE,
    JAL,
    //JALR,
    COND_EQ,
}

/// Load/Store type
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum LoadStoreType {
    LW,
    SW,
}
