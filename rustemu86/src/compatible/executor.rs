use compatible::{Result, CompatibleException};
use compatible::decoder::{ExecuteInst, ArithLogicInst};

pub struct WriteBackPacket {
    usize: index,
    u64: value,
}

pub(super) fn execute(inst: ExecuteInst) -> Result<WriteBackPacket> {
    match inst {
        ExecuteInst::ArithLogic(inst) => {
            Ok(WriteBackPacket { index: 0, value: inst.execute() })
        }
    }
}