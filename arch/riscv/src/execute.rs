//! Execute stage.
//! Returns write back data.

use crate::decode::DecodedInstr;
use crate::gpr::Gpr;
use crate::isa::{
    opcode::Opcode,
    instr_format::ITypeInstrFormat,
    funct::Rv32iOpImmFunct3,
};
use num::FromPrimitive;

/// Packet to modify CPU state finally.
pub enum WriteBackData {
    Gpr { target: usize, value: u32 },
    Halt,
}

/// Exceptions occur in execute stage.
#[derive(Debug, Fail, PartialEq)]
pub enum ExecuteError {
    #[fail(display = "overflow occurs")]
    Overflow,
}

/// Executes an instruction.
pub fn execute(instr: DecodedInstr, gpr: &Gpr) -> Result<WriteBackData, ExecuteError> {
    let DecodedInstr(instr) = instr;
    let opcode = Opcode::from_u32(instr.opcode()).unwrap();
    match opcode {
        Opcode::OpWfi => Ok(WriteBackData::Halt),
        Opcode::OpImm => execute_op_imm(instr, &gpr),
    }
}

// Operand fetch
fn fetch_operand(rs1: usize, rs2: usize, gpr: &Gpr) -> (u32, u32) {
    (gpr.read_u32(rs1), gpr.read_u32(rs2))
}

// Executes OP-IMM instruction.
fn execute_op_imm(instr: ITypeInstrFormat, gpr: &Gpr) -> Result<WriteBackData, ExecuteError> {
    let funct3 = Rv32iOpImmFunct3::from_u32(instr.funct3()).unwrap();
    let (rs1, _rs2) = fetch_operand(instr.rs1() as usize, 0, &gpr);
    match funct3 {
        Rv32iOpImmFunct3::ADDI => Ok(WriteBackData::Gpr {
            target: instr.rd() as usize,
            value: (rs1 as i32 + instr.imm12()) as u32,
        }),
        Rv32iOpImmFunct3::ORI => Ok(WriteBackData::Gpr {
            target: instr.rd() as usize,
            value: (rs1 | instr.imm12() as u32) as u32,
        }),
        _ => unimplemented!(),
    }
}