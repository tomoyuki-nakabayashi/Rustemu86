use bit_field::BitField;

#[derive(Debug, Clone, Copy)]
pub struct ModRm {
    mode: usize,
    reg: usize,
    rm : usize,
}

impl ModRm {
    pub fn new(byte: u8) -> ModRm {
        ModRm {
            mode: byte.get_bits(6..8) as usize,
            reg: byte.get_bits(3..6) as usize,
            rm: byte.get_bits(0..3) as usize,
        }
    }

    pub fn get_reg_rm(&self) -> (usize, usize) {
        return (self.reg, self.rm)
    }
}