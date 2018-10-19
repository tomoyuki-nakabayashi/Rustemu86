/// General Purpose Register File.
/// 

pub(crate) struct RegisterFile{
    rams: Vec<u64>,
}

impl RegisterFile {
    pub fn new() -> RegisterFile {
        RegisterFile {
            rams: vec![0xffff_ffff_ffff_ffff; 8],
        }
    }

    pub fn read_u64(&self, index: usize) -> u64 {
        self.rams[index]
    }

    pub fn write_u64(&mut self, index: usize, value: u64) {
        self.rams[index] = value;
    }
}

enum_from_primitive! {
  #[derive(Debug, Clone, Copy, PartialEq)]
  pub enum Reg32 {
    Eax = 0x00,
    Ecx = 0x01,
    Edx = 0x02,
    Ebx = 0x03,
    Esp = 0x04,
    Ebp = 0x05,
    Esi = 0x06,
    Edi = 0x07,
  }
}