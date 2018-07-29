use loader::BinaryReader;
use cpu::Cpu;

#[derive(Debug)]
pub struct Rustemu86 {
  // Must have cpu, memory, peripherals
  cpu: Cpu,
}

impl Rustemu86 {
  pub fn new() -> Rustemu86 {
    Rustemu86 {
      cpu: Cpu::new(),
    }
  }
}
