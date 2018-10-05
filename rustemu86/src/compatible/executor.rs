use compatible::Result;
use compatible::decoder::ExecuteInst;
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
    use self::ExecuteInst::{ArithLogic, Privileged};
    match inst {
        ArithLogic(inst) => {
            Ok( WriteBackType::Gpr(GprWriteBack { index: 0, value: inst.execute() }))
        }
        Privileged(_inst) => {
            Ok( WriteBackType::Status( StatusWriteBack{ state: CpuState::Halted } ))
        }
    }
}