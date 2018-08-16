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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum InstType {
  ArithLogic,
  Branch,
  LoadStore,
}

struct NopInst;
impl ExStageInst for NopInst {
  fn get_inst_type(&self) -> InstType { InstType::ArithLogic }
  fn get_ex_opcode(&self) -> Option<ExOpcode> { Some(ExOpcode::Halt) }
  fn get_dest_reg(&self) -> Reg64Id { Reg64Id::Unknown }
  fn get_operand1(&self) -> u64 { 0 }
  fn get_operand2(&self) -> u64 { 0 }
  fn get_operand3(&self) -> u64 { 0 }
}

struct ArithLogicInst {
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

struct BranchInst {
  inst_type: InstType,
  opcode: ExOpcode,
  rip: u64,
  displacement: u64,
}

impl BranchInst {
  fn new(op: ExOpcode, rip: u64, disp: u64) -> BranchInst {
    BranchInst {
      inst_type: InstType::Branch,
      opcode: op,
      rip: rip,
      displacement: disp,
    }
  }
}

impl ExStageInst for BranchInst {
  fn get_inst_type(&self) -> InstType { self.inst_type }
  fn get_ex_opcode(&self) -> Option<ExOpcode> { Some(self.opcode) }
  fn get_dest_reg(&self) -> Reg64Id { Reg64Id::Unknown }
  fn get_operand1(&self) -> u64 { self.rip }
  fn get_operand2(&self) -> u64 { self.displacement }
  fn get_operand3(&self) -> u64 { 0 }
}

struct LoadStoreInst {
  inst_type: InstType,
  opcode: ExOpcode,
  dest: Reg64Id,
  addr: u64,
  displacement: u64,
  result: u64,
}

impl LoadStoreInst {
  fn new_store(op: ExOpcode, addr: u64, disp: u64, result: u64) -> LoadStoreInst {
    LoadStoreInst {
      inst_type: InstType::LoadStore,
      opcode: op,
      dest: Reg64Id::Unknown,
      addr: addr,
      displacement: disp,
      result: result,
    }
  }

  fn new_load(op: ExOpcode, dest: Reg64Id, addr: u64, disp: u64) -> LoadStoreInst {
    LoadStoreInst {
      inst_type: InstType::LoadStore,
      opcode: op,
      dest: dest,
      addr: addr,
      displacement: disp,
      result: 0,
    }
  }
}

impl ExStageInst for LoadStoreInst {
  fn get_inst_type(&self) -> InstType { self.inst_type }
  fn get_ex_opcode(&self) -> Option<ExOpcode> { Some(self.opcode) }
  fn get_dest_reg(&self) -> Reg64Id { self.dest }
  fn get_operand1(&self) -> u64 { self.addr }
  fn get_operand2(&self) -> u64 { self.displacement }
  fn get_operand3(&self) -> u64 { self.result }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ExOpcode {
  Add,
  Inc,
  Mov,
  Jump,
  Load,
  Store,
  Halt,
}

#[derive(Debug, PartialEq)]
pub enum DestType {
  Register,
  Rip,
  Halted,
  Memory,
  MemToReg,
}

pub fn decode(rf: &RegisterFile, inst: &FetchedInst) -> Result<Box<ExStageInst>, InternalException> {
  match inst.opcode {
    Opcode::Inc => Ok(decode_inc(&rf, &inst)),
    Opcode::Add => Ok(decode_add(&rf, &inst)),
    Opcode::MovToReg => Ok(decode_load(&rf, &inst)),
    Opcode::MovToRm => Ok(decode_store(&rf, &inst)),
    Opcode::MovImm32 => Ok(decode_reg_mov(&inst)),
    Opcode::JmpRel8 => Ok(decode_jmp(&inst)),
    Opcode::Halt => Ok(Box::new(NopInst)),
    opcode @ _ => Err(InternalException::UndefinedInstruction {opcode}),
  }
}

fn decode_store(rf: &RegisterFile, inst: &FetchedInst) -> Box<ExStageInst> {
  let addr = rf.read64(inst.mod_rm.rm);
  let result = rf.read64(inst.mod_rm.reg);
  Box::new(LoadStoreInst::new_store(ExOpcode::Store, addr, 0, result)) as Box<ExStageInst>
}

fn decode_load(rf: &RegisterFile, inst: &FetchedInst) -> Box<ExStageInst> {
  let dest = inst.mod_rm.reg;
  let addr = rf.read64(inst.mod_rm.rm);
  Box::new(LoadStoreInst::new_load(ExOpcode::Load, dest, addr, 0)) as Box<ExStageInst>
}

fn decode_reg_mov(inst: &FetchedInst) -> Box<ExStageInst> {
  let dest = Reg64Id::from_u8(inst.r).unwrap();
  let op1 = inst.immediate;
  Box::new(ArithLogicInst::new(ExOpcode::Mov, dest, op1, 0, 0)) as Box<ExStageInst>
}

fn decode_inc(rf: &RegisterFile, inst: &FetchedInst) -> Box<ExStageInst> {
  let dest = inst.mod_rm.rm;
  let op1 = rf.read64(dest);
  Box::new(ArithLogicInst::new(ExOpcode::Inc, dest, op1, 0, 0)) as Box<ExStageInst>
}

fn decode_add(rf: &RegisterFile, inst: &FetchedInst) -> Box<ExStageInst> {
  let dest = inst.mod_rm.rm;
  let src = inst.mod_rm.reg;
  let op1 = rf.read64(dest);
  let op2 = rf.read64(src);
  Box::new(ArithLogicInst::new(ExOpcode::Add, dest, op1, op2, 0)) as Box<ExStageInst>
}

fn decode_jmp(inst: &FetchedInst) -> Box<ExStageInst> {
  let disp = inst.displacement;
  let rip = inst.next_rip as u64;

  Box::new(BranchInst::new(ExOpcode::Jump, rip, disp)) as Box<ExStageInst>
}
