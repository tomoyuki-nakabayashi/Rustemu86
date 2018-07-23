use byteorder::{ReadBytesExt, LittleEndian};
use loader::BinaryReader;

#[derive(Debug)]
pub struct Rustemu86 {
  // Must have cpu, memory, peripherals
  rax: u64,
}

impl Rustemu86 {
  fn execute_mov_imm_instruction(&mut self, inst: &[u8]) {
    let mut imm = &inst[2..];
    self.rax = imm.read_u32::<LittleEndian>().unwrap().into();
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn execute_an_instruction() {
    let mut emu = Rustemu86{
      rax: 0xFFFFFFFF,
    };

    let inst: &[u8] = &[0xb8, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.execute_mov_imm_instruction(&inst);
    assert_eq!(emu.rax, 0);
  }
}
