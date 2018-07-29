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

pub fn start_emulation(bin: &mut BinaryReader) -> io::Result<()> {
  let mut program = Vec::new();
  bin.reader.read_to_end(&mut program)?;
  println!("Program load... {} bytes.", program.len());
  let mut cpu = Cpu::new();
  cpu.run(&program)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn success_emulation() {
    let mut reader = loader::load("../tests/asms/simple_add").unwrap();
    let result = start_emulation(&mut reader);
    assert!(result.is_ok());
  }
}
