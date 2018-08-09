use cpu::decoder::ModRm;
use cpu::opcode::{REX, REX_WRXB};
use cpu::opcode::Opcode;
use cpu::InternalException;
use byteorder::{LittleEndian, ReadBytesExt};
use bit_field::BitField;
use num::FromPrimitive;

#[derive(Debug)]
pub struct FetchUnit {
  rip: usize,
}

impl FetchUnit {
  pub fn new() -> FetchUnit {
    FetchUnit{ rip: 0 }
  }

  pub fn fetch(&mut self, program: &[u8]) -> Result<FetchedInst, InternalException> {
    let inst = FetchedInstBuilder::new(self.rip as usize, &program)
                  .parse_rex_prefix()
                  .parse_r()
                  .parse_opcode()
                  .parse_modrm()
                  .parse_disp()
                  .parse_imm()
                  .build();
    self.rip = inst.next_rip;
    Ok(inst)
  }

  pub fn get_rip(&self) -> u64 {
    self.rip as u64
  }

  pub fn set_rip(&mut self, next_rip: u64) {
    self.rip = next_rip as usize
  }
}

pub struct FetchedInst {
  pub lecacy_prefix: u32,
  pub rex_prefix: u8,
  pub opcode: Opcode,
  pub r: u8,
  pub mod_rm: ModRm,
  pub sib: u8,
  pub displacement: u64,
  pub immediate: u64,
  pub next_rip: usize,
}

impl FetchedInst {
  pub fn new(prefix: u32, rex_prefix: u8, opcode: Opcode, r: u8, mod_rm: ModRm, sib: u8, disp: u64, imm: u64) -> FetchedInst {
    FetchedInst {
      lecacy_prefix: prefix,
      rex_prefix: rex_prefix,
      opcode: opcode,
      r: r,
      mod_rm: mod_rm,
      sib: sib,
      displacement: disp,
      immediate: imm,
      next_rip: 0,
    }
  }
}

struct FetchedInstBuilder<'a> {
  lecacy_prefix: u32,
  rex_prefix: u8,
  opcode: Opcode,  // Opcode enum.
  r: u8,
  mod_rm: ModRm,
  sib: u8,
  displacement: u64,
  immediate: u64,
  rip: usize,
  program: &'a [u8],
}

impl<'a> FetchedInstBuilder<'a> {
  fn new(rip: usize, program: &[u8]) -> FetchedInstBuilder {
    FetchedInstBuilder {
      lecacy_prefix: 0,
      rex_prefix: 0,
      opcode: Opcode::Invalid,
      r: 0,
      mod_rm: ModRm::new_invalid(),
      sib: 0,
      displacement: 0,
      immediate: 0,
      rip: rip,
      program: program,
    }
  }

  fn parse_rex_prefix(&mut self) -> &mut FetchedInstBuilder<'a> {
    let candidate = self.program[self.rip];
    match candidate {
      REX...REX_WRXB => { self.rex_prefix = candidate; self.rip += 1 },
      _ => (),
    }
    self
  }

  fn parse_r(&mut self) -> &mut FetchedInstBuilder<'a> {
    let candidate = self.program[self.rip];
    self.r = candidate.get_bits(0..3);
    self
  }

  fn parse_opcode(&mut self) -> &mut FetchedInstBuilder<'a> {
    let candidate = self.program[self.rip];
    let plus_r_opcode = || { Opcode::from_u8(candidate & 0xf8) };
    self.opcode = Opcode::from_u8(candidate)
                          .or_else(plus_r_opcode)
                          .or_else(|| Some(Opcode::Invalid))
                          .unwrap();
    self.rip += 1;
    self
  }

  fn parse_modrm(&mut self) -> &mut FetchedInstBuilder<'a> {
    match self.opcode {
      Opcode::Add | Opcode::Inc => {
        self.mod_rm = ModRm::new(self.program[self.rip]);
        self.rip += 1;
      }
      _ => (),
    }
    self
  }

  fn parse_disp(&mut self) -> &mut FetchedInstBuilder<'a> {
    match self.opcode {
      Opcode::JmpRel8 => { self.displacement = self.program[self.rip] as u64; self.rip += 1 }
      _ => (),
    }
    self
  }

  fn parse_imm(&mut self) -> &mut FetchedInstBuilder<'a> {
    match self.opcode {
      Opcode::MovImm32 => {
        let mut imm = &self.program[self.rip..self.rip+4];
        self.immediate = imm.read_u32::<LittleEndian>().unwrap().into();
        self.rip += 4
      }
      _ => (),
    }
    self
  }

  fn build(&self) -> FetchedInst {
    FetchedInst {
      lecacy_prefix: self.lecacy_prefix,
      rex_prefix: self.rex_prefix,
      opcode: self.opcode,
      r: self.r,
      mod_rm: self.mod_rm,
      sib: self.sib,
      displacement: self.displacement,
      immediate: self.immediate,
      next_rip: self.rip,
    }
  }
}
