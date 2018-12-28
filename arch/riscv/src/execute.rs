//! Execute stage.
//! Returns write back data.

use crate::decode::{AluInstr, DecodedInstr};
use crate::isa::opcode::AluOpcode;

/// Packet to modify CPU state finally.
pub enum WriteBackData {
    Gpr { target: u32, value: u32 },
    Halt,
}

/// Exceptions occur in execute stage.
#[derive(Debug, Fail, PartialEq)]
pub enum ExecuteError {
    #[fail(display = "overflow occurs")]
    Overflow,
}

/// Executes an instruction.
pub fn execute(instr: &DecodedInstr) -> Result<WriteBackData, ExecuteError> {
    match instr {
        DecodedInstr::System(_) => Ok(WriteBackData::Halt),
        DecodedInstr::Alu(ref decoded) => execute_alu(decoded),
        DecodedInstr::Lsu(_) => unimplemented!(),
    }
}

// Executes ALU operation.
fn execute_alu(instr: &AluInstr) -> Result<WriteBackData, ExecuteError> {
    let value = alu_op(instr.alu_opcode, instr.operand1, instr.operand2);
    Ok(WriteBackData::Gpr {
        target: instr.dest,
        value,
    })
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
