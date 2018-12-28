//! Opcode

/// Raw Opcode in an instruction[6:0].
enum_from_primitive! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Opcode {
        OpImm = 0b001_0011,
        OpWfi = 0b111_0011,
    }
}

/// Opcode for ALU
enum_from_primitive! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum ALU_OPCODE {
        ADD = 0b001_1000,
        OR  = 0b010_1110,
    }
}