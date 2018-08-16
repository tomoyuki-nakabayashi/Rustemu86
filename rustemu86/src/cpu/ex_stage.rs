use cpu::isa::registers::Reg64Id;
use cpu::decoder::ExOpcode;
use cpu::decoder::ExecuteInstType;
use cpu::decoder::ExecuteInst;

#[derive(Debug, PartialEq)]
pub enum DestType {
  Register,
  Rip,
  CpuState,
  Memory,
  MemToReg,
}

/* 
pub enum DestinationType {
  Rip(u64),
  GeneralRegister(Reg64Id, u64),
  CpuState(CpuState),
}
 */

#[derive(Debug)]
pub struct WriteBackInst {
  pub dest_type: DestType,
  pub dest_rf: Reg64Id,
  pub addr: u64,
  pub result: u64,
}

impl WriteBackInst {
  fn new_invalid_inst() -> WriteBackInst {
    WriteBackInst {
      dest_type: DestType::Register,
      dest_rf: Reg64Id::Unknown,
      addr: 0,
      result: 0,
    }
  }

  fn new_dest_reg(dest: Reg64Id, result: u64) -> WriteBackInst {
    WriteBackInst {
      dest_type: DestType::Register,
      dest_rf: dest,
      addr: 0,
      result: result,
    }
  }

  fn new_dest_rip(result: u64) -> WriteBackInst {
    WriteBackInst {
      dest_type: DestType::Rip,
      dest_rf: Reg64Id::Unknown,
      addr: 0,
      result: result,
    }
  }

  fn new_dest_mem(addr: u64, result: u64) -> WriteBackInst {
    WriteBackInst {
      dest_type: DestType::Memory,
      dest_rf: Reg64Id::Unknown,
      addr: addr,
      result: result,
    }
  }

  fn new_dest_mem_to_reg(dest: Reg64Id, addr: u64) -> WriteBackInst {
    WriteBackInst {
      dest_type: DestType::MemToReg,
      dest_rf: dest,
      addr: addr,
      result: 0,
    }
  }

  fn new_dest_halt() -> WriteBackInst {
    WriteBackInst {
      dest_type: DestType::CpuState,
      dest_rf: Reg64Id::Unknown,
      addr: 0,
      result: 0,
    }
  }
}

pub fn execute_new(inst: ExecuteInstType) -> WriteBackInst {
  match inst {
    ExecuteInstType::ArithLogic(inst) => execute_arith_logic_new(inst),
    ExecuteInstType::Branch(inst) => execute_branch_new(inst),
    ExecuteInstType::LoadStore(inst) => execute_load_store_new(inst),
    ExecuteInstType::Privilege(inst) => execute_privilege(inst),
  }
}

fn execute_arith_logic_new(inst: ExecuteInst) -> WriteBackInst {
  match inst.get_opcode() {
    ExOpcode::Inc => execute_inc_new(inst),
    ExOpcode::Add => execute_add_new(inst),
    ExOpcode::Mov => execute_mov_new(inst),
    _ => WriteBackInst::new_invalid_inst(),
  }
}

fn execute_inc_new(inst: ExecuteInst) -> WriteBackInst {
  let result = inst.get_op1() + 1;
  let dest = inst.get_dest();
  WriteBackInst::new_dest_reg(dest, result)
}

fn execute_add_new(inst: ExecuteInst) -> WriteBackInst {
  let result = inst.get_op1() + inst.get_op2();
  let dest = inst.get_dest();
  WriteBackInst::new_dest_reg(dest, result)
}

fn execute_mov_new(inst: ExecuteInst) -> WriteBackInst {
  let result = inst.get_op1();
  let dest = inst.get_dest();
  WriteBackInst::new_dest_reg(dest, result)
}

fn execute_branch_new(inst: ExecuteInst) -> WriteBackInst {
    match inst.get_opcode() {
    ExOpcode::Jump => execute_jump_new(inst),
    _ => WriteBackInst::new_invalid_inst(),
  }
}

fn execute_jump_new(inst: ExecuteInst) -> WriteBackInst {
  let result = inst.get_rip() + inst.get_op1();
  WriteBackInst::new_dest_rip(result)
}

fn execute_load_store_new(inst: ExecuteInst) -> WriteBackInst {
  match inst.get_opcode() {
    ExOpcode::Load => execute_load_new(inst),
    ExOpcode::Store => execute_store_new(inst),
    _ => WriteBackInst::new_invalid_inst(),
  }
}

fn execute_load_new(inst: ExecuteInst) -> WriteBackInst {
  let addr = inst.get_op1();
  let dest = inst.get_dest();
  WriteBackInst::new_dest_mem_to_reg(dest, addr)
}

fn execute_store_new(inst: ExecuteInst) -> WriteBackInst {
  let addr = inst.get_op1();
  let data = inst.get_op2();
  WriteBackInst::new_dest_mem(addr, data)
}

fn execute_privilege(inst: ExecuteInst) -> WriteBackInst {
  match inst.get_opcode() {
    ExOpcode::Halt => WriteBackInst::new_dest_halt(),
    _ => WriteBackInst::new_invalid_inst(),
  }
}
