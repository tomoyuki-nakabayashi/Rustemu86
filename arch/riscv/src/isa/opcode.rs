//! Opcode

enum_from_primitive! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Opcode {
        OpImm = 0b001_0011,
        OpWfi = 0b111_0011,
    }
}
