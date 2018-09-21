use compatible::{Result, CompatibleException};
use compatible::decoder::{ExecuteInst, ArithLogicInst};

pub struct WriteBackPacket {
    pub(super) index: usize,
    pub(super) value: u64,
}

pub(super) fn execute(inst: ExecuteInst) -> Result<WriteBackPacket> {
    match inst {
        ExecuteInst::ArithLogic(inst) => {
            Ok(WriteBackPacket { index: 0, value: inst.execute() })
        }
    }
}