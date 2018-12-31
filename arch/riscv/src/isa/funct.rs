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
        ADD = 0b000
    }
}
