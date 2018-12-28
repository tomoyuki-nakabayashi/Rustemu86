//! Execute stage.
//! Returns write back data.

use crate::decode::{DecodedInstr, AluInstr};
use crate::isa::opcode::ALU_OPCODE;

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
pub fn execute(instr: DecodedInstr) -> Result<WriteBackData, ExecuteError> {
    match instr {
        DecodedInstr::System(_) => Ok(WriteBackData::Halt),
        DecodedInstr::Alu(decoded) => execute_op_imm(decoded),
        DecodedInstr::Lsu(_) => unimplemented!(),
    }
}

// Executes OP-IMM instruction.
fn execute_op_imm(instr: AluInstr) -> Result<WriteBackData, ExecuteError> {
    match instr.alu_opcode {
        ALU_OPCODE::ADD => Ok(WriteBackData::Gpr {
            target: instr.dest,
            value: (instr.operand1 as i32 + instr.operand2 as i32) as u32,
        }),
        ALU_OPCODE::OR => Ok(WriteBackData::Gpr {
            target: instr.dest,
            value: (instr.operand1 | instr.operand2),
        }),
        _ => unimplemented!(),
    }
}