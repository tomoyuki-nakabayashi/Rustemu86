use byteorder::{ReadBytesExt, LittleEndian};
use loader::BinaryReader;
use register_file::RegisterFile;
use register_file::GeneralRegisterId;
use register_file::GeneralRegisterId::{RegRax, RegRcx, RegRdx, RegRbx};

#[derive(Debug)]
pub struct Rustemu86 {
  // Must have cpu, memory, peripherals
  pub rf: RegisterFile,
}

impl Rustemu86 {
  fn mov_imm(&mut self, inst: &[u8]) {
    let dest = GeneralRegisterId::from_u8(inst[0] & 0b00000111).unwrap();
    let mut imm = &inst[2..];
    let imm: u64 = imm.read_u32::<LittleEndian>().unwrap().into();

    self.rf.write64(dest, imm);
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn execute_mov_imm() {
    let mut emu = Rustemu86{
      rf: RegisterFile::new(),
    };

    let mut insts: Vec<&[u8]> = Vec::with_capacity(4);
    insts.push(&[0xb8, 0x00, 0x00, 0x00, 0x00, 0x00]);  // mov rax, 0
    insts.push(&[0xb9, 0x00, 0x00, 0x00, 0x00, 0x00]);  // mov rcx, 0
    insts.push(&[0xba, 0x00, 0x00, 0x00, 0x00, 0x00]);  // mov rdx, 0
    insts.push(&[0xbb, 0x00, 0x00, 0x00, 0x00, 0x00]);  // mov rbx, 0

    emu.mov_imm(&insts[0]);
    assert_eq!(emu.rf.read64(RegRax), 0);

    emu.mov_imm(&insts[1]);
    assert_eq!(emu.rf.read64(RegRcx), 0);

    emu.mov_imm(&insts[2]);
    assert_eq!(emu.rf.read64(RegRdx), 0);

    emu.mov_imm(&insts[3]);
    assert_eq!(emu.rf.read64(RegRbx), 0);
  }
}
