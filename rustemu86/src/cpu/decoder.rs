use byteorder::{LittleEndian, ReadBytesExt};
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

pub fn decode_mov_imm64(inst: &[u8]) -> DecodedInst {
  const MOV_OP: u8 = 0xb8;
  let dest = Reg64Id::from_u8(inst[0] - MOV_OP).unwrap();
  let mut imm = &inst[1..];
  let imm: u64 = imm.read_u32::<LittleEndian>().unwrap().into();

  DecodedInst::new(DestType::Register, dest, imm)
}

pub fn decode_mov_new(inst: &FetchedInst) -> DecodedInst {
  let dest = Reg64Id::from_u32(inst.opcode.get_bits(0..3)).unwrap();
  DecodedInst::new(DestType::Register, dest, inst.immediate)
}

pub fn decode_inc_new(rf: &RegisterFile, inst: &FetchedInst) -> DecodedInst {
  let dest = inst.mod_rm.rm;
  let incremented_value = rf.read64(dest) + 1;
  DecodedInst::new(DestType::Register, dest, incremented_value)
}

pub fn decode_inc(rf: &RegisterFile, inst: &[u8]) -> DecodedInst {
  let mod_rm = decode_mod_rm(inst[2]);
  let dest = mod_rm.rm;
  let incremented_value = rf.read64(dest) + 1;

  DecodedInst::new(DestType::Register, dest, incremented_value)
}

pub fn decode_add(rf: &RegisterFile, inst: &[u8]) -> DecodedInst {
  let mod_rm = decode_mod_rm(inst[2]);
  let dest = mod_rm.rm;
  let src = mod_rm.reg;
  let result_value = rf.read64(dest) + rf.read64(src);

  DecodedInst::new(DestType::Register, dest, result_value)
}

pub fn decode_jmp(rip: u64, inst: &[u8]) -> DecodedInst {
  let disp = inst[1];
  let rip = rip + disp as u64;

  DecodedInst::new(DestType::Rip, Reg64Id::Unknown, rip)
}

pub fn undefined(_rf: &RegisterFile, _inst: &[u8]) {}

#[cfg(test)]
mod test {
  use super::*;
  use cpu::fetcher::FetchedInst;

  #[test]
  fn decode_mov_with_new_struct() {
    let inst = FetchedInst {
      lecacy_prefix: 0,
      opcode: 0xb8,
      mod_rm: decode_mod_rm(0),
      sib: 0,
      displacement: 0,
      immediate: 0,
    };

    let decoded = decode_mov_new(&inst);
    let correct = decode_mov_imm64(&[0xb8, 0x00, 0x00, 0x00, 0x00]);

    assert_eq!(decoded.dest_type, correct.dest_type);
    assert_eq!(decoded.dest_rf, correct.dest_rf);
    assert_eq!(decoded.result, correct.result);
  }
}