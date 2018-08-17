use cpu::isa::registers::Reg64Id;
use std::cmp::PartialEq;
use std::fmt;

#[derive(Debug)]
pub struct RegisterFile {
  rax: u64,
  rcx: u64,
  rdx: u64,
  rbx: u64,
  rsi: u64,
  rdi: u64,
  rbp: u64,
  rsp: u64,
}

impl RegisterFile {
  pub fn new() -> RegisterFile {
    RegisterFile {
      rax: 0xFFFFFFFF,
      rcx: 0xFFFFFFFF,
      rdx: 0xFFFFFFFF,
      rbx: 0xFFFFFFFF,
      rsi: 0xFFFFFFFF,
      rdi: 0xFFFFFFFF,
      rbp: 0xFFFFFFFF,
      rsp: 0xFFFFFFFF,
    }
  }

  pub fn write64(&mut self, dest: Reg64Id, value: u64) {
    match dest {
      Reg64Id::Rax => self.rax = value,
      Reg64Id::Rcx => self.rcx = value,
      Reg64Id::Rdx => self.rdx = value,
      Reg64Id::Rbx => self.rbx = value,
      Reg64Id::Rsi => self.rsi = value,
      Reg64Id::Rdi => self.rdi = value,
      Reg64Id::Rbp => self.rbp = value,
      Reg64Id::Rsp => self.rsp = value,
      Reg64Id::Unknown => (),
    }
  }

  pub fn read64(&self, src: Reg64Id) -> u64 {
    match src {
      Reg64Id::Rax => self.rax,
      Reg64Id::Rcx => self.rcx,
      Reg64Id::Rdx => self.rdx,
      Reg64Id::Rbx => self.rbx,
      Reg64Id::Rsi => self.rsi,
      Reg64Id::Rdi => self.rdi,
      Reg64Id::Rbp => self.rbp,
      Reg64Id::Rsp => self.rsp,
      Reg64Id::Unknown => 0,
    }
  }
}

impl fmt::Display for RegisterFile {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "  rax: 0x{:>08X}\n  rcx: 0x{:>08X}\n  rdx: 0x{:>08X}\n  rbx: 0x{:>08X}\n  rsi: 0x{:>08X}\n  rdi: 0x{:>08X}\n  rbp: 0x{:>08X}\n  rsp: 0x{:>08X}",
      self.rax, self.rcx, self.rdx, self.rbx, self.rsi, self.rdi, self.rbp, self.rsp
    )
  }
}

impl PartialEq for RegisterFile {
  fn eq(&self, other: &RegisterFile) -> bool {
    return (self.rax == other.rax)
      && (self.rcx == other.rcx)
      && (self.rdx == other.rdx)
      && (self.rbx == other.rbx)
      && (self.rsi == other.rsi)
      && (self.rdi == other.rdi)
      && (self.rbp == other.rbp)
      && (self.rsp == other.rsp);
  }
}
