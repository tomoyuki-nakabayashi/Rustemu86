use cpu::Cpu;
use cpu::decoder::ModRm;
use cpu::opcode::*;
use byteorder::{LittleEndian, ReadBytesExt};

/* 
trait Inst64 {
  fn opcode(&self) -> Opcode;  // Opcode is enum of 1 to 3 bytes.
}
 */

pub struct FetchedInst {
  pub lecacy_prefix: u32,
  // MandatoryPrefix, RexPrefix
  pub opcode: u32,  // Opcode enum.
  pub mod_rm: ModRm,
  pub sib: u8,
  pub displacement: u64,
  pub immediate: u64,
  pub length: u64,
}

impl FetchedInst {
  pub fn new(prefix: u32, opcode: u32, mod_rm: ModRm, sib: u8, disp: u64, imm: u64, len: u64) -> FetchedInst {
    FetchedInst {
      lecacy_prefix: prefix,
      opcode: opcode,
      mod_rm: mod_rm,
      sib: sib,
      displacement: disp,
      immediate: imm,
      length: len,
    }
  }
}

pub fn fetch(cpu: &Cpu, program: &[u8]) -> Result<FetchedInst, ()> {
  let rip = cpu.rip as usize;
  match program[rip] {
    REX_W => Ok(fetch_two_operand(&cpu, &program)),
    MOV_RAX...MOV_DI => Ok(fetch_imm32_to_reg(&cpu, &program)),
    JMP_REL8 => Ok(fetch_jmp_rel8(&cpu, &program)),
    _ => Err(()),
  }
}

fn fetch_imm32_to_reg(cpu: &Cpu, program: &[u8]) -> FetchedInst {
  let rip = cpu.rip as usize;
  let opcode = program[rip] as u32;
  let mut imm = &program[rip+1..rip+5];
  let imm: u64 = imm.read_u32::<LittleEndian>().unwrap().into();
  FetchedInst::new(0, opcode, ModRm::new_invalid(), 0, 0, imm, 5)
}

fn fetch_two_operand(cpu: &Cpu, program: &[u8]) -> FetchedInst {
  let rip = cpu.rip as usize;
  let mut opcode = &program[rip..rip+2];
  let opcode = opcode.read_u16::<LittleEndian>().unwrap().into();
  FetchedInst::new(0, opcode, ModRm::new(program[rip+2]), 0, 0, 0, 3)
}

fn fetch_jmp_rel8(cpu: &Cpu, program: &[u8]) -> FetchedInst {
  let rip = cpu.rip as usize;
  let opcode = program[rip] as u32;
  let disp = program[rip+1] as u64;
  FetchedInst::new(0, opcode, ModRm::new_invalid(), 0, disp, 0, 2)
}