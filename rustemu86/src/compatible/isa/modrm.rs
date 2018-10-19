use bit_field::BitField;
use compatible::gpr::Reg32;
use num::FromPrimitive;

/// Mod R/M.
#[derive(Debug, Clone, Copy)]
pub struct ModRm {
    mode: usize,
    reg: Reg32,
    rm : Reg32,
}

impl ModRm {
    pub fn new(byte: u8) -> ModRm {
        ModRm {
            mode: byte.get_bits(6..8) as usize,
            reg: Reg32::from_u8(byte.get_bits(3..6)).expect("Never fail"),
            rm: Reg32::from_u8(byte.get_bits(0..3)).expect("Never fail"),
        }
    }

    pub fn get_reg_rm(&self) -> (Reg32, Reg32) {
        return (self.reg, self.rm)
    }
}