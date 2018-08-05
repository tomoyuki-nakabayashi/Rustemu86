use cpu::decoder::ModRm;

pub struct InstructionX86_64 {
  pub lecacy_prefix: [u8; 4],
  pub opcode: [u8; 4],
  pub mod_rm: ModRm,
  pub sib: u8,
  pub displacement: u64,
  pub immediate: u64,
}