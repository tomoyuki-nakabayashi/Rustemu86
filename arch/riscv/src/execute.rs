//! Execute stage.
//! Returns write back data.

use crate::decode::{AluInstr, BrInstr, DecodedInstr, LsuInstr};
use crate::isa::opcode::{AluOpcode, BranchType, LoadStoreType};

/// Packet to modify CPU state finally.
pub enum WriteBackData {
    Gpr { target: u32, value: u32 },
    Lsu(LsuOp),
    Halt,
}

impl Default for WriteBackData {
    /// Create dummy data which will be ignored because destination is zero
    /// register.
    fn default() -> WriteBackData {
        WriteBackData::Gpr {
            target: 0,
            value: 0,
        }
    }
}

pub struct LsuOp {
    pub op: LoadStoreType,
    pub dest: u32,
    pub addr: u32,
    pub value: u32,
}

/// Exceptions occur in execute stage.
#[derive(Debug, Fail, PartialEq)]
pub enum ExecuteError {
    #[fail(display = "overflow occurs")]
    Overflow,
}

/// Executes an instruction.
pub fn execute(instr: &DecodedInstr) -> Result<(WriteBackData, u32), ExecuteError> {
    match instr {
        DecodedInstr::System(ref decoded) => Ok((WriteBackData::Halt, decoded.next_pc)),
        DecodedInstr::Alu(ref decoded) => execute_alu(decoded),
        DecodedInstr::Br(ref decoded) => execute_branch(decoded),
        DecodedInstr::Lsu(ref decoded) => execute_lsu(decoded),
    }
}

// Executes ALU operation.
fn execute_alu(instr: &AluInstr) -> Result<(WriteBackData, u32), ExecuteError> {
    let value = alu_op(instr.alu_opcode, instr.operand1, instr.operand2);
    Ok((
        WriteBackData::Gpr {
            target: instr.dest,
            value,
        },
        instr.next_pc,
    ))
}

// Must not be failed.
// Decode stage validated that the instructions is correct.
fn alu_op(op: AluOpcode, rs1: u32, rs2: u32) -> u32 {
    use self::AluOpcode::*;
    match op {
        ADD => (rs1 as i32 + rs2 as i32) as u32,
        OR => rs1 | rs2,
    }
}

// Execute branch operation.
fn execute_branch(instr: &BrInstr) -> Result<(WriteBackData, u32), ExecuteError> {
    match instr.op {
        BranchType::JAL => {
            let link = WriteBackData::Gpr {
                target: instr.dest,
                value: instr.next_pc,
            };
            let next_pc = instr.base + instr.offset;
            Ok((link, next_pc))
        }
        BranchType::COND_EQ => {
            let next_pc = if instr.operand1 == instr.operand2 {
                instr.base + instr.offset
            } else {
                instr.next_pc
            };
            Ok((WriteBackData::default(), next_pc))
        }
    }
}

// Execute load/store operation
fn execute_lsu(instr: &LsuInstr) -> Result<(WriteBackData, u32), ExecuteError> {
    let sub_op = &instr.alu;
    let addr = alu_op(sub_op.alu_opcode, sub_op.operand1, sub_op.operand2);
    Ok((
        WriteBackData::Lsu(LsuOp {
            op: instr.op,
            dest: sub_op.dest,
            addr,
            value: 0,
        }),
        sub_op.next_pc,
    ))
}
