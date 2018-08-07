use cpu::register_file::Reg64Id;
use cpu::register_file::RegisterFile;
use cpu::fetcher::FetchedInst;
use num::FromPrimitive;
use bit_field::BitField;

#[derive(Debug, PartialEq)]
pub enum DestType {
  Register,
  Rip,
}

#[derive(Debug)]
pub struct DecodedInst {
  pub dest_type: DestType,
  pub dest_rf: Reg64Id,
  pub result: u64,
}

impl DecodedInst {
  pub fn new(dest_type: DestType, rf: Reg64Id, result: u64) -> DecodedInst {
    DecodedInst {
      dest_type: dest_type,
      dest_rf: rf,
      result: result,
    }
  }
}

#[derive(Debug)]
enum ModRmModeField {
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

#[derive(Debug)]
pub struct ModRm {
  mode: ModRmModeField,
  reg: Reg64Id,
  rm: Reg64Id,
}

impl ModRm {
  pub fn new(modrm: u8) -> ModRm {
    let mode = (modrm & 0b11000000) >> 6;
    let reg = (modrm & 0b00111000) >> 3;
    let rm = modrm & 0b00000111;

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

pub fn decode_mod_rm(modrm: u8) -> ModRm {
  let mode = (modrm & 0b11000000) >> 6;
  let reg = (modrm & 0b00111000) >> 3;
  let rm = modrm & 0b00000111;

  ModRm {
    mode: ModRmModeField::from_u8(mode).unwrap(),
    reg: Reg64Id::from_u8(reg).unwrap(),
    rm: Reg64Id::from_u8(rm).unwrap(),
  }
}

pub fn decode_mov_new(inst: &FetchedInst) -> DecodedInst {
  let dest = Reg64Id::from_u8(inst.opcode.get_bits(0..3)).unwrap();
  DecodedInst::new(DestType::Register, dest, inst.immediate)
}

pub fn decode_inc_new(rf: &RegisterFile, inst: &FetchedInst) -> DecodedInst {
  let dest = inst.mod_rm.rm;
  let incremented_value = rf.read64(dest) + 1;
  DecodedInst::new(DestType::Register, dest, incremented_value)
}

pub fn decode_add_new(rf: &RegisterFile, inst: &FetchedInst) -> DecodedInst {
  let dest = inst.mod_rm.rm;
  let src = inst.mod_rm.reg;
  let result_value = rf.read64(dest) + rf.read64(src);
  DecodedInst::new(DestType::Register, dest, result_value)
}

pub fn decode_jmp_new(rip: u64, inst: &FetchedInst) -> DecodedInst {
  let disp = inst.displacement;
  let rip = rip + disp as u64;

  DecodedInst::new(DestType::Rip, Reg64Id::Unknown, rip)
}

pub fn undefined(_rf: &RegisterFile, _inst: &[u8]) {}
