extern crate byteorder;
extern crate getopts;
extern crate num;
extern crate bit_field;
#[macro_use] extern crate failure;
#[macro_use] extern crate enum_primitive;

pub mod args;
pub mod cpu;
pub mod peripherals;
pub mod loader;
pub mod rustemu86;

use args::EmulationMode;
use cpu::Cpu;
use peripherals::interconnect::Interconnect;
use rustemu86::{Interactive, NoneDebug, PerCycleDump};

pub struct CpuError {}

pub fn start_emulation(program: &mut Vec<u8>, mode_option: EmulationMode) -> Result<(), CpuError> {
  let mut interconnect = Interconnect::new(mode_option);
  let should_remove_program = program.clone();
  interconnect.init_memory(should_remove_program);
  let mut cpu = Cpu::new(interconnect);

  let result = match mode_option {
    EmulationMode::Normal | EmulationMode::IntegrationTest => cpu.run(&program, &NoneDebug {}),
    EmulationMode::PerCycleDump => cpu.run(&program, &PerCycleDump {}),
    EmulationMode::InteractiveMode => cpu.run(&program, &Interactive {}),
  };

  match result {
    Ok(_) => Ok(()),
    Err(_) => Err(CpuError{}),
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn success_emulation() {
    let mut reader = loader::load("./tests/asms/simple_add").unwrap();
    let mut program = loader::map_to_memory(&mut reader).unwrap();
    let result = start_emulation(&mut program, EmulationMode::Normal);
    assert!(result.is_ok());
  }

  #[test]
  fn success_emulation_with_per_cycle_dump() {
    let mut reader = loader::load("./tests/asms/simple_add").unwrap();
    let mut program = loader::map_to_memory(&mut reader).unwrap();
    let result = start_emulation(&mut program, EmulationMode::PerCycleDump);
    assert!(result.is_ok());
  }
}
