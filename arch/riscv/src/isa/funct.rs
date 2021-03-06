//! Filed definitions for funct3/funct7

/// funct3 for OP-IMM of RV32I
enum_from_primitive! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Rv32iOpImmFunct3 {
        ADDI  = 0b000,
        SLLI  = 0b001,
        SLTI  = 0b010,
        SLTIU = 0b011,
        XORI  = 0b100,
        SRxI  = 0b101,
        ORI   = 0b110,
        ANDI  = 0b111,
    }
}

/// funct3 for OP of RV32I
enum_from_primitive! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Rv32iOpFunct3 {
        ADD = 0b000,
        SLL = 0b001,
        SLT = 0b010,
        SLTU = 0b011,
        XOR = 0b100,
        SRx = 0b101,
        OR = 0b110,
        AND = 0b111,
    }
}

/// funct3 for BRANCH of RV32I
enum_from_primitive! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Rv32iBranchFunct3 {
        BEQ = 0b000,
        BNE = 0b001,
        BLT = 0b100,
        BGE = 0b101,
        BLTU = 0b110,
        BGEU = 0b111,
    }
}

/// funct3 for LOAD of RV32I
enum_from_primitive! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Rv32iLoadFunct3 {
        LB = 0b000,
        LH = 0b001,
        LW = 0b010,
        LBU = 0b100,
        LHU = 0b101,
    }
}

/// funct3 for STORE of RV32I
enum_from_primitive! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Rv32iStoreFunct3 {
        SB = 0b000,
        SH = 0b001,
        SW = 0b010,
    }
}

/// funct3 for SYSTEM of RV32I
enum_from_primitive! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Rv32iSystemFunct3 {
        PRIV = 0b000,
        CSRRW = 0b001,
        CSRRS = 0b010,
        CSRRC = 0b011,
        CSRRWI = 0b101,
    }
}
