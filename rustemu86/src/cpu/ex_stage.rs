use cpu::isa::registers::Reg64Id;
use cpu::decoder::DestType;
use cpu::decoder::ExStageInst;

#[derive(Debug)]
pub struct WriteBackInst {
  pub dest_type: DestType,
  pub dest_rf: Reg64Id,
  pub result: u64,
}

pub fn execute(inst: &Box<ExStageInst>) -> WriteBackInst {
  WriteBackInst {
    dest_type: DestType::Register,
    dest_rf: Reg64Id::Rax,
    result: 2,
  }
}