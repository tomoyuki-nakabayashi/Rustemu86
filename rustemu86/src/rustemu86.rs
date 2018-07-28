use byteorder::{ReadBytesExt, LittleEndian};
use loader::BinaryReader;
use register_file::RegisterFile;
use instructions;

#[derive(Debug)]
pub struct Rustemu86 {
  // Must have cpu, memory, peripherals
  rf: RegisterFile,
  rip: u64,
}

impl Rustemu86 {
  pub fn new() -> Rustemu86 {
    Rustemu86 {
      rf: RegisterFile::new(),
      rip: 0,
    }
  }

  fn update_rip(&mut self, inst: &[u8]) {
    let inst_length: u64 = inst.len() as u64;
    self.rip += inst_length;
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use register_file::Reg64Id::{Rax, Rcx, Rdx, Rbx};

  #[test]
  fn increment_program_counter() {
    let mut emu = Rustemu86::new();

    emu.update_rip(&[0xb8, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(emu.rip, 6);
  }

  #[test]
  fn execute_mov_imm64() {
    let mut emu = Rustemu86::new();

    let mut insts: Vec<&[u8]> = Vec::with_capacity(4);
    insts.push(&[0xb8, 0x00, 0x00, 0x00, 0x00, 0x00]);  // mov rax, 0
    insts.push(&[0xb9, 0x00, 0x00, 0x00, 0x00, 0x00]);  // mov rcx, 0
    insts.push(&[0xba, 0x00, 0x00, 0x00, 0x00, 0x00]);  // mov rdx, 0
    insts.push(&[0xbb, 0x00, 0x00, 0x00, 0x00, 0x00]);  // mov rbx, 0

    instructions::mov_imm64(&mut emu.rf, &insts[0]);
    assert_eq!(emu.rf.read64(Rax), 0);

    instructions::mov_imm64(&mut emu.rf, &insts[1]);
    assert_eq!(emu.rf.read64(Rcx), 0);

    instructions::mov_imm64(&mut emu.rf, &insts[2]);
    assert_eq!(emu.rf.read64(Rdx), 0);

    instructions::mov_imm64(&mut emu.rf, &insts[3]);
    assert_eq!(emu.rf.read64(Rbx), 0);
  }

  #[test]
  fn execute_inc_reg() {
    let mut emu = Rustemu86::new();
    instructions::mov_imm64(&mut emu.rf, &[0xb8, 0x00, 0x00, 0x00, 0x00, 0x00]);

    let insts: &[u8] = &[0x48, 0xff, 0xc0];
    for i in 1..10 {
      instructions::inc(&mut emu.rf, insts);
      assert_eq!(emu.rf.read64(Rax), i);
    }
  }

  #[test]
  fn execute_add() {
    let mut emu = Rustemu86::new();
    instructions::mov_imm64(&mut emu.rf, &[0xb8, 0x00, 0x00, 0x00, 0x00, 0x01]);
    instructions::mov_imm64(&mut emu.rf, &[0xb9, 0x00, 0x00, 0x00, 0x00, 0x02]);

    let insts: &[u8] = &[0x48, 0x01, 0xc8];
    instructions::add(&mut emu.rf, insts);
    assert_eq!(emu.rf.read64(Rax), 3);
  }
}
