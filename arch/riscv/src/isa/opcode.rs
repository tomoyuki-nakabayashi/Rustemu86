//! Opcode

/// Raw Opcode in an instruction[6:0].
enum_from_primitive! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Opcode {
        OpImm    = 0b001_0011,
        Op       = 0b011_0011,
        Jal      = 0b110_1111,
        OpSystem = 0b111_0011,
    }
}

/// Opcode for ALU
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AluOpcode {
    ADD,
    OR,
}

/// Branch type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BranchType {
    //NONE,
    JAL,
    //JALR,
    //CONDITIONAL,
}
