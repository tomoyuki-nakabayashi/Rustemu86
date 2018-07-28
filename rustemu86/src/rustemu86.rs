use byteorder::{ReadBytesExt, LittleEndian};
use loader::BinaryReader;
use register_file::RegisterFile;
use instructions;

#[derive(Debug)]
pub struct Rustemu86 {
  // Must have cpu, memory, peripherals
  rf: RegisterFile,
}

#[cfg(test)]
mod test {
  use super::*;
  use register_file::Reg64Id::{Rax, Rcx, Rdx, Rbx};

  #[test]
  fn execute_mov_imm64() {
    let mut emu = Rustemu86{
      rf: RegisterFile::new(),
    };

    let mut insts: Vec<&[u8]> = Vec::with_capacity(4);
    insts.push(&[0xb8, 0x00, 0x00, 0x00, 0x00, 0x00]);  // mov rax, 0
    insts.push(&[0xb9, 0x00, 0x00, 0x00, 0x00, 0x00]);  // mov rcx, 0
    insts.push(&[0xba, 0x00, 0x00, 0x00, 0x00, 0x00]);  // mov rdx, 0
    insts.push(&[0xbb, 0x00, 0x00, 0x00, 0x00, 0x00]);  // mov rbx, 0

    instructions::mov_imm64(&mut emu.rf, &insts[0]);
    assert_eq!(emu.rf.read64(Rax), 0);

    instructions::mov_imm64(&mut emu.rf, &insts[1]);
    assert_eq!(emu.rf.read64(Rcx), 0);

    instructions::mov_imm64(&mut emu.rf, &insts[2]);
    assert_eq!(emu.rf.read64(Rdx), 0);

    instructions::mov_imm64(&mut emu.rf, &insts[3]);
    assert_eq!(emu.rf.read64(Rbx), 0);
  }

  #[test]
  fn execute_inc_reg() {
    let mut emu = Rustemu86{
      rf: RegisterFile::new(),
    };
    instructions::mov_imm64(&mut emu.rf, &[0xb8, 0x00, 0x00, 0x00, 0x00, 0x00]);

    let insts: &[u8] = &[0x48, 0xff, 0xc0];
    instructions::inc(&mut emu.rf, insts);
    assert_eq!(emu.rf.read64(Rax), 1);
    instructions::inc(&mut emu.rf, insts);
    assert_eq!(emu.rf.read64(Rax), 2);
  }
}
