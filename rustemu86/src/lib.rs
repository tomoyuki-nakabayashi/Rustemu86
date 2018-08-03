extern crate byteorder;
extern crate getopts;
#[macro_use] extern crate failure;

pub mod args;
pub mod cpu;
pub mod instructions;
pub mod loader;
pub mod register_file;
pub mod rustemu86;

use args::EmulationMode;
use cpu::Cpu;
use loader::BinaryReader;
use rustemu86::{Interactive, NoneDebug, PerCycleDump};
use std::io;
use std::io::Read;

pub fn start_emulation(bin: &mut BinaryReader, mode_option: EmulationMode) -> io::Result<()> {
  let mut program = Vec::new();
  bin.reader.read_to_end(&mut program)?;
  println!("Program load... {} bytes.", program.len());
  let mut cpu = Cpu::new();

  match mode_option {
    EmulationMode::Normal => cpu.run(&program, &NoneDebug {}),
    EmulationMode::PerCycleDump => cpu.run(&program, &PerCycleDump {}),
    EmulationMode::InteractiveMode => cpu.run(&program, &Interactive {}),
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn success_emulation() {
    let mut reader = loader::load("../tests/asms/simple_add").unwrap();
    let result = start_emulation(&mut reader, EmulationMode::Normal);
    assert!(result.is_ok());
  }

  #[test]
  fn success_emulation_with_per_cycle_dump() {
    let mut reader = loader::load("../tests/asms/simple_add").unwrap();
    let result = start_emulation(&mut reader, EmulationMode::PerCycleDump);
    assert!(result.is_ok());
  }
}
