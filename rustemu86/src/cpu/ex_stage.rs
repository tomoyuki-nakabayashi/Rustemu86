use cpu::isa::registers::Reg64Id;
use cpu::decoder::DestType;
use cpu::decoder::ExStageInst;
use cpu::decoder::InstType;
use cpu::decoder::ExOpcode;

#[derive(Debug)]
pub struct WriteBackInst {
  pub dest_type: DestType,
  pub dest_rf: Reg64Id,
  pub result: u64,
}

impl WriteBackInst {
  fn new_invalid_inst() -> WriteBackInst {
    WriteBackInst {
      dest_type: DestType::Register,
      dest_rf: Reg64Id::Unknown,
      result: 0,
    }
  }

  fn new_dest_reg(dest: Reg64Id, result: u64) -> WriteBackInst {
    WriteBackInst {
      dest_type: DestType::Register,
      dest_rf: dest,
      result: result,
    }
  }
}

pub fn execute(inst: &Box<ExStageInst>) -> WriteBackInst {
  match inst.get_inst_type() {
    InstType::ArithLogic => execute_arith_logic(&inst),
    _ => WriteBackInst::new_invalid_inst(),  // WA
  }
}

fn execute_arith_logic(inst: &Box<ExStageInst>) -> WriteBackInst {
  match inst.get_ex_opcode().unwrap() {
    ExOpcode::Inc => execute_inc(&inst),
    ExOpcode::Add => execute_add(&inst),
    ExOpcode::Mov => execute_mov(&inst),
    _ => WriteBackInst::new_dest_reg(Reg64Id::Unknown, 0),  // WA
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
