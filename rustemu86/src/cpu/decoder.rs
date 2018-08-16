use cpu::isa::registers::Reg64Id;
use cpu::isa::opcode::Opcode;
use cpu::register_file::RegisterFile;
use cpu::fetcher::FetchedInst;
use cpu::exceptions::InternalException;
use num::FromPrimitive;

pub enum ExecuteInstType {
  ArithLogic(ExecuteInst),
  Branch(ExecuteInst),
  LoadStore(ExecuteInst),
  Privilege(ExecuteInst),
}

pub struct ExecuteInst {
  opcode: ExOpcode,
  dest: Option<Reg64Id>,
  rip: Option<u64>,
  op1: Option<u64>,
  op2: Option<u64>,
  op3: Option<u64>,
  op4: Option<u64>,
}

impl ExecuteInst {
  pub fn get_opcode(&self) -> ExOpcode { self.opcode }
  pub fn get_dest(&self) -> Reg64Id {
    self.dest.expect("Destination register was not decoded.")
  }
  pub fn get_rip(&self) -> u64 { self.rip.expect("Rip was not decoded.") }
  pub fn get_op1(&self) -> u64 { self.op1.expect("Operand1 was not decoded.") }
  pub fn get_op2(&self) -> u64 { self.op2.expect("Operand2 was not decoded.") }
  pub fn get_op3(&self) -> u64 { self.op3.expect("Operand3 was not decoded.") }
  pub fn get_op4(&self) -> u64 { self.op4.expect("Operand4 was not decoded.") }
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

pub fn decode_new(rf: &RegisterFile, inst: &FetchedInst) -> Result<ExecuteInstType, InternalException> {
  use self::ExecuteInstType::*;
  match inst.opcode {
    Opcode::Inc => Ok(ArithLogic(decode_inc_new(&rf, &inst))),
    Opcode::Add => Ok(ArithLogic(decode_add_new(&rf, &inst))),
    Opcode::MovToReg => Ok(LoadStore(decode_load_new(&rf, &inst))),
    Opcode::MovToRm => Ok(LoadStore(decode_store_new(&rf, &inst))),
    Opcode::MovImm32 => Ok(ArithLogic(decode_reg_mov_new(&inst))),
    Opcode::JmpRel8 => Ok(Branch(decode_jmp_new(&inst))),
    Opcode::Halt => Ok(Privilege(decode_halt(&inst))),
    opcode @ _ => Err(InternalException::UndefinedInstruction {opcode}),
  }
}


fn decode_inc_new(rf: &RegisterFile, inst: &FetchedInst) -> ExecuteInst {
  let dest = inst.mod_rm.rm;
  let op1 = rf.read64(dest);
  ExecuteInst { opcode: ExOpcode::Inc, dest: Some(dest), rip: None,
    op1: Some(op1), op2: None, op3: None, op4: None }
}

fn decode_add_new(rf: &RegisterFile, inst: &FetchedInst) -> ExecuteInst {
  let dest = inst.mod_rm.rm;
  let src = inst.mod_rm.reg;
  let op1 = rf.read64(dest);
  let op2 = rf.read64(src);
  ExecuteInst { opcode: ExOpcode::Add, dest: Some(dest), rip: None,
    op1: Some(op1), op2: Some(op2), op3: None, op4: None }
}

fn decode_reg_mov_new(inst: &FetchedInst) -> ExecuteInst {
  let dest = Reg64Id::from_u8(inst.r).unwrap();
  let op1 = inst.immediate;
  ExecuteInst { opcode: ExOpcode::Mov, dest: Some(dest), rip: None, 
    op1: Some(op1), op2: None, op3: None, op4: None }
}

fn decode_load_new(rf: &RegisterFile, inst: &FetchedInst) -> ExecuteInst {
  let dest = inst.mod_rm.reg;
  let addr = rf.read64(inst.mod_rm.rm);
  ExecuteInst { opcode: ExOpcode::Load, dest: Some(dest), rip: None,
    op1: Some(addr), op2: None, op3: None, op4: None }
}

fn decode_store_new(rf: &RegisterFile, inst: &FetchedInst) -> ExecuteInst {
  let addr = rf.read64(inst.mod_rm.rm);
  let result = rf.read64(inst.mod_rm.reg);
  ExecuteInst { opcode: ExOpcode::Store, dest: None, rip: None,
    op1: Some(addr), op2: Some(result), op3: None, op4: None }
}

fn decode_jmp_new(inst: &FetchedInst) -> ExecuteInst {
  let disp = inst.displacement;
  let rip = inst.next_rip as u64;
  ExecuteInst { opcode: ExOpcode::Jump, dest: None, rip: Some(rip),
    op1: Some(disp), op2: None, op3: None, op4: None }
}

fn decode_halt(_inst: &FetchedInst) -> ExecuteInst {
  ExecuteInst { opcode: ExOpcode::Halt, dest: None, rip: None,
    op1: None, op2: None, op3: None, op4: None }
}
