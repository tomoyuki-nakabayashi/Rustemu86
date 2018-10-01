use bit_field::BitField;

#[derive(Debug, Clone, Copy)]
pub struct ModRm {
    mode: u8,
    reg: u8,
    rm : u8,
}

impl ModRm {
    pub fn new(byte: u8) -> ModRm {
        ModRm {
            mode: byte.get_bits(6..8),
            reg: byte.get_bits(3..6),
            rm: byte.get_bits(0..3),
        }
    }
}