use cpu::Cpu;
use cpu::decoder::ModRm;
use cpu::opcode::*;
use byteorder::{LittleEndian, ReadBytesExt};

pub struct FetchedInst {
  pub lecacy_prefix: u32,
  pub opcode: u32,
  pub mod_rm: ModRm,
  pub sib: u8,
  pub displacement: u64,
  pub immediate: u64,
}

impl FetchedInst {
  pub fn new(prefix: u32, opcode: u32, mod_rm: ModRm, sib: u8, disp: u64, imm: u64) -> FetchedInst {
    FetchedInst {
      lecacy_prefix: prefix,
      opcode: opcode,
      mod_rm: mod_rm,
      sib: sib,
      displacement: disp,
      immediate: imm,
    }
  }
}

pub fn fetch(cpu: &Cpu, program: &[u8]) -> Result<FetchedInst, ()> {
  let rip = cpu.rip as usize;
  match program[rip] {
    MOV_RAX...MOV_DI => Ok(fetch_imm32_to_reg(&cpu, &program)),
    _ => Err(()),
  }
}

fn fetch_imm32_to_reg(cpu: &Cpu, program: &[u8]) -> FetchedInst {
  let rip = cpu.rip as usize;
  let opcode = program[rip] as u32;
  let mut imm = &program[rip+1..rip+5];
  let imm: u64 = imm.read_u32::<LittleEndian>().unwrap().into();
  FetchedInst::new(0, opcode, ModRm::new_invalid(), 0, 0, imm)
}