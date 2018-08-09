use cpu::isa::registers::Reg64Id;
use std::cmp::PartialEq;
use std::fmt;

#[derive(Debug)]
pub struct RegisterFile {
  rax: u64,
  rcx: u64,
  rdx: u64,
  rbx: u64,
}

impl RegisterFile {
  pub fn new() -> RegisterFile {
    RegisterFile {
      rax: 0xFFFFFFFF,
      rcx: 0xFFFFFFFF,
      rdx: 0xFFFFFFFF,
      rbx: 0xFFFFFFFF,
    }
  }

  pub fn write64(&mut self, dest: Reg64Id, value: u64) {
    match dest {
      Reg64Id::Rax => self.rax = value,
      Reg64Id::Rcx => self.rcx = value,
      Reg64Id::Rdx => self.rdx = value,
      Reg64Id::Rbx => self.rbx = value,
      Reg64Id::Unknown => (),
    }
  }

  pub fn read64(&self, src: Reg64Id) -> u64 {
    match src {
      Reg64Id::Rax => self.rax,
      Reg64Id::Rcx => self.rcx,
      Reg64Id::Rdx => self.rdx,
      Reg64Id::Rbx => self.rbx,
      Reg64Id::Unknown => 0,
    }
  }
}

impl fmt::Display for RegisterFile {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "  rax: 0x{:>08X}\n  rcx: 0x{:>08X}\n  rdx: 0x{:>08X}\n  rbx: 0x{:>08X}",
      self.rax, self.rcx, self.rdx, self.rbx
    )
  }
}

impl PartialEq for RegisterFile {
  fn eq(&self, other: &RegisterFile) -> bool {
    return (self.rax == other.rax)
      && (self.rcx == other.rcx)
      && (self.rdx == other.rdx)
      && (self.rbx == other.rbx);
  }
}
