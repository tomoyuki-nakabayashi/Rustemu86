pub enum GeneralRegisterId {
  RegRax,
  RegRcx,
  RegRdx,
  RegRbx,
}

use self::GeneralRegisterId::{RegRax, RegRcx, RegRdx, RegRbx};
impl GeneralRegisterId {
  pub fn from_u8(n: u8) -> Option<GeneralRegisterId> {
    match n {
      0 => Some(RegRax),
      1 => Some(RegRcx),
      2 => Some(RegRdx),
      3 => Some(RegRbx),
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

  pub fn write64(&mut self, dest: GeneralRegisterId, value: u64) {
    match dest {
      GeneralRegisterId::RegRax => self.rax = value,
      GeneralRegisterId::RegRcx => self.rcx = value,
      GeneralRegisterId::RegRdx => self.rdx = value,
      GeneralRegisterId::RegRbx => self.rbx = value,
    }
  }

  pub fn read64(&mut self, src: GeneralRegisterId) -> u64 {
    match src {
      GeneralRegisterId::RegRax => self.rax,
      GeneralRegisterId::RegRcx => self.rcx,
      GeneralRegisterId::RegRdx => self.rdx,
      GeneralRegisterId::RegRbx => self.rbx,
    }
  }
}