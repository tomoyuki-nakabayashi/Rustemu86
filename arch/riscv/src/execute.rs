//! Execute stage.
//! Returns write back data.

use crate::decode::{AluInstr, BrInstr, CsrInstr, DecodedInstr, LsuInstr};
use crate::isa::opcode::{AluOp, BranchType, LoadStoreType};
use bit_field::BitField;

/// Packet to modify CPU state finally.
pub enum WriteBackData {
    Gpr { target: u32, value: u32 },
    Csr(CsrInstr),
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
pub fn execute(instr: DecodedInstr) -> Result<(WriteBackData, u32), ExecuteError> {
    match instr {
        DecodedInstr::System { npc } => Ok((WriteBackData::Halt, npc)),
        DecodedInstr::Csr(decoded) => forward_system(decoded),
        DecodedInstr::Alu(decoded) => execute_alu(decoded),
        DecodedInstr::Br(decoded) => execute_branch(decoded),
        DecodedInstr::Lsu(decoded) => execute_lsu(decoded),
    }
}

// Forward decoded packet to CSR.
fn forward_system(instr: CsrInstr) -> Result<(WriteBackData, u32), ExecuteError> {
    let next_pc = instr.next_pc;
    match instr.op {
        _ => Ok((WriteBackData::Csr(instr), next_pc)), // dummy next PC
    }
}

// Executes ALU operation.
fn execute_alu(instr: AluInstr) -> Result<(WriteBackData, u32), ExecuteError> {
    let value = alu_op(instr.alu_opcode, instr.src1, instr.src2);
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
fn alu_op(op: AluOp, src1: u32, src2: u32) -> u32 {
    use self::AluOp::*;
    match op {
        ADD => (src1 as i32).wrapping_add(src2 as i32) as u32,
        SUB => (src1 as i32).wrapping_sub(src2 as i32) as u32,
        OR => src1 | src2,
        SLT => {
            if (src1 as i32) < (src2 as i32) {
                1
            } else {
                0
            }
        }
        SLTU => {
            if src1 < src2 {
                1
            } else {
                0
            }
        }
        AND => src1 & src2,
        XOR => src1 ^ src2,
        SLL => src1 << src2.get_bits(0..5),
        SRL => src1 >> src2.get_bits(0..5),
        SRA => ((src1 as i32) >> src2.get_bits(0..5)) as u32,
        LUI => src2 << 12,
        AUIPC => src1 + (src2 << 12),
    }
}

// Execute branch operation.
fn execute_branch(instr: BrInstr) -> Result<(WriteBackData, u32), ExecuteError> {
    match instr.op {
        BranchType::JALR => {
            let link = WriteBackData::Gpr {
                target: instr.dest,
                value: instr.next_pc,
            };
            // base is rs1.
            let next_pc = instr.src1 + instr.offset;
            Ok((link, next_pc))
        }
        BranchType::JAL => {
            let link = WriteBackData::Gpr {
                target: instr.dest,
                value: instr.next_pc,
            };
            let next_pc = instr.base + instr.offset;
            Ok((link, next_pc))
        }
        BranchType::COND_EQ => {
            let next_pc = branch_target(&instr, instr.src1 == instr.src2);
            Ok((WriteBackData::default(), next_pc))
        }
        BranchType::COND_NE => {
            let next_pc = branch_target(&instr, instr.src1 != instr.src2);
            Ok((WriteBackData::default(), next_pc))
        }
        BranchType::COND_LT => {
            let next_pc = branch_target(&instr, (instr.src1 as i32) < (instr.src2 as i32));
            Ok((WriteBackData::default(), next_pc))
        }
        BranchType::COND_LTU => {
            let next_pc = branch_target(&instr, instr.src1 < instr.src2);
            Ok((WriteBackData::default(), next_pc))
        }
        BranchType::COND_GE => {
            let next_pc = branch_target(&instr, (instr.src1 as i32) >= (instr.src2 as i32));
            Ok((WriteBackData::default(), next_pc))
        }
        BranchType::COND_GEU => {
            let next_pc = branch_target(&instr, instr.src1 >= instr.src2);
            Ok((WriteBackData::default(), next_pc))
        }
    }
}

// helper for conditional branch returns next PC.
#[inline(always)]
fn branch_target(instr: &BrInstr, condition: bool) -> u32 {
    if condition {
        instr.base + instr.offset
    } else {
        instr.next_pc
    }
}

// Execute load/store operation
fn execute_lsu(instr: LsuInstr) -> Result<(WriteBackData, u32), ExecuteError> {
    let addr = instr.base + instr.offset;
    Ok((
        WriteBackData::Lsu(LsuOp {
            op: instr.op,
            dest: instr.dest,
            addr,
            value: 0,
        }),
        instr.next_pc,
    ))
}
