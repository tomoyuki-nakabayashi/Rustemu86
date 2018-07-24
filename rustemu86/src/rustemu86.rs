use byteorder::{ReadBytesExt, LittleEndian};
use loader::BinaryReader;
use num::FromPrimitive;

enum GeneralRegisterId {
  RegRax,
  RegRcx,
  RegRdx,
  RegRbx,
}

impl FromPrimitive for GeneralRegisterId {
  fn from_i64(n: i64) -> Option<GeneralRegisterId> {
    match n {
      0 => Some(GeneralRegisterId::RegRax),
      1 => Some(GeneralRegisterId::RegRcx),
      2 => Some(GeneralRegisterId::RegRdx),
      3 => Some(GeneralRegisterId::RegRbx),
      _ => None,
    }
  }

  fn from_u64(n: u64) -> Option<GeneralRegisterId> {
    match n {
      0 => Some(GeneralRegisterId::RegRax),
      1 => Some(GeneralRegisterId::RegRcx),
      2 => Some(GeneralRegisterId::RegRdx),
      3 => Some(GeneralRegisterId::RegRbx),
      _ => None,
    }
  }

  fn from_u8(n: u8) -> Option<GeneralRegisterId> {
    match n {
      0 => Some(GeneralRegisterId::RegRax),
      1 => Some(GeneralRegisterId::RegRcx),
      2 => Some(GeneralRegisterId::RegRdx),
      3 => Some(GeneralRegisterId::RegRbx),
      _ => None,
    }
  }
}

#[derive(Debug)]
pub struct Rustemu86 {
  // Must have cpu, memory, peripherals
  rax: u64,
  rcx: u64,
  rdx: u64,
  rbx: u64,
}

impl Rustemu86 {
  fn mov_imm(&mut self, inst: &[u8]) {
    let dest = GeneralRegisterId::from_u8(inst[0] & 0b00000111).unwrap();
    let mut imm = &inst[2..];
    let imm: u64 = imm.read_u32::<LittleEndian>().unwrap().into();

    match dest {
      GeneralRegisterId::RegRax => self.rax = imm,
      GeneralRegisterId::RegRcx => self.rcx = imm,
      GeneralRegisterId::RegRdx => self.rdx = imm,
      GeneralRegisterId::RegRbx => self.rbx = imm,
      _ => ()
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn execute_mov_imm() {
    let mut emu = Rustemu86{
      rax: 0xFFFFFFFF,
      rcx: 0xFFFFFFFF,
      rdx: 0xFFFFFFFF,
      rbx: 0xFFFFFFFF,
    };

    let mut insts: Vec<&[u8]> = Vec::with_capacity(4);
    insts.push(&[0xb8, 0x00, 0x00, 0x00, 0x00, 0x00]);  // mov rax, 0
    insts.push(&[0xb9, 0x00, 0x00, 0x00, 0x00, 0x00]);  // mov rcx, 0
    insts.push(&[0xba, 0x00, 0x00, 0x00, 0x00, 0x00]);  // mov rdx, 0
    insts.push(&[0xbb, 0x00, 0x00, 0x00, 0x00, 0x00]);  // mov rbx, 0

    emu.mov_imm(&insts[0]);
    assert_eq!(emu.rax, 0);

    emu.mov_imm(&insts[1]);
    assert_eq!(emu.rcx, 0);

    emu.mov_imm(&insts[2]);
    assert_eq!(emu.rdx, 0);

    emu.mov_imm(&insts[3]);
    assert_eq!(emu.rbx, 0);
  }
}
