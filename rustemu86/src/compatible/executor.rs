use compatible::Result;
use compatible::decoder::ExecuteInst;
use compatible::status_regs::CpuState;
use compatible::gpr::Reg32;
use compatible::gpr::Reg32::*;

pub trait Execute {
    type ResultValue;
    fn execute(&self) -> Self::ResultValue;
}

pub enum WriteBackType {
    Gpr(GprWriteBack),
    Status(StatusWriteBack),
}

pub struct GprWriteBack {
    pub(super) index: Reg32,
    pub(super) value: u64,
}

pub struct StatusWriteBack {
    pub(super) state: CpuState,
}

pub(super) fn execute(inst: &ExecuteInst) -> Result<WriteBackType> {
    use self::ExecuteInst::{ArithLogic, Privileged};
    match inst {
        ArithLogic(inst) => {
            Ok( WriteBackType::Gpr(GprWriteBack { index: Eax, value: inst.execute() }))
        }
        Privileged(inst) => {
            Ok( WriteBackType::Status( StatusWriteBack{ state: inst.execute() } ))
        }
    }
}