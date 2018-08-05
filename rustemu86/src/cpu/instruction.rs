use cpu::decoder::ModRm;

pub struct InstructionX86_64 {
  pub lecacy_prefix: u32,
  pub opcode: u32,
  pub mod_rm: ModRm,
  pub sib: u8,
  pub displacement: u64,
  pub immediate: u64,
}

impl InstructionX86_64 {
  pub fn new(prefix: u32, opcode: u32, mod_rm: ModRm, sib: u8, disp: u64, imm: u64) -> InstructionX86_64 {
    InstructionX86_64 {
      lecacy_prefix: prefix,
      opcode: opcode,
      mod_rm: mod_rm,
      sib: sib,
      displacement: disp,
      immediate: imm,
    }
  }
}
