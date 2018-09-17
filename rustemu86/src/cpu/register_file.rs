use cpu::isa::registers::Reg64Id;
use std::fmt;

#[derive(Debug)]
pub struct RegisterFile {
  ram: Vec<u64>,
}

impl RegisterFile {
  pub fn new() -> RegisterFile {
    RegisterFile {
      ram: vec![0; 8],
    }
  }

  pub fn write64(&mut self, dest: Reg64Id, value: u64) {
    self.ram[dest as usize] = value;
  }

  pub fn read64(&self, src: Reg64Id) -> u64 {
    self.ram[src as usize]
  }
}

impl fmt::Display for RegisterFile {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "  rax: 0x{:>08X}\n  rcx: 0x{:>08X}\n  rdx: 0x{:>08X}\n  rbx: 0x{:>08X}\n  rsp: 0x{:>08X}\n  rbp: 0x{:>08X}\n  rsi: 0x{:>08X}\n  rdi: 0x{:>08X}",
      self.ram[0], self.ram[1], self.ram[2], self.ram[3], self.ram[4], self.ram[5], self.ram[6], self.ram[7]
    )
  }
}
