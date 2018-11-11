use targets::x86::executor::Execute;
use targets::x86::fetcher::FetchedInst;
use targets::x86::gpr::{Reg32, RegisterFile, SegReg};
use targets::x86::isa::eflags::EFlags;
use targets::x86::isa::modrm::ModRm;
use targets::x86::isa::opcode::Opcode;
use targets::x86::status_regs::CpuState;
use targets::x86::Result;

pub enum ExecuteInst {
    ArithLogic(ArithLogicInst),
    Segment(SegmentInst),
    StatusOp(StatusOpInst),
    Privileged(PrivilegedInst),
}

pub struct ArithLogicInst {
    target: Reg32,
    left: u64,
    right: u64,
    expr: Box<dyn Fn(u64, u64) -> u64>,
}

impl Execute for ArithLogicInst {
    type ResultValue = (Reg32, u64);
    fn execute(&self) -> Self::ResultValue {
        (self.target, (self.expr)(self.left, self.right))
    }
}

pub struct SegmentInst {
    target: SegReg,
    left: u64,
    right: u64,
    expr: Box<dyn Fn(u64, u64) -> u64>,
}

impl Execute for SegmentInst {
    type ResultValue = (SegReg, u64);
    fn execute(&self) -> Self::ResultValue {
        (self.target, (self.expr)(self.left, self.right))
    }
}

pub struct StatusOpInst {
    target: EFlags,
    value: bool,
}

impl Execute for StatusOpInst {
    type ResultValue = (EFlags, bool);
    fn execute(&self) -> Self::ResultValue {
        (self.target, self.value)
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
    use self::Opcode::*;
    match inst.get_opcode() {
        Cld => decode_eflags_operation(EFlags::DIRECTION_FLAG, false),
        Lea => decode_al_modrm(&inst, &gpr, Box::new(|_, b| b)),
        MovRmSreg => decode_seg_modrm(&inst, &gpr, Box::new(|_, b| b)),
        MovOi => decode_al_rd(&inst, &gpr, Box::new(|_, b| b)),
        Xor => decode_al_modrm(&inst, &gpr, Box::new(|a, b| a ^ b)),
        Hlt => Ok(ExecuteInst::Privileged(PrivilegedInst {})),
    }
}

// helper function to decode mod rm
fn decode_modrm(modrm: ModRm, rf: &RegisterFile, inst: &FetchedInst) -> (Reg32, u64, u64) {
    let (reg, rm) = modrm.get_reg_rm();
    let (left, right) = match modrm.get_mode() {
        0x00 => (rf.read_u64(reg), inst.get_disp()),
        0x11 => (rf.read_u64(reg), rf.read_u64(rm)),
        _ => (0, 0),
    };
    (reg, left, right)
}

fn decode_al_modrm(
    inst: &FetchedInst,
    gpr: &RegisterFile,
    expr: Box<dyn Fn(u64, u64) -> u64>,
) -> Result<ExecuteInst> {
    let (target, left, right) = decode_modrm(inst.get_modrm(), &gpr, inst);
    let inst = ArithLogicInst {
        target: target,
        left: left,
        right: right,
        expr: expr,
    };
    Ok(ExecuteInst::ArithLogic(inst))
}

fn decode_seg_modrm(
    inst: &FetchedInst,
    gpr: &RegisterFile,
    expr: Box<dyn Fn(u64, u64) -> u64>,
) -> Result<ExecuteInst> {
    let (sreg, rm) = inst.get_modrm().get_sreg_rm();
    let inst = SegmentInst {
        target: sreg,
        left: 0, // Never use
        right: gpr.read_u64(rm),
        expr: expr,
    };
    Ok(ExecuteInst::Segment(inst))
}

fn decode_al_rd(
    inst: &FetchedInst,
    gpr: &RegisterFile,
    expr: Box<dyn Fn(u64, u64) -> u64>,
) -> Result<ExecuteInst> {
    let inst = ArithLogicInst {
        target: inst.get_rd(),
        left: gpr.read_u64(inst.get_rd()),
        right: inst.get_imm(),
        expr: expr,
    };
    Ok(ExecuteInst::ArithLogic(inst))
}

fn decode_eflags_operation(target: EFlags, value: bool) -> Result<ExecuteInst> {
    let inst = StatusOpInst {
        target: target,
        value: value,
    };
    Ok(ExecuteInst::StatusOp(inst))
}

fn nop(_left: u64, _right: u64) -> u64 {
    0
}
