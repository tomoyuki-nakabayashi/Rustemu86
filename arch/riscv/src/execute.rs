//! Execute stage.
//! Returns write back data.

use crate::decode::{AluInstr, BrInstr, DecodedInstr};
use crate::isa::opcode::{AluOpcode, BranchType};

/// Packet to modify CPU state finally.
pub enum WriteBackData {
    Gpr { target: u32, value: u32 },
    Halt,
}

impl Default for WriteBackData {
    /// Create dummy data which will be ignored because destination is zero
    /// register.
    fn default() -> WriteBackData {
        WriteBackData::Gpr { target: 0, value: 0 }
    }
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
        DecodedInstr::Lsu(_) => unimplemented!(),
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
