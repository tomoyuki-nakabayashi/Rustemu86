extern crate bit_field;

pub mod register_file;
pub mod fetcher;
pub mod decoder;
pub mod ex_stage;
pub mod exceptions;
pub mod isa;

use self::register_file::RegisterFile;
use self::fetcher::FetchUnit;
use self::decoder::DecodedInst;
use self::decoder::DestType;
use self::ex_stage::WriteBackInst;
use self::exceptions::InternalException;
use rustemu86::DebugMode;
use std::fmt;

static mut MEMORY: [u64; 100] = [0; 100];

#[derive(Debug)]
pub struct Cpu {
  rf: RegisterFile,
  fetch_unit: FetchUnit,
  executed_insts: u64,
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      rf: RegisterFile::new(),
      fetch_unit: FetchUnit::new(),
      executed_insts: 0,
    }
  }

  pub fn run<T>(&mut self, program: &Vec<u8>, debug_mode: &T) -> Result<(), InternalException>
  where
    T: DebugMode,
  {
    while (self.fetch_unit.get_rip() as usize) < program.len() {
      let inst = self.fetch_unit.fetch(&program)?;
      let inst = decoder::decode(self.fetch_unit.get_rip(), &self.rf, &inst)?;
      self.execute(&inst);
      self.executed_insts += 1;
      debug_mode.do_cycle_end_action(&self);
    }
    println!("Finish emulation. {} instructions executed.", self.executed_insts);
    Ok(())
  }

  fn execute(&mut self, inst: &DecodedInst) {
    match inst.dest_type {
      DestType::Register => self.rf.write64(inst.dest_rf, inst.result),
      DestType::Rip => self.fetch_unit.set_rip(inst.result),
      DestType::Memory => unsafe { MEMORY[self.rf.read64(inst.dest_rf) as usize] = inst.result },
    }
  }

  fn new_execute(&mut self, inst: &WriteBackInst) {
    match inst.dest_type {
      DestType::Register => self.rf.write64(inst.dest_rf, inst.result),
      DestType::Rip => self.fetch_unit.set_rip(inst.result),
      DestType::Memory => unsafe { MEMORY[self.rf.read64(inst.dest_rf) as usize] = inst.result },
    }
  }
}

impl fmt::Display for Cpu {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "=== CPU status ({} instructions executed.)===\nRIP: {}\nRegisters:\n{}",
      self.executed_insts, self.fetch_unit.get_rip(), self.rf
    )
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use rustemu86;
  use cpu::isa::registers::Reg64Id::{Rax, Rcx, Rbx};

  #[test]
  fn execute_two_instructions() {
    let program = vec![0xb8, 0x00, 0x00, 0x00, 0x00, // mov rax, 0
                       0x48, 0xff, 0xc0];            // inc rax
    let mut cpu = Cpu::new();
    let result = cpu.run(&program, &rustemu86::NoneDebug{});

    assert!(result.is_ok());
    assert_eq!(cpu.fetch_unit.get_rip(), 8);
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
    assert_eq!(cpu.fetch_unit.get_rip(), 7);
  }

  #[test]
  fn execute_load_store() {
//    let program = vec![0x48, 0x89, 0x18, 0x48, 0x8b, 0x08];
    let program = vec![0x48, 0x89, 0x18];
    let mut cpu = Cpu::new();
    cpu.rf.write64(Rax, 0);
    cpu.rf.write64(Rbx, 1);

    let result = cpu.run(&program, &rustemu86::NoneDebug{});

    assert!(result.is_ok());
    assert_eq!(unsafe { MEMORY[0] }, 1);
//    assert_eq!(cpu.rf.read64(Rcx), 1);
  }

  #[test]
  fn new_decode_and_execute() {
    let program = vec![0x48, 0xff, 0xc0];
    let mut cpu = Cpu::new();
    cpu.rf.write64(Rax, 1);

    let inst = cpu.fetch_unit.fetch(&program).unwrap();
    let inst = decoder::new_decode(cpu.fetch_unit.get_rip(), &cpu.rf, &inst).unwrap();
    let inst = ex_stage::execute(&inst);
    cpu.new_execute(&inst);
    
    assert_eq!(cpu.rf.read64(Rax), 2);
  }
}
