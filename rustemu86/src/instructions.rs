use byteorder::{LittleEndian, ReadBytesExt};
use register_file::Reg64Id;
use register_file::RegisterFile;

#[derive(Debug)]
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
struct ModRm {
  mode: ModRmModeField,
  reg: Reg64Id,
  rm: Reg64Id,
}

fn decode_mod_rm(modrm: u8) -> ModRm {
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

pub fn undefined(_rf: &mut RegisterFile, _inst: &[u8]) {}
