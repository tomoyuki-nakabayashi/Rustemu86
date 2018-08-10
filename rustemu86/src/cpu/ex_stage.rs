use cpu::isa::registers::Reg64Id;
use cpu::decoder::DestType;
use cpu::decoder::ExStageInst;
use cpu::decoder::InstType;
use cpu::decoder::ExOpcode;

#[derive(Debug)]
pub struct WriteBackInst {
  pub dest_type: DestType,
  pub dest_rf: Reg64Id,
  pub dest_addr: u64,
  pub result: u64,
}

impl WriteBackInst {
  fn new_invalid_inst() -> WriteBackInst {
    WriteBackInst {
      dest_type: DestType::Register,
      dest_rf: Reg64Id::Unknown,
      dest_addr: 0,
      result: 0,
    }
  }

  fn new_dest_reg(dest: Reg64Id, result: u64) -> WriteBackInst {
    WriteBackInst {
      dest_type: DestType::Register,
      dest_rf: dest,
      dest_addr: 0,
      result: result,
    }
  }

  fn new_dest_rip(result: u64) -> WriteBackInst {
    WriteBackInst {
      dest_type: DestType::Rip,
      dest_rf: Reg64Id::Unknown,
      dest_addr: 0,
      result: result,
    }
  }

  fn new_dest_mem(addr: u64, result: u64) -> WriteBackInst {
    WriteBackInst {
      dest_type: DestType::Memory,
      dest_rf: Reg64Id::Unknown,
      dest_addr: addr,
      result: result,
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
    ExOpcode::Store => execute_store(&inst),
    _ => WriteBackInst::new_invalid_inst(),
  }
}

fn execute_store(inst: &Box<ExStageInst>) -> WriteBackInst {
  let addr = inst.get_operand1();
  let result = inst.get_operand3();
  WriteBackInst::new_dest_mem(addr, result)
}
