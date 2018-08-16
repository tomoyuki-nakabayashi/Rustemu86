use cpu::isa::registers::Reg64Id;
use cpu::decoder::DestType;
use cpu::decoder::ExStageInst;
use cpu::decoder::InstType;
use cpu::decoder::ExOpcode;

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
      dest_type: DestType::Halted,
      dest_rf: Reg64Id::Unknown,
      addr: 0,
      result: 0,
    }
  }
}

pub fn execute(inst: &Box<ExStageInst>) -> WriteBackInst {
  match inst.get_inst_type() {
    InstType::ArithLogic => execute_arith_logic(&inst),
    InstType::Branch => execute_branch(&inst),
    InstType::LoadStore => execute_load_store(&inst),
  }
}

fn execute_arith_logic(inst: &Box<ExStageInst>) -> WriteBackInst {
  match inst.get_ex_opcode().unwrap() {
    ExOpcode::Inc => execute_inc(&inst),
    ExOpcode::Add => execute_add(&inst),
    ExOpcode::Mov => execute_mov(&inst),
    ExOpcode::Halt => WriteBackInst::new_dest_halt(),
    _ => WriteBackInst::new_invalid_inst(),
  }
}

fn execute_inc(inst: &Box<ExStageInst>) -> WriteBackInst {
  let result = inst.get_operand1() + 1;
  WriteBackInst::new_dest_reg(inst.get_dest_reg(), result)
}

fn execute_add(inst: &Box<ExStageInst>) -> WriteBackInst {
  let result = inst.get_operand1() + inst.get_operand2();
  WriteBackInst::new_dest_reg(inst.get_dest_reg(), result)
}

fn execute_mov(inst: &Box<ExStageInst>) -> WriteBackInst {
  let result = inst.get_operand1();
  WriteBackInst::new_dest_reg(inst.get_dest_reg(), result)
}

fn execute_branch(inst: &Box<ExStageInst>) -> WriteBackInst {
    match inst.get_ex_opcode().unwrap() {
    ExOpcode::Jump => execute_jump(&inst),
    _ => WriteBackInst::new_invalid_inst(),
  }
}

fn execute_jump(inst: &Box<ExStageInst>) -> WriteBackInst {
  let result = inst.get_operand1() + inst.get_operand2();
  WriteBackInst::new_dest_rip(result)
}

fn execute_load_store(inst: &Box<ExStageInst>) -> WriteBackInst {
  match inst.get_ex_opcode().unwrap() {
    ExOpcode::Load => execute_load(&inst),
    ExOpcode::Store => execute_store(&inst),
    _ => WriteBackInst::new_invalid_inst(),
  }
}

fn execute_load(inst: &Box<ExStageInst>) -> WriteBackInst {
  let addr = inst.get_operand1();
  WriteBackInst::new_dest_mem_to_reg(inst.get_dest_reg(), addr)
}

fn execute_store(inst: &Box<ExStageInst>) -> WriteBackInst {
  let addr = inst.get_operand1();
  let result = inst.get_operand3();
  WriteBackInst::new_dest_mem(addr, result)
}
