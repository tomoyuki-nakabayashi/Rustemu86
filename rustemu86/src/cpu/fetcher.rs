use cpu::isa::modrm::ModRm;
use cpu::isa::modrm::Sib;
use cpu::isa::opcode::{REX, REX_WRXB};
use cpu::isa::opcode::Opcode;
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
                  .parse_opcode()
                  .parse_modrm()
                  .parse_sib()
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
  pub mod_rm: Option<ModRm>,
  pub sib: Option<Sib>,
  pub displacement: u64,
  pub immediate: u64,
  pub next_rip: usize,
}

impl FetchedInst {
  pub fn new(prefix: u32, rex_prefix: u8, opcode: Opcode, r: u8, mod_rm: Option<ModRm>, sib: Option<Sib>, disp: u64, imm: u64) -> FetchedInst {
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
  mod_rm: Option<ModRm>,
  sib: Option<Sib>,
  displacement: u64,
  immediate: u64,
  rip_base: usize,
  rip_offset: usize,
  program: &'a [u8],
}

impl<'a> FetchedInstBuilder<'a> {
  fn new(rip: usize, program: &[u8]) -> FetchedInstBuilder {
    FetchedInstBuilder {
      lecacy_prefix: 0,
      rex_prefix: 0,
      opcode: Opcode::Invalid,
      r: 0,
      mod_rm: None,
      sib: None,
      displacement: 0,
      immediate: 0,
      rip_base: rip,
      rip_offset: 0,
      program: program,
    }
  }

  fn parse_rex_prefix(&mut self) -> &mut FetchedInstBuilder<'a> {
    let candidate = self.program[self.rip_offset];
    match candidate {
      REX...REX_WRXB => { self.rex_prefix = candidate; self.rip_offset += 1 },
      _ => (),
    }
    self
  }

  fn parse_opcode(&mut self) -> &mut FetchedInstBuilder<'a> {
    let candidate = self.program[self.rip_offset];
    let mut r: u8 = 0;
    {
      let extract_r = |opcode| { r = candidate.get_bits(0..3); Some(opcode) };
      let plus_r_opcode = || { Opcode::from_u8(candidate & 0xf8).and_then(extract_r) };
      self.opcode = Opcode::from_u8(candidate)
                            .or_else(plus_r_opcode)
                            .or_else(|| Some(Opcode::Invalid))
                            .unwrap();
    }
    self.r = r;
    self.rip_offset += 1;
    self
  }

  fn parse_modrm(&mut self) -> &mut FetchedInstBuilder<'a> {
    match self.opcode {
      Opcode::Add | Opcode::Inc | Opcode::MovToRm | Opcode::MovToReg |
      Opcode::MovRmImm32 | Opcode::MovRm8Imm8 => {
        self.mod_rm = Some(ModRm::new(self.program[self.rip_offset]));
        self.rip_offset += 1;
      }
      _ => (),
    }
    self
  }

  fn parse_sib(&mut self) -> &mut FetchedInstBuilder<'a> {
    match self.opcode {
      Opcode::MovRm8Imm8 => {
        self.sib = Some(Sib::new(self.program[self.rip_offset]));
        self.rip_offset += 1;
      }
      _ => (),
    }
    self
  }

  fn parse_disp(&mut self) -> &mut FetchedInstBuilder<'a> {
    match self.opcode {
      Opcode::JmpRel8 => {
        self.displacement = self.program[self.rip_offset] as u64;
        self.rip_offset += 1
      }
      Opcode::CallRel32 | Opcode::MovRm8Imm8 => {
        let mut disp = &self.program[self.rip_offset..self.rip_offset+4];
        self.displacement = sign_extend_from_u32(disp.read_u32::<LittleEndian>().unwrap());
        self.rip_offset += 4;
      }
      _ => (),
    }
    self
  }

  fn parse_imm(&mut self) -> &mut FetchedInstBuilder<'a> {
    match self.opcode {
      Opcode::MovImm32 | Opcode::MovRmImm32 => {
        let mut imm = &self.program[self.rip_offset..self.rip_offset+4];
        self.immediate = imm.read_u32::<LittleEndian>().unwrap().into();
        self.rip_offset += 4
      }
      Opcode::MovRm8Imm8 => {
        self.immediate = self.program[self.rip_offset] as u64;
        self.rip_offset += 1
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
      next_rip: self.rip_base + self.rip_offset,
    }
  }
}

fn zero_extend_from_u32(i: u32) -> u64 {
  i as u64
}

fn sign_extend_from_u32(i: u32) -> u64 {
  let i = i as i32;
  i as u64
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_sign_extend() {
    let i: u32 = 0xFFFF_FFFF;
    let sign_extend = sign_extend_from_u32(i);
    let zero_extend = zero_extend_from_u32(i);

    assert_eq!(sign_extend, 0xFFFF_FFFF_FFFF_FFFF);
    assert_eq!(zero_extend, 0x0000_0000_FFFF_FFFF);
  }
}
