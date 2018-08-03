use instructions;
use instructions::DecodedInst;
use instructions::DestType;
use register_file::Reg64Id::*;
use register_file::RegisterFile;
use rustemu86::DebugMode;
use std::cmp::PartialEq;
use std::fmt;
use std::io;

#[derive(Debug, Fail)]
enum InternalException {
  #[fail(display = "undefined instruction: {}", opcode)]
  UndefinedInstruction {
    opcode: u8,
  },
}

#[derive(Debug)]
pub struct Cpu {
  rf: RegisterFile,
  rip: u64,
  executed_insts: u64,
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      rf: RegisterFile::new(),
      rip: 0,
      executed_insts: 0,
    }
  }

  pub fn run<T>(&mut self, program: &Vec<u8>, debug_mode: &T) -> io::Result<()>
  where
    T: DebugMode,
  {
    while (self.rip as usize) < program.len() {
      let inst: &[u8] = self.fetch(&program);
      let exec = Cpu::decode(&inst);
      exec(&mut self.rf, &inst);
      self.executed_insts += 1;
      debug_mode.do_cycle_end_action(&self);
    }
    println!("Finish emulation.");
    Ok(())
  }

  fn fetch<'a>(&mut self, buf: &'a Vec<u8>) -> &'a [u8] {
    let mut inst: &[u8] = &buf;
    let rip: usize = self.rip as usize;
    match buf[rip] {
      0x48 => inst = &buf[rip..rip + 3],
      0xb8...0xbf => inst = &buf[rip..rip + 5],
      _ => (),
    }
    self.rip += inst.len() as u64;
    inst
  }

  fn decode(inst: &[u8]) -> fn(&mut RegisterFile, &[u8]) {
    match inst[0] {
      0x48 => match inst[1] {
        0x01 => instructions::add,
        0xff => instructions::inc,
        _ => instructions::undefined,
      },
      0xb8...0xbf => instructions::mov_imm64,
      _ => instructions::undefined,
    }
  }

  fn decoder(&self, inst: &[u8]) -> Result<DecodedInst, InternalException> {
    match inst[0] {
      0xb8...0xbf => Ok(instructions::decode_mov_imm64(&inst)),
      opcode @ _ => Err(InternalException::UndefinedInstruction {opcode}),
    }
  }

  fn execute(&mut self, inst: &DecodedInst) {
    match inst.dest_type {
      DestType::Register => self.rf.write64(inst.dest_rf, inst.result),
    }
  }
}

impl fmt::Display for Cpu {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "=== CPU status ({} instructions executed.)===\nRIP: {}\nRegisters:\n{}",
      self.executed_insts, self.rip, self.rf
    )
  }
}

impl PartialEq for Cpu {
  fn eq(&self, other: &Cpu) -> bool {
    return (self.rip == other.rip)
      && (self.executed_insts == other.executed_insts)
      && (self.rf == other.rf);
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use instructions;
  use register_file::Reg64Id::{Rax, Rbx, Rcx, Rdx};

  #[test]
  fn compare_cpus() {
    let cpu1 = Cpu::new();
    let cpu2 = Cpu::new();

    assert_eq!(cpu1, cpu2);
  }

  #[test]
  fn fetch_instructions() {
    let mut cpu = Cpu::new();
    let insts = vec![0xb8, 0x00, 0x00, 0x00, 0x00, 0x48, 0xff, 0xc0];

    cpu.fetch(&insts);
    assert_eq!(cpu.rip, 5);
    cpu.fetch(&insts);
    assert_eq!(cpu.rip, 8);
  }

  #[test]
  fn decode() {
    let mut cpu = Cpu::new();
    let inst = vec![0xb8, 0x00, 0x00, 0x00, 0x00];
    let exec = Cpu::decode(cpu.fetch(&inst));

    exec(&mut cpu.rf, &[0xb8, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(cpu.rf.read64(Rax), 0);
  }

  #[test]
  fn execute_inst() {
    let mut cpu = Cpu::new();
    let inst = vec![0xb8, 0x00, 0x00, 0x00, 0x00];
    let inst = cpu.decoder(&inst).unwrap();
    cpu.execute(&inst);

    assert_eq!(cpu.rf.read64(Rax), 0);
  }

  #[test]
  fn decode_undefined_instruction() {
    let cpu = Cpu::new();
    let inst = vec![0x06];
    assert!(cpu.decoder(&inst).is_err());
  }

  #[test]
  fn execute_mov_imm64() {
    let mut cpu = Cpu::new();

    let mut insts: Vec<&[u8]> = Vec::with_capacity(4);
    insts.push(&[0xb8, 0x00, 0x00, 0x00, 0x00]); // mov rax, 0
    insts.push(&[0xb9, 0x00, 0x00, 0x00, 0x00]); // mov rcx, 0
    insts.push(&[0xba, 0x00, 0x00, 0x00, 0x00]); // mov rdx, 0
    insts.push(&[0xbb, 0x00, 0x00, 0x00, 0x00]); // mov rbx, 0

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
    instructions::mov_imm64(&mut cpu.rf, &[0xb8, 0x00, 0x00, 0x00, 0x00]);

    let insts: &[u8] = &[0x48, 0xff, 0xc0];
    for i in 1..10 {
      instructions::inc(&mut cpu.rf, insts);
      assert_eq!(cpu.rf.read64(Rax), i);
    }
  }

  #[test]
  fn execute_add() {
    let mut cpu = Cpu::new();
    instructions::mov_imm64(&mut cpu.rf, &[0xb8, 0x01, 0x00, 0x00, 0x00]);
    instructions::mov_imm64(&mut cpu.rf, &[0xb9, 0x02, 0x00, 0x00, 0x00]);

    let insts: &[u8] = &[0x48, 0x01, 0xc8];
    instructions::add(&mut cpu.rf, insts);
    assert_eq!(cpu.rf.read64(Rax), 3);
  }

  #[test]
  fn execute_jmp() {
    let mut cpu = Cpu::new();
    instructions::jmp(&mut cpu.rip, &[0xeb, 0x05]);

    assert_eq!(cpu.rip, 5);
  }
}
