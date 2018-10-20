use targets::x86::Result;
use targets::x86::decoder::ExecuteInst;
use targets::x86::status_regs::CpuState;
use targets::x86::gpr::Reg32;
use targets::x86::gpr::Reg32::*;

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