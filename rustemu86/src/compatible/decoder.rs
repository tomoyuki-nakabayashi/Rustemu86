use compatible::Result;
use compatible::isa::opcode::OpcodeCompat;
use compatible::fetcher::FetchedInst;
use compatible::gpr::RegisterFile;
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

pub(super) fn decode(inst: &FetchedInst, gpr: &RegisterFile) -> Result<ExecuteInst> {
    match inst.get_opcode() {
        OpcodeCompat::Xor => {
            let decoded = decode_al_modrm(&inst, &gpr, Box::new(|a, b| a ^ b ));
            Ok(decoded)
        }
        OpcodeCompat::Hlt => {
            Ok(ExecuteInst::Privileged(PrivilegedInst{}))
        }
    }

}

fn decode_al_modrm(
    inst: &FetchedInst,
    gpr: &RegisterFile,
    expr: Box<dyn Fn(u64, u64) -> u64>) -> ExecuteInst
{
    let (reg, rm) = inst.get_modrm().get_reg_rm();
    let inst = ArithLogicInst{
        left: gpr.read_u64(reg),
        right: gpr.read_u64(rm),
        expr: expr,
    };
    ExecuteInst::ArithLogic(inst)
}

fn nop(_left: u64, _right: u64) -> u64 {
    0
}