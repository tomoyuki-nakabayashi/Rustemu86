extern crate bit_field;

pub mod register_file;
pub mod decoder;
pub mod fetcher;
pub mod opcode;
pub mod instruction;

use self::decoder::DecodedInst;
use self::decoder::DestType;
use self::opcode::*;
use self::register_file::RegisterFile;
use self::fetcher::FetchedInst;
use rustemu86::DebugMode;
use std::fmt;

#[derive(Debug, Fail)]
pub enum InternalException {
  #[fail(display = "fetch error")]
  FetchError{},
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

  pub fn run<T>(&mut self, program: &Vec<u8>, debug_mode: &T) -> Result<(), InternalException>
  where
    T: DebugMode,
  {
    while (self.rip as usize) < program.len() {
      let inst = self.fetch(&program)?;
      let inst = self.decode(&inst)?;
      self.execute(&inst);
      self.executed_insts += 1;
      debug_mode.do_cycle_end_action(&self);
    }
    println!("Finish emulation. {} instructions executed.", self.executed_insts);
    Ok(())
  }

  fn fetch(&mut self, program: &Vec<u8>) -> Result<FetchedInst, InternalException> {
    let inst = self::fetcher::fetch(&self, &program);
    if inst.is_err() {
      return Err(InternalException::FetchError{})
    }
    let inst = inst.unwrap();
    self.rip += inst.length;
    Ok(inst)
  }

  fn decode(&self, inst: &FetchedInst) -> Result<DecodedInst, InternalException> {
    match inst.opcode {
      ADD => Ok(decoder::decode_add_new(&self.rf, &inst)),
      INC => Ok(decoder::decode_inc_new(&self.rf, &inst)),
      MOV_RAX...MOV_DI => Ok(decoder::decode_mov_new(&inst)),
      JMP_REL8 => Ok(decoder::decode_jmp_new(self.rip, &inst)),
      opcode @ _ => Err(InternalException::UndefinedInstruction {opcode}),
    }
  }

  fn execute(&mut self, inst: &DecodedInst) {
    match inst.dest_type {
      DestType::Register => self.rf.write64(inst.dest_rf, inst.result),
      DestType::Rip => self.rip = inst.result,
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

#[cfg(test)]
mod test {
  use super::*;
  use rustemu86;
  use cpu::register_file::Reg64Id::{Rax, Rcx};

  #[test]
  fn execute_two_instructions() {
    let program = vec![0xb8, 0x00, 0x00, 0x00, 0x00, // mov rax, 0
                       0x48, 0xff, 0xc0];            // inc rax
    let mut cpu = Cpu::new();
    let result = cpu.run(&program, &rustemu86::NoneDebug{});

    assert!(result.is_ok());
    assert_eq!(cpu.rip, 8);
  }

  #[test]
  fn execute_mov32() {
    let program = vec![0xb8, 0x00, 0x00, 0x00, 0x00];
    let mut cpu = Cpu::new();

    let result = cpu.run(&program, &rustemu86::NoneDebug{});

    assert!(result.is_ok());
    assert_eq!(cpu.rf.read64(Rax), 0);
  }

  #[test]
  fn execute_inc() {
    let program = vec![0x48, 0xff, 0xc0];
    let mut cpu = Cpu::new();
    cpu.rf.write64(Rax, 0);

    let result = cpu.run(&program, &rustemu86::NoneDebug{});

    assert!(result.is_ok());
    assert_eq!(cpu.rf.read64(Rax), 1);
  }

  #[test]
  fn execute_add() {
    let program = vec![0x48, 0x01, 0xc8];
    let mut cpu = Cpu::new();
    cpu.rf.write64(Rax, 1);
    cpu.rf.write64(Rcx, 2);

    let result = cpu.run(&program, &rustemu86::NoneDebug{});
    
    assert!(result.is_ok());
    assert_eq!(cpu.rf.read64(Rax), 3);
  }

  #[test]
  fn execute_jmp() {
    let mut cpu = Cpu::new();
    let program = vec![0xeb, 0x05];

    let result = cpu.run(&program, &rustemu86::NoneDebug{});
    
    assert!(result.is_ok());
    assert_eq!(cpu.rip, 7);
  }
}
