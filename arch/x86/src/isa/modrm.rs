use crate::gpr::{Reg32, SegReg};
use bit_field::BitField;
use num::FromPrimitive;

/// Mod R/M.
#[derive(Debug, Clone, Copy)]
pub struct ModRm {
    mode: usize, // TODO: Define enum.
    reg: u8,
    rm: u8,
}

impl ModRm {
    /// New from a byte.
    pub fn new(byte: u8) -> ModRm {
        ModRm {
            mode: byte.get_bits(6..8) as usize,
            reg: byte.get_bits(3..6),
            rm: byte.get_bits(0..3),
        }
    }

    /// ModRM mode.
    pub fn get_mode(&self) -> usize {
        self.mode
    }

    pub fn get_reg_rm(&self) -> (Reg32, Reg32) {
        let reg = Reg32::from_u8(self.reg).expect("Never fail.");
        let rm = Reg32::from_u8(self.rm).expect("Never fail.");
        (reg, rm)
    }

    pub fn get_sreg_rm(&self) -> (SegReg, Reg32) {
        let sreg = SegReg::from_u8(self.reg).expect("Cannot convert to Segment register index.");
        let rm = Reg32::from_u8(self.rm).expect("Never fail.");
        (sreg, rm)
    }
}
