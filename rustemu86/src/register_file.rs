use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Reg64Id {
  Rax,
  Rcx,
  Rdx,
  Rbx,
}

use self::Reg64Id::{Rax, Rcx, Rdx, Rbx};
impl Reg64Id {
  pub fn from_u8(n: u8) -> Option<Reg64Id> {
    match n {
      0 => Some(Rax),
      1 => Some(Rcx),
      2 => Some(Rdx),
      3 => Some(Rbx),
      _ => None,
    }
  }
}

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
    }
  }

  pub fn read64(&mut self, src: Reg64Id) -> u64 {
    match src {
      Reg64Id::Rax => self.rax,
      Reg64Id::Rcx => self.rcx,
      Reg64Id::Rdx => self.rdx,
      Reg64Id::Rbx => self.rbx,
    }
  }
}

impl fmt::Display for RegisterFile {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "  rax: {}\n  rcx: {}\n  rdx: {}\n  rbx: {}",
        self.rax, self.rcx, self.rdx, self.rbx)
  }
}
