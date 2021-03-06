use crate::decoder::ExecuteInst;
use crate::gpr::{Reg32, SegReg};
use crate::isa::eflags::EFlags;
use crate::status_regs::CpuState;
use crate::Result;

pub trait Execute {
    type ResultValue;
    fn execute(&self) -> Self::ResultValue;
}

pub enum WriteBackType {
    Gpr(GprWriteBack),
    Store(StoreWriteBack),
    Segment(SegmentWriteBack),
    EFlags(EFlagsWriteBack),
    Status(StatusWriteBack),
}

pub struct GprWriteBack {
    pub(super) index: Reg32,
    pub(super) value: u64,
}

pub struct StoreWriteBack {
    pub(super) index: usize,
    pub(super) value: u64,
}

pub struct SegmentWriteBack {
    pub(super) index: SegReg,
    pub(super) value: u64,
}

pub struct EFlagsWriteBack {
    pub(super) target: EFlags,
    pub(super) value: bool,
}

pub struct StatusWriteBack {
    pub(super) state: CpuState,
}

pub(super) fn execute(inst: &ExecuteInst) -> Result<WriteBackType> {
    use self::ExecuteInst::{ArithLogic, Privileged, Segment, StatusOp, Store};
    match inst {
        ArithLogic(inst) => {
            let (target, value) = inst.execute();
            Ok(WriteBackType::Gpr(GprWriteBack {
                index: target,
                value,
            }))
        }
        Store(inst) => {
            let (target, value) = inst.execute();
            Ok(WriteBackType::Store(StoreWriteBack {
                index: target,
                value,
            }))
        }
        Segment(inst) => {
            let (target, value) = inst.execute();
            Ok(WriteBackType::Segment(SegmentWriteBack {
                index: target,
                value,
            }))
        }
        StatusOp(inst) => {
            let (target, value) = inst.execute();
            Ok(WriteBackType::EFlags(EFlagsWriteBack {
                target,
                value,
            }))
        }
        Privileged(inst) => Ok(WriteBackType::Status(StatusWriteBack {
            state: inst.execute(),
        })),
    }
}
