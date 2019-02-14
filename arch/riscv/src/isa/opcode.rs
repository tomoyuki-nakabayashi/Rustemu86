//! Opcode

/// Raw Opcode in an instruction[6:0].
enum_from_primitive! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Opcode {
        Load     = 0b000_0011,
        MiscMem  = 0b000_1111,
        OpImm    = 0b001_0011,
        Auipc    = 0b001_0111,
        Store    = 0b010_0011,
        Op       = 0b011_0011,
        Lui      = 0b011_0111,
        Branch   = 0b110_0011,
        Jalr     = 0b110_0111,
        Jal      = 0b110_1111,
        OpSystem = 0b111_0011,
    }
}

/// Opcode for ALU
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AluOp {
    ADD,
    SUB,
    OR,
    SLT,
    SLTU,
    AND,
    XOR,
    SLL,
    SRL,
    SRA,
    LUI,
    AUIPC,
}

/// Branch type
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum BranchType {
    //NONE,
    JAL,
    JALR,
    COND_EQ,
    COND_NE,
    COND_LT,
    COND_LTU,
    COND_GE,
    COND_GEU,
}

/// Load/Store type
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum LoadStoreType {
    LW,
    LH,
    LHU,
    LB,
    LBU,
    SW,
    SH,
    SB,
}

/// Csr OP
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum CsrOp {
    WRITE,
    SET,
    CLEAR,
}

/// Priviledged OP
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum PrivOp {
    WFI,
    MRET,
    ECALL,
}
