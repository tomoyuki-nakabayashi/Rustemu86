use register_file::RegisterFile;

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

  pub fn decode(&mut self, inst: &[u8]) {
    self.rip += inst.len() as u64;
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use instructions;
  use register_file::Reg64Id::{Rax, Rcx, Rdx, Rbx};

  #[test]
  fn increment_program_counter() {
    let mut cpu = Cpu::new();

    cpu.decode(&[0xb8, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(cpu.rip, 6);
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