use register_file::RegisterFile;
use instructions;

#[derive(Debug)]
pub struct Cpu {
  rf: RegisterFile,
  rip: u64,
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      rf: RegisterFile::new(),
      rip: 0,
    }
  }

  pub fn fetch<'a>(&mut self, buf: &'a Vec<u8>) -> &'a [u8] {
    let mut inst: &[u8] = &buf;
    let rip: usize = self.rip as usize;
    match buf[rip] {
      0x48 => inst = &buf[rip..=rip+2],
      0xb8 ... 0xbf => inst = &buf[rip..=rip+5],
      _ => (),
    }
    self.rip += inst.len() as u64;
    inst
  }

  pub fn decode(_inst: &[u8]) -> fn(&mut RegisterFile, &[u8]) {
    instructions::mov_imm64
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use instructions;
  use register_file::Reg64Id::{Rax, Rcx, Rdx, Rbx};

  #[test]
  fn fetch() {
    let mut cpu = Cpu::new();

    cpu.fetch(&vec![0xb8, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(cpu.rip, 6);
  }

  #[test]
  fn fetch_instructions() {
    let mut cpu = Cpu::new();
    let mut insts = vec![0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0xff, 0xc0];

    cpu.fetch(&insts);
    assert_eq!(cpu.rip, 6);
    cpu.fetch(&insts);
    assert_eq!(cpu.rip, 9);
  }

  #[test]
  fn decode() {
    let mut cpu = Cpu::new();
    let inst = Cpu::decode(&[0xb8, 0x00, 0x00, 0x00, 0x00, 0x00]);

    inst(&mut cpu.rf, &[0xb8, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(cpu.rf.read64(Rax), 0);
  }

  #[test]
  fn execute_mov_imm64() {
    let mut cpu = Cpu::new();

    let mut insts: Vec<&[u8]> = Vec::with_capacity(4);
    insts.push(&[0xb8, 0x00, 0x00, 0x00, 0x00, 0x00]);  // mov rax, 0
    insts.push(&[0xb9, 0x00, 0x00, 0x00, 0x00, 0x00]);  // mov rcx, 0
    insts.push(&[0xba, 0x00, 0x00, 0x00, 0x00, 0x00]);  // mov rdx, 0
    insts.push(&[0xbb, 0x00, 0x00, 0x00, 0x00, 0x00]);  // mov rbx, 0

    instructions::mov_imm64(&mut cpu.rf, &insts[0]);
    assert_eq!(cpu.rf.read64(Rax), 0);

    instructions::mov_imm64(&mut cpu.rf, &insts[1]);
    assert_eq!(cpu.rf.read64(Rcx), 0);

    instructions::mov_imm64(&mut cpu.rf, &insts[2]);
    assert_eq!(cpu.rf.read64(Rdx), 0);

    instructions::mov_imm64(&mut cpu.rf, &insts[3]);
    assert_eq!(cpu.rf.read64(Rbx), 0);
  }

  #[test]
  fn execute_inc_reg() {
    let mut cpu = Cpu::new();
    instructions::mov_imm64(&mut cpu.rf, &[0xb8, 0x00, 0x00, 0x00, 0x00, 0x00]);

    let insts: &[u8] = &[0x48, 0xff, 0xc0];
    for i in 1..10 {
      instructions::inc(&mut cpu.rf, insts);
      assert_eq!(cpu.rf.read64(Rax), i);
    }
  }

  #[test]
  fn execute_add() {
    let mut cpu = Cpu::new();
    instructions::mov_imm64(&mut cpu.rf, &[0xb8, 0x00, 0x00, 0x00, 0x00, 0x01]);
    instructions::mov_imm64(&mut cpu.rf, &[0xb9, 0x00, 0x00, 0x00, 0x00, 0x02]);

    let insts: &[u8] = &[0x48, 0x01, 0xc8];
    instructions::add(&mut cpu.rf, insts);
    assert_eq!(cpu.rf.read64(Rax), 3);
  }
}