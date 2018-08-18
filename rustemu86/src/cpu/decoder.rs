use cpu::isa::registers::Reg64Id;
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
  Return,
  Load,
  Store,
  Halt,
}

pub fn decode(rf: &RegisterFile, inst: &FetchedInst) -> Result<Vec<ExecuteInstType>, InternalException> {
  use cpu::isa::opcode::Opcode::*;
  match inst.opcode {
    Add =>  Ok(decode_add(&rf, &inst)),
    CallRel32 => Ok(decode_call(&rf, &inst)),
    Halt => Ok(decode_halt(&inst)),
    Inc => Ok(decode_inc(&rf, &inst)),
    JmpRel8 => Ok(decode_jmp(&inst)),
    MovToReg => Ok(decode_load(&rf, &inst)),
    MovToRm => Ok(decode_store(&rf, &inst)),
    MovImm32 => Ok(decode_reg_mov(&inst)),
    MovRmImm32 => Ok(decode_mov_rm_imm(&inst)),
    PushR => Ok(decode_pushr(&rf, &inst)),
    PopR => Ok(decode_popr(&rf, &inst)),
    Ret => Ok(decode_ret(&rf, &inst)),
    opcode @ _ => Err(InternalException::UndefinedInstruction {opcode}),
  }
}

/////////////////////////////////////////////////////////////////////////////
// Arithmetic and Logic instructions.
/////////////////////////////////////////////////////////////////////////////
fn decode_inc(rf: &RegisterFile, inst: &FetchedInst) -> Vec<ExecuteInstType> {
  let dest = inst.mod_rm.rm;
  let op1 = rf.read64(dest);
  let uop1 = ExecuteInst { opcode: ExOpcode::Inc, dest: Some(dest), rip: None,
    op1: Some(op1), op2: None, op3: None, op4: None };
  vec![ExecuteInstType::ArithLogic(uop1)]
}

fn decode_add(rf: &RegisterFile, inst: &FetchedInst) -> Vec<ExecuteInstType> {
  let dest = inst.mod_rm.rm;
  let src = inst.mod_rm.reg;
  let op1 = rf.read64(dest);
  let op2 = rf.read64(src);
  let uop1 = ExecuteInst { opcode: ExOpcode::Add, dest: Some(dest), rip: None,
    op1: Some(op1), op2: Some(op2), op3: None, op4: None };
  vec![ExecuteInstType::ArithLogic(uop1)]
}

fn decode_reg_mov(inst: &FetchedInst) -> Vec<ExecuteInstType> {
  let dest = Reg64Id::from_u8(inst.r).unwrap();
  let op1 = inst.immediate;
  let mov = ExecuteInst { opcode: ExOpcode::Mov, dest: Some(dest), rip: None, 
    op1: Some(op1), op2: None, op3: None, op4: None };
  vec![ExecuteInstType::ArithLogic(mov)]
}

fn decode_mov_rm_imm(inst: &FetchedInst) -> Vec<ExecuteInstType> {
  let dest = inst.mod_rm.rm;
  let imm = inst.immediate;
  let uop1 = ExecuteInst { opcode: ExOpcode::Mov, dest: Some(dest), rip: None, 
    op1: Some(imm), op2: None, op3: None, op4: None };
  vec![ExecuteInstType::ArithLogic(uop1)]
}

/////////////////////////////////////////////////////////////////////////////
// Load and Store instructions.
/////////////////////////////////////////////////////////////////////////////
fn decode_load(rf: &RegisterFile, inst: &FetchedInst) -> Vec<ExecuteInstType> {
  let dest = inst.mod_rm.reg;
  let addr = rf.read64(inst.mod_rm.rm);
  let load = ExecuteInst { opcode: ExOpcode::Load, dest: Some(dest), rip: None,
    op1: Some(addr), op2: None, op3: None, op4: None };
  vec![ExecuteInstType::LoadStore(load)]
}

fn decode_store(rf: &RegisterFile, inst: &FetchedInst) -> Vec<ExecuteInstType> {
  let addr = rf.read64(inst.mod_rm.rm);
  let result = rf.read64(inst.mod_rm.reg);
  let store = ExecuteInst { opcode: ExOpcode::Store, dest: None, rip: None,
    op1: Some(addr), op2: Some(result), op3: None, op4: None };
  vec![ExecuteInstType::LoadStore(store)]
}

/////////////////////////////////////////////////////////////////////////////
// Branch instructions.
/////////////////////////////////////////////////////////////////////////////
fn decode_jmp(inst: &FetchedInst) -> Vec<ExecuteInstType> {
  let disp = inst.displacement;
  let rip = inst.next_rip as u64;
  let jmp = ExecuteInst { opcode: ExOpcode::Jump, dest: None, rip: Some(rip),
    op1: Some(disp), op2: None, op3: None, op4: None };
  vec![ExecuteInstType::Branch(jmp)]
}

/////////////////////////////////////////////////////////////////////////////
// Privileged instructions.
/////////////////////////////////////////////////////////////////////////////
fn decode_halt(_inst: &FetchedInst) -> Vec<ExecuteInstType> {
  let hlt = ExecuteInst { opcode: ExOpcode::Halt, dest: None, rip: None,
    op1: None, op2: None, op3: None, op4: None };
  vec![ExecuteInstType::Privilege(hlt)]
}

/////////////////////////////////////////////////////////////////////////////
// Complex instructions that require plural micro operations.
/////////////////////////////////////////////////////////////////////////////
fn decode_call(rf: &RegisterFile, inst: &FetchedInst) -> Vec<ExecuteInstType> {
  let new_sp = rf.read64(Reg64Id::Rsp) - 8;
  let update_sp = ExecuteInst { opcode: ExOpcode::Mov, dest: Some(Reg64Id::Rsp), rip: None,
    op1: Some(new_sp), op2: None, op3: None, op4: None };

  let ret_addr = inst.next_rip as u64;
  let push = ExecuteInst { opcode: ExOpcode::Store, dest: None, rip: None,
    op1: Some(new_sp), op2: Some(ret_addr), op3: None, op4: None };

  let disp = inst.displacement;
  let rip = inst.next_rip as u64;
  let call = ExecuteInst { opcode: ExOpcode::Jump, dest: None, rip: Some(rip),
    op1: Some(disp), op2: None, op3: None, op4: None };

  vec![ExecuteInstType::ArithLogic(update_sp), ExecuteInstType::LoadStore(push),
      ExecuteInstType::Branch(call)]
}

fn decode_ret(rf: &RegisterFile, _inst: &FetchedInst) -> Vec<ExecuteInstType> {
  let sp = rf.read64(Reg64Id::Rsp);
  let ret = ExecuteInst { opcode: ExOpcode::Return, dest: None, rip: None,
    op1: Some(sp), op2: None, op3: None, op4: None };

  let new_sp = rf.read64(Reg64Id::Rsp) + 8;
  let update_sp = ExecuteInst { opcode: ExOpcode::Mov, dest: Some(Reg64Id::Rsp), rip: None,
    op1: Some(new_sp), op2: None, op3: None, op4: None };

  vec![ExecuteInstType::Branch(ret), ExecuteInstType::ArithLogic(update_sp)]
}

fn decode_pushr(rf: &RegisterFile, inst: &FetchedInst) -> Vec<ExecuteInstType> {
  let new_sp = rf.read64(Reg64Id::Rsp) - 8;
  let update_sp = ExecuteInst { opcode: ExOpcode::Mov, dest: Some(Reg64Id::Rsp), rip: None,
    op1: Some(new_sp), op2: None, op3: None, op4: None };

  let data = rf.read64(Reg64Id::from_u8(inst.r).unwrap());
  let push = ExecuteInst { opcode: ExOpcode::Store, dest: None, rip: None,
    op1: Some(new_sp), op2: Some(data), op3: None, op4: None };

  vec![ExecuteInstType::ArithLogic(update_sp), ExecuteInstType::LoadStore(push)]
}

fn decode_popr(rf: &RegisterFile, inst: &FetchedInst) -> Vec<ExecuteInstType> {
  let dest = Reg64Id::from_u8(inst.r).unwrap();
  let sp = rf.read64(Reg64Id::Rsp);
  let pop = ExecuteInst { opcode: ExOpcode::Load, dest: Some(dest), rip: None,
    op1: Some(sp), op2: None, op3: None, op4: None };

  let new_sp = rf.read64(Reg64Id::Rsp) + 8;
  let update_sp = ExecuteInst { opcode: ExOpcode::Mov, dest: Some(Reg64Id::Rsp), rip: None,
    op1: Some(new_sp), op2: None, op3: None, op4: None };

  vec![ExecuteInstType::LoadStore(pop), ExecuteInstType::ArithLogic(update_sp)]
}