extern crate getopts;
extern crate byteorder;

pub mod args;
pub mod loader;
pub mod rustemu86;
pub mod cpu;
pub mod register_file;
pub mod instructions;

use std::io;
use std::io::Read;
use loader::BinaryReader;
use cpu::Cpu;
use args::EmulationMode;
use rustemu86::{EmulationStrategy, NormalEmulation, PerCycleDumpEmulation};

pub fn start_emulation(bin: &mut BinaryReader, mode_option: EmulationMode)
    -> io::Result<()> {
  let mut program = Vec::new();
  bin.reader.read_to_end(&mut program)?;
  println!("Program load... {} bytes.", program.len());
  let mut cpu = Cpu::new();

  match mode_option {
    EmulationMode::Normal => cpu.run(&program),
    EmulationMode::PerCycleDump => cpu.run_with_dump(&program, &PerCycleDumpEmulation{}),
    EmulationMode::InteractiveMode => Ok(()),
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
