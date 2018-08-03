use instructions;
use instructions::DecodedInst;
use instructions::DestType;
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
      let inst = self.decode(&inst).unwrap();
      self.execute(&inst);
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

  fn decode(&self, inst: &[u8]) -> Result<DecodedInst, InternalException> {
    match inst[0] {
      0x48 => match inst[1] {
        0x01 => Ok(instructions::decode_add(&self.rf, &inst)),
        0xff => Ok(instructions::decode_inc(&self.rf, &inst)),
        opcode @ _ => Err(InternalException::UndefinedInstruction {opcode}),
      },
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
  use register_file::Reg64Id::{Rax, Rcx};

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
  fn decode_undefined_instruction() {
    let cpu = Cpu::new();
    let inst = vec![0x06];
    assert!(cpu.decode(&inst).is_err());
  }

  #[test]
  fn mov64() {
    let inst = vec![0xb8, 0x00, 0x00, 0x00, 0x00];
    let mut cpu = Cpu::new();
    let inst = cpu.fetch(&inst);
    let inst = cpu.decode(&inst).unwrap();
    cpu.execute(&inst);

    assert_eq!(cpu.rf.read64(Rax), 0);
  }

  #[test]
  fn execute_inc() {
    let inst = vec![0x48, 0xff, 0xc0];
    let mut cpu = Cpu::new();
    cpu.rf.write64(Rax, 0);

    for i in 1..10 {
      let inst = cpu.decode(&inst).unwrap();
      cpu.execute(&inst);
      assert_eq!(cpu.rf.read64(Rax), i);
    }
  }

  #[test]
  fn execute_add() {
    let mut cpu = Cpu::new();
    cpu.rf.write64(Rax, 1);
    cpu.rf.write64(Rcx, 2);

    let inst = vec![0x48, 0x01, 0xc8];
    let inst = cpu.decode(&inst).unwrap();
    cpu.execute(&inst);
    assert_eq!(cpu.rf.read64(Rax), 3);
  }

  #[test]
  fn execute_jmp() {
    let mut cpu = Cpu::new();
    instructions::jmp(&mut cpu.rip, &[0xeb, 0x05]);

    assert_eq!(cpu.rip, 5);
  }
}
