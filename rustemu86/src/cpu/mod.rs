extern crate bit_field;

pub mod register_file;
pub mod fetcher;
pub mod decoder;
pub mod ex_stage;
pub mod exceptions;
pub mod isa;

use self::register_file::RegisterFile;
use self::fetcher::FetchUnit;
use self::decoder::DestType;
use self::ex_stage::WriteBackInst;
use self::exceptions::InternalException;
use peripherals::interconnect::Interconnect;
use rustemu86::DebugMode;
use std::fmt;

pub struct Cpu {
  rf: RegisterFile,
  fetch_unit: FetchUnit,
  executed_insts: u64,
  interconnect: Interconnect,
  state: CpuState,
}

impl Cpu {
  pub fn new(interconnect: Interconnect) -> Cpu {
    Cpu {
      rf: RegisterFile::new(),
      fetch_unit: FetchUnit::new(),
      executed_insts: 0,
      interconnect: interconnect,
      state: CpuState::Running,
    }
  }

  pub fn run<T>(&mut self, debug_mode: &T) -> Result<(), InternalException>
  where
    T: DebugMode,
  {
    while self.state == CpuState::Running {
      let inst_candidate = self.interconnect.fetch_inst_candidate(self.fetch_unit.get_rip());
      let inst = self.fetch_unit.fetch_new(&inst_candidate)?;
      let inst = decoder::decode(&self.rf, &inst)?;
      let inst = ex_stage::execute(&inst);
      self.write_back(&inst);
      self.executed_insts += 1;
      debug_mode.do_cycle_end_action(&self);
    }
    println!("Finish emulation. {} instructions executed.", self.executed_insts);
    Ok(())
  }

  fn write_back(&mut self, inst: &WriteBackInst) {
    match inst.dest_type {
      DestType::Register => self.rf.write64(inst.dest_rf, inst.result),
      DestType::Rip => self.fetch_unit.set_rip(inst.result),
      DestType::CpuState => self.state = CpuState::Halt,
      DestType::Memory => self.interconnect.write64(inst.addr, inst.result),
      DestType::MemToReg => self.rf.write64(inst.dest_rf, self.interconnect.read64(inst.addr)),
    }
    self.executed_insts += 1;
  }
}

impl fmt::Debug for Cpu {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "=== CPU status ({} instructions executed.)===\nRIP: {}\nRegisters:\n{}",
      self.executed_insts, self.fetch_unit.get_rip(), self.rf
    )
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

#[derive(PartialEq, Debug)]
enum CpuState {
  Running,
  Halt,
}

#[cfg(test)]
mod test {
  use super::*;
  use rustemu86;
  use ::args::EmulationMode;
  use peripherals::interconnect::Interconnect;
  use cpu::isa::registers::Reg64Id::{Rax, Rcx, Rbx};

  fn execute_program(program: Vec<u8>) -> Cpu {
    let mut interconnect = Interconnect::new(EmulationMode::Normal);
    interconnect.init_memory(program);
    let mut cpu = Cpu::new(interconnect);
    let result = cpu.run(&rustemu86::NoneDebug{});

    assert!(result.is_ok(), "{:?}", result.err());
    cpu
  }

  fn execute_program_after_init(program: Vec<u8>, initializer: &Fn(&mut Cpu)) -> Cpu {
    let mut interconnect = Interconnect::new(EmulationMode::Normal);
    interconnect.init_memory(program);
    let mut cpu = Cpu::new(interconnect);
    initializer(&mut cpu);
    let result = cpu.run(&rustemu86::NoneDebug{});

    assert!(result.is_ok(), "{:?}", result.err());
    cpu
  }

  #[test]
  fn execute_two_instructions() {
    let program = vec![0xb8, 0x00, 0x00, 0x00, 0x00, // mov rax, 0
                       0x48, 0xff, 0xc0,             // inc rax
                       0xf4];                        // hlt
    let cpu = execute_program(program);
    assert_eq!(cpu.fetch_unit.get_rip(), 9);
  }

  #[test]
  fn execute_mov32() {
    let program = vec![0xb8, 0x00, 0x00, 0x00, 0x00, 0xf4];
    let cpu = execute_program(program);
    assert_eq!(cpu.rf.read64(Rax), 0);
  }

  #[test]
  fn execute_inc() {
    let program = vec![0x48, 0xff, 0xc0, 0xf4];
    let initializer = |cpu: &mut Cpu| cpu.rf.write64(Rax, 0);
    let cpu = execute_program_after_init(program, &initializer);
    assert_eq!(cpu.rf.read64(Rax), 1);
  }

  #[test]
  fn execute_add() {
    let program = vec![0x48, 0x01, 0xc8, 0xf4];
    let initializer = |cpu: &mut Cpu| {
      cpu.rf.write64(Rax, 1);
      cpu.rf.write64(Rcx, 2);
    };
    let cpu = execute_program_after_init(program, &initializer);
    assert_eq!(cpu.rf.read64(Rax), 3);
  }

  #[test]
  fn execute_jmp() {
    let program = vec![0xeb, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf4];
    let cpu = execute_program(program);
    assert_eq!(cpu.fetch_unit.get_rip(), 8);
  }

  #[test]
  fn execute_load_store() {
    let program = vec![0x48, 0x89, 0x18, 0x48, 0x8b, 0x08, 0xf4];
    let initializer = |cpu: &mut Cpu| {
      cpu.rf.write64(Rax, 100);
      cpu.rf.write64(Rbx, 1);
    };
    let cpu = execute_program_after_init(program, &initializer);
    assert_eq!(cpu.interconnect.read64(100), 1);
    assert_eq!(cpu.rf.read64(Rcx), 1);
  }
}
