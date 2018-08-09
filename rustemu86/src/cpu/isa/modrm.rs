use num::FromPrimitive;
use bit_field::BitField;
use cpu::isa::registers::Reg64Id;

#[derive(Debug, Clone, Copy)]
pub enum ModRmModeField {
  Indirect,
  OneByteDisp,
  FourByteDisp,
  Direct,
}

use self::ModRmModeField::{Direct, FourByteDisp, Indirect, OneByteDisp};
impl ModRmModeField {
  fn from_u8(n: u8) -> Option<ModRmModeField> {
    match n {
      0 => Some(Indirect),
      1 => Some(OneByteDisp),
      2 => Some(FourByteDisp),
      3 => Some(Direct),
      _ => None,
    }
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

  pub fn new_invalid() -> ModRm {
    ModRm {
      mode: Direct,
      reg: Reg64Id::Unknown,
      rm: Reg64Id::Unknown,
    }
  }
}