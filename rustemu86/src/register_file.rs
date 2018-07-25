pub enum GeneralRegisterId {
  RegRax,
  RegRcx,
  RegRdx,
  RegRbx,
}

#[derive(Debug)]
pub struct RegisterFile {
  pub rax: u64,
  pub rcx: u64,
  pub rdx: u64,
  pub rbx: u64,
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
}