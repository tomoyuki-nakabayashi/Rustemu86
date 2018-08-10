use cpu::isa::registers::Reg64Id;
use cpu::isa::opcode::Opcode;
use cpu::register_file::RegisterFile;
use cpu::fetcher::FetchedInst;
use cpu::exceptions::InternalException;
use num::FromPrimitive;

pub trait ExStageInst {
  fn get_inst_type(&self) -> InstType;
  fn get_ex_opcode(&self) -> Option<ExOpcode>;
  fn get_dest_reg(&self) -> Reg64Id;
  fn get_operand1(&self) -> u64;
  fn get_operand2(&self) -> u64;
  fn get_operand3(&self) -> u64;
}

pub struct ArithLogicInst {
  inst_type: InstType,
  opcode: ExOpcode,
  dest: Reg64Id,
  operand1: u64,
  operand2: u64,
  operand3: u64,
}

impl ArithLogicInst {
  fn new(op: ExOpcode, dest: Reg64Id, op1: u64, op2: u64, op3: u64) -> ArithLogicInst {
    ArithLogicInst {
      inst_type: InstType::ArithLogic,
      opcode: op,
      dest: dest,
      operand1: op1,
      operand2: op2,
      operand3: op3,
    }
  }
}

impl ExStageInst for ArithLogicInst {
  fn get_inst_type(&self) -> InstType { self.inst_type }
  fn get_ex_opcode(&self) -> Option<ExOpcode> { Some(self.opcode) }
  fn get_dest_reg(&self) -> Reg64Id { self.dest }
  fn get_operand1(&self) -> u64 { self.operand1 }
  fn get_operand2(&self) -> u64 { self.operand2 }
  fn get_operand3(&self) -> u64 { self.operand3 }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum InstType {
  ArithLogic,
  Branch,
  MemoryAccess,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ExOpcode {
  Add,
  Inc,
}

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

pub fn new_decode(rip: u64, rf: &RegisterFile, inst: &FetchedInst) -> Result<Box<ExStageInst>, InternalException> {
  match inst.opcode {
    Opcode::Inc => Ok(decode_inc_new(&rf, &inst)),
    Opcode::Add => Ok(decode_add_new(&rf, &inst)),
    opcode @ _ => Err(InternalException::UndefinedInstruction {opcode}),
  }
}

pub fn decode(rip: u64, rf: &RegisterFile, inst: &FetchedInst) -> Result<DecodedInst, InternalException> {
  match inst.opcode {
    Opcode::Add => Ok(decode_add(&rf, &inst)),
    Opcode::MovToRm => Ok(decode_store(&rf, &inst)),
//    Opcode::MovToReg => Ok(decode_load(&rf, &inst)),
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
/* 
fn decode_load(rf: &RegisterFile, inst: &FetchedInst) -> DecodedInst {
  let dest = inst.mod_rm.reg;
  let src = inst.mod_rm.rm;
  let result_value = rf.read64(src);
  DecodedInst::new(DestType::Memory, dest, result_value)
}
 */
fn decode_mov(inst: &FetchedInst) -> DecodedInst {
  let dest = Reg64Id::from_u8(inst.r).unwrap();
  DecodedInst::new(DestType::Register, dest, inst.immediate)
}

fn decode_inc_new(rf: &RegisterFile, inst: &FetchedInst) -> Box<ExStageInst> {
  let dest = inst.mod_rm.rm;
  let op1 = rf.read64(dest);
  Box::new(ArithLogicInst::new(ExOpcode::Inc, dest, op1, 0, 0)) as Box<ExStageInst>
}

fn decode_inc(rf: &RegisterFile, inst: &FetchedInst) -> DecodedInst {
  let dest = inst.mod_rm.rm;
  let incremented_value = rf.read64(dest) + 1;
  DecodedInst::new(DestType::Register, dest, incremented_value)
}

fn decode_add_new(rf: &RegisterFile, inst: &FetchedInst) -> Box<ExStageInst> {
  let dest = inst.mod_rm.rm;
  let src = inst.mod_rm.reg;
  let op1 = rf.read64(dest);
  let op2 = rf.read64(src);
  Box::new(ArithLogicInst::new(ExOpcode::Add, dest, op1, op2, 0)) as Box<ExStageInst>
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