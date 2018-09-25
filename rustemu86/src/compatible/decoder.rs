use compatible::{Result, CompatibleException};
use compatible::isa::opcode::OpcodeCompat;
use compatible::fetcher::FetchedInst;

pub(crate) enum ExecuteInst {
    ArithLogic(ArithLogicInst),
    Privileged(PrivilegedInst),
}

pub(crate) struct ArithLogicInst {
    expr: Box<dyn Fn(u64, u64) -> u64>,
}

impl ArithLogicInst {
    pub(crate) fn execute(&self) -> u64 {
        (self.expr)(0, 0)
    }
}

pub(crate) struct PrivilegedInst {}

pub(super) fn decode(inst: FetchedInst) -> Result<ExecuteInst> {
    match inst.get_opcode() {
        OpcodeCompat::Xor => {
            Ok(ExecuteInst::ArithLogic( ArithLogicInst { expr: Box::new(|a, b| a ^ b ) }))
        }
        OpcodeCompat::Hlt => {
            Ok(ExecuteInst::Privileged(PrivilegedInst{}))
        }
    }

}