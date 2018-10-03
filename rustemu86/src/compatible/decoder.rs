use compatible::{Result, CompatibleException};
use compatible::isa::opcode::OpcodeCompat;
use compatible::fetcher::FetchedInst;
use std::default::Default;

pub(crate) enum ExecuteInst {
    ArithLogic(ArithLogicInst),
    Privileged(PrivilegedInst),
}

pub(crate) struct ArithLogicInst {
    left: u64,
    right: u64,
    expr: Box<dyn Fn(u64, u64) -> u64>,
}

impl ArithLogicInst {
    pub(crate) fn execute(&self) -> u64 {
        (self.expr)(0, 0)
    }
}

impl Default for ArithLogicInst {
    fn default() -> ArithLogicInst {
        ArithLogicInst {
            left: 0,
            right: 0,
            expr: Box::new(nop),
        }
    }
}

pub(crate) struct PrivilegedInst {}

pub(super) fn decode(inst: FetchedInst) -> Result<ExecuteInst> {
    match inst.get_opcode() {
        OpcodeCompat::Xor => {
            let inst = ArithLogicInst{expr: Box::new(|a, b| a ^ b ), ..Default::default()};
            Ok(ExecuteInst::ArithLogic(inst))
        }
        OpcodeCompat::Hlt => {
            Ok(ExecuteInst::Privileged(PrivilegedInst{}))
        }
    }

}

fn nop(_left: u64, _right: u64) -> u64 {
    0
}