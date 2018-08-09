use cpu::isa::registers::Reg64Id;
use cpu::isa::opcode::Opcode;
use cpu::register_file::RegisterFile;
use cpu::fetcher::FetchedInst;
use cpu::exceptions::InternalException;
use num::FromPrimitive;

#[derive(Debug, PartialEq)]
pub enum DestType {
  Register,
  Rip,
  Memory,
}

#[derive(Debug)]
pub struct DecodedInst {
  pub dest_type: DestType,
  pub dest_rf: Reg64Id,
  pub result: u64,
}

impl DecodedInst {
  pub fn new(dest_type: DestType, rf: Reg64Id, result: u64) -> DecodedInst {
    DecodedInst {
      dest_type: dest_type,
      dest_rf: rf,
      result: result,
    }
  }
}

pub fn decode(rip: u64, rf: &RegisterFile, inst: &FetchedInst) -> Result<DecodedInst, InternalException> {
  match inst.opcode {
    Opcode::Add => Ok(decode_add(&rf, &inst)),
    Opcode::MovToRm => Ok(decode_store(&rf, &inst)),
    Opcode::MovImm32 => Ok(decode_mov(&inst)),
    Opcode::JmpRel8 => Ok(decode_jmp(rip, &inst)),
    Opcode::Inc => Ok(decode_inc(&rf, &inst)),
    opcode @ _ => Err(InternalException::UndefinedInstruction {opcode}),
  }
}

fn decode_store(rf: &RegisterFile, inst: &FetchedInst) -> DecodedInst {
  let dest = inst.mod_rm.rm;
  let src = inst.mod_rm.reg;
  let result_value = rf.read64(src);
  DecodedInst::new(DestType::Memory, dest, result_value)
}

fn decode_mov(inst: &FetchedInst) -> DecodedInst {
  let dest = Reg64Id::from_u8(inst.r).unwrap();
  DecodedInst::new(DestType::Register, dest, inst.immediate)
}

fn decode_inc(rf: &RegisterFile, inst: &FetchedInst) -> DecodedInst {
  let dest = inst.mod_rm.rm;
  let incremented_value = rf.read64(dest) + 1;
  DecodedInst::new(DestType::Register, dest, incremented_value)
}

fn decode_add(rf: &RegisterFile, inst: &FetchedInst) -> DecodedInst {
  let dest = inst.mod_rm.rm;
  let src = inst.mod_rm.reg;
  let result_value = rf.read64(dest) + rf.read64(src);
  DecodedInst::new(DestType::Register, dest, result_value)
}

fn decode_jmp(rip: u64, inst: &FetchedInst) -> DecodedInst {
  let disp = inst.displacement;
  let rip = rip + disp as u64;

  DecodedInst::new(DestType::Rip, Reg64Id::Unknown, rip)
}