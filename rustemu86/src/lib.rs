extern crate byteorder;
extern crate getopts;
#[macro_use] extern crate failure;

pub mod args;
pub mod cpu;
pub mod loader;
pub mod rustemu86;

use args::EmulationMode;
use cpu::Cpu;
use rustemu86::{Interactive, NoneDebug, PerCycleDump};

pub struct CpuError {}

pub fn start_emulation(program: &mut Vec<u8>, mode_option: EmulationMode) -> Result<(), CpuError> {
  let mut cpu = Cpu::new();

  let _result = match mode_option {
    EmulationMode::Normal => cpu.run(&program, &NoneDebug {}),
    EmulationMode::PerCycleDump => cpu.run(&program, &PerCycleDump {}),
    EmulationMode::InteractiveMode => cpu.run(&program, &Interactive {}),
  };

  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn success_emulation() {
    let mut reader = loader::load("../tests/asms/simple_add").unwrap();
    let mut program = loader::map_to_memory(&mut reader).unwrap();
    let result = start_emulation(&mut program, EmulationMode::Normal);
    assert!(result.is_ok());
  }

  #[test]
  fn success_emulation_with_per_cycle_dump() {
    let mut reader = loader::load("../tests/asms/simple_add").unwrap();
    let mut program = loader::map_to_memory(&mut reader).unwrap();
    let result = start_emulation(&mut program, EmulationMode::PerCycleDump);
    assert!(result.is_ok());
  }
}
