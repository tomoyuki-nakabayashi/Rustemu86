use compatible::{Result, CompatibleException};
use compatible::decoder::{ExecuteInst, ArithLogicInst};
use compatible::status_regs::CpuState;

pub(super) enum WriteBackType {
    Gpr(GprWriteBack),
    Status(StatusWriteBack),
}

pub struct GprWriteBack {
    pub(super) index: usize,
    pub(super) value: u64,
}

pub struct StatusWriteBack {
    pub(super) state: CpuState,
}

pub(super) fn execute(inst: ExecuteInst) -> Result<WriteBackType> {
    match inst {
        ExecuteInst::ArithLogic(inst) => {
            Ok( WriteBackType::Gpr(GprWriteBack { index: 0, value: inst.execute() }))
        }
        ExecuteInst::Privileged(inst) => {
            Ok( WriteBackType::Status( StatusWriteBack{ state: CpuState::Halted } ))
        }
    }
}