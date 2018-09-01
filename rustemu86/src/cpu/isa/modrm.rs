use bit_field::BitField;
use cpu::isa::registers::Reg64Id;
use num::FromPrimitive;

enum_from_primitive! {
#[derive(Debug, Clone, Copy, PartialEq)]
  pub enum ModRmModeField {
    Indirect = 0b00,
    OneByteDisp = 0b01,
    FourByteDisp = 0b10,
    Direct = 0b11,
    Unused = 0xff,
  }
}

#[derive(Debug, Clone, Copy)]
pub struct ModRm {
    pub mode: ModRmModeField,
    pub reg: Reg64Id,
    pub rm: Reg64Id,
}

impl ModRm {
    pub fn new(modrm: u8) -> ModRm {
        let mode = modrm.get_bits(6..8);
        let reg = modrm.get_bits(3..6);
        let rm = modrm.get_bits(0..3);

        ModRm {
            mode: ModRmModeField::from_u8(mode).unwrap(),
            reg: Reg64Id::from_u8(reg).unwrap(),
            rm: Reg64Id::from_u8(rm).unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sib {
    pub scale: u8,
    pub index: Reg64Id,
    pub base: Reg64Id,
}

impl Sib {
    pub fn new(modrm: u8) -> Sib {
        let scale = modrm.get_bits(6..8);
        let index = modrm.get_bits(3..6);
        let base = modrm.get_bits(0..3);

        Sib {
            scale: 2 ^ scale,
            index: Reg64Id::from_u8(index).unwrap(),
            base: Reg64Id::from_u8(base).unwrap(),
        }
    }
}
