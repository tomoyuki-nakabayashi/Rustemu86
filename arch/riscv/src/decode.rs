//! Decode stage.
mod operand_fetch;

use crate::gpr::Gpr;
use crate::isa::instr_format::{ITypeInstr, RTypeInstr};
use crate::isa::opcode::{AluOpcode, Opcode};
use self::operand_fetch::OperandFetch;
use bit_field::BitField;
use num::FromPrimitive;

/// Exceptions occur in decode stage.
#[derive(Debug, Fail, PartialEq)]
pub enum DecodeError {
    #[fail(display = "undefined opcode: 0b{:07b}", opcode)]
    UndefinedInstr { opcode: u32 },

    #[fail(display = "undefined funct3: 0b{:03b}", funct3)]
    UndefinedFunct3 { funct3: u32 },
}

/// Decoded instruction.
//#[derive(Debug, PartialEq)]
//pub struct DecodedInstr(pub ITypeInstr);

#[derive(Debug, PartialEq)]
pub enum DecodedInstr {
    System(ITypeInstr),
    Alu(AluInstr),
    Lsu(LsuInstr),
}

/// Decoded format for instructions executed in ALU.
#[derive(Debug, PartialEq)]
pub struct AluInstr {
    pub alu_opcode: AluOpcode,
    pub dest: u32,
    pub operand1: u32,
    pub operand2: u32,
    pub operand3: u32,
}

impl AluInstr {
    fn from<T: OperandFetch>(op: AluOpcode, instr: &T, gpr: &Gpr) -> AluInstr {
        AluInstr {
            alu_opcode: op,
            dest: instr.dest(),
            operand1: instr.operand1(&gpr),
            operand2: instr.operand2(&gpr),
            operand3: 0,
        }
    }
}

/// Decoded format for instructions going to LSU.
/// load/store instructions generate their address in ALU.
/// So that, it also has `AluInstr`.
#[derive(Debug, PartialEq)]
pub struct LsuInstr {
    alu: AluInstr,
    // TODO: Add fields to let LSU know operation.
}

/// Decode an instruction.
/// There are two sub-stage in the decode.
///   - Decode an instruction according to opcode.
///   - Prepare operand either reading GPR or zero/sign extending the immediate.
pub fn decode(instr: u32, gpr: &Gpr) -> Result<DecodedInstr, DecodeError> {
    let opcode = get_opcode(instr)?;
    match opcode {
        Opcode::OpSystem => Ok(DecodedInstr::System(ITypeInstr(instr))),
        Opcode::OpImm => {
            decode_op_imm(ITypeInstr(instr), &gpr).map(|i| DecodedInstr::Alu(i))
        }
        Opcode::Op => {
            decode_op(RTypeInstr(instr), &gpr).map(|i| DecodedInstr::Alu(i))
        }
    }
}

// get opcode
fn get_opcode(instr: u32) -> Result<Opcode, DecodeError> {
    let opcode = instr.get_bits(0..7);
    Opcode::from_u32(opcode).ok_or(DecodeError::UndefinedInstr { opcode })
}

// decode OP-IMM
fn decode_op_imm(instr: ITypeInstr, gpr: &Gpr) -> Result<AluInstr, DecodeError> {
    use crate::isa::funct::Rv32iOpImmFunct3::{self, *};
    let funct3 =
        Rv32iOpImmFunct3::from_u32(instr.funct3()).ok_or(DecodeError::UndefinedFunct3 {
            funct3: instr.funct3(),
        })?;
    match funct3 {
        //ADDI => Ok(AluInstr::from_i_type(AluOpcode::ADD, instr, &gpr)),
        ADDI => Ok(AluInstr::from(AluOpcode::ADD, &instr, &gpr)),
        ORI => Ok(AluInstr::from(AluOpcode::OR, &instr, &gpr)),
        _ => unimplemented!(),
    }
}

// decode OP
fn decode_op(instr: RTypeInstr, gpr: &Gpr) -> Result<AluInstr, DecodeError> {
    use crate::isa::funct::Rv32iOpFunct3::{self, *};
    let funct3 = Rv32iOpFunct3::from_u32(instr.funct3()).ok_or(DecodeError::UndefinedFunct3 {
        funct3: instr.funct3(),
    })?;
    match funct3 {
        ADD => Ok(AluInstr::from(AluOpcode::ADD, &instr, &gpr)),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decode_undefined_opcode() {
        let gpr = Gpr::new();
        let instr = 0x0000_0007u32; // FLW won't implement for the present.
        let result = decode(instr, &gpr);

        assert_eq!(
            Err(DecodeError::UndefinedInstr { opcode: 0b0000111 }),
            result
        );
    }
}
