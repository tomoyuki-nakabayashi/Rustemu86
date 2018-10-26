use targets::x86::Result;
use targets::x86::isa::opcode::OpcodeCompat;
use targets::x86::fetcher::FetchedInst;
use targets::x86::gpr::RegisterFile;
use targets::x86::status_regs::CpuState;
use targets::x86::executor::Execute;
use std::default::Default;

pub enum ExecuteInst {
    ArithLogic(ArithLogicInst),
    Privileged(PrivilegedInst),
}

pub struct ArithLogicInst {
    left: u64,
    right: u64,
    expr: Box<dyn Fn(u64, u64) -> u64>,
}

impl Execute for ArithLogicInst {
    type ResultValue = u64;
    fn execute(&self) -> Self::ResultValue {
        (self.expr)(self.right, self.left)
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

pub struct PrivilegedInst {}

impl Execute for PrivilegedInst {
    type ResultValue = CpuState;
    fn execute(&self) -> Self::ResultValue {
        CpuState::Halted
    }
}

pub(super) fn decode(inst: &FetchedInst, gpr: &RegisterFile) -> Result<ExecuteInst> {
    match inst.get_opcode() {
        OpcodeCompat::MovRmSreg => {
            let decoded = decode_al_modrm(&inst, &gpr, Box::new(|_, b| b ));
            Ok(decoded)
        }
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