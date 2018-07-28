use byteorder::{ReadBytesExt, LittleEndian};
use register_file::RegisterFile;
use register_file::Reg64Id;

#[derive(Debug)]
enum ModRmModeField {
  Indirect,
  OneByteDisp,
  FourByteDisp,
  Direct,
}

use self::ModRmModeField::{Indirect, OneByteDisp, FourByteDisp, Direct};
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
  let rm = (modrm & 0b00000111);

  ModRm {
    mode: ModRmModeField::from_u8(mode).unwrap(),
    reg: Reg64Id::from_u8(reg).unwrap(),
    rm: Reg64Id::from_u8(rm).unwrap(),
  }
}

pub fn mov_imm64(rf: &mut RegisterFile, inst: &[u8]) {
  const MOV_OP: u8 = 0xb8;
  let dest = Reg64Id::from_u8(inst[0] - MOV_OP).unwrap();
  let mut imm = &inst[2..];
  let imm: u64 = imm.read_u32::<LittleEndian>().unwrap().into();

  rf.write64(dest, imm);
}

pub fn inc(rf: &mut RegisterFile, inst: &[u8]) {
  let mod_rm = decode_mod_rm(inst[2]);
  let dest = mod_rm.rm;
  let incremented_value = rf.read64(dest)+1;
  rf.write64(dest, incremented_value);
}