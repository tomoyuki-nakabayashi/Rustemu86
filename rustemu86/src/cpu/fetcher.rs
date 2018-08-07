use cpu::decoder::ModRm;
use cpu::opcode::*;
use cpu::InternalException;
use cpu::InternalException::FetchError;
use byteorder::{LittleEndian, ReadBytesExt};

/* 
trait Inst64 {
  fn opcode(&self) -> Opcode;  // Opcode is enum of 1 to 3 bytes.
}
 */

#[derive(Debug)]
pub struct FetchUnit {
  rip: u64,
}

impl FetchUnit {
  pub fn new() -> FetchUnit {
    FetchUnit{ rip: 0 }
  }

  pub fn fetch(&mut self, program: &[u8]) -> Result<FetchedInst, InternalException> {
    let current = self.rip;
    match program[current as usize] {
      REX_W => { self.rip += 3; Ok(fetch_two_operand(current, &program)) },
      MOV_RAX...MOV_DI => { self.rip += 5; Ok(fetch_imm32_to_reg(current, &program)) },
      JMP_REL8 => { self.rip += 2; Ok(fetch_jmp_rel8(current, &program)) },
      _ => Err(FetchError{}),
    }
  }

  pub fn get_rip(&self) -> u64 {
    self.rip
  }

  pub fn set_rip(&mut self, next_rip: u64) {
    self.rip = next_rip
  }
}

pub struct FetchedInst {
  pub lecacy_prefix: u32,
  // MandatoryPrefix, RexPrefix
  pub opcode: u8,  // Opcode enum.
  pub mod_rm: ModRm,
  pub sib: u8,
  pub displacement: u64,
  pub immediate: u64,
  pub length: u64,
}

impl FetchedInst {
  pub fn new(prefix: u32, opcode: u8, mod_rm: ModRm, sib: u8, disp: u64, imm: u64, len: u64) -> FetchedInst {
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

fn fetch_imm32_to_reg(rip: u64, program: &[u8]) -> FetchedInst {
  let rip = rip as usize;
  let opcode = program[rip];
  let mut imm = &program[rip+1..rip+5];
  let imm: u64 = imm.read_u32::<LittleEndian>().unwrap().into();
  FetchedInst::new(0, opcode, ModRm::new_invalid(), 0, 0, imm, 5)
}

fn fetch_two_operand(rip: u64, program: &[u8]) -> FetchedInst {
  let rip = rip as usize;
  let opcode = program[rip+1];
  FetchedInst::new(0, opcode, ModRm::new(program[rip+2]), 0, 0, 0, 3)
}

fn fetch_jmp_rel8(rip: u64, program: &[u8]) -> FetchedInst {
  let rip = rip as usize;
  let opcode = program[rip];
  let disp = program[rip+1] as u64;
  FetchedInst::new(0, opcode, ModRm::new_invalid(), 0, disp, 0, 2)
}