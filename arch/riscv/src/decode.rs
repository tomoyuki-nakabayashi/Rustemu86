//! Decode stage.
use crate::isa::instr_format::ITypeInstrFormat;
use crate::isa::opcode::{Opcode, ALU_OPCODE};
use crate::gpr::Gpr;
use bit_field::BitField;
use num::FromPrimitive;

/// Exceptions occur in decode stage.
#[derive(Debug, Fail, PartialEq)]
pub enum DecodeError {
    #[fail(display = "undefined opcode: 0b{:7b}", opcode)]
    UndefinedInstr { opcode: u32 },
}

/// Decoded instruction.
//#[derive(Debug, PartialEq)]
//pub struct DecodedInstr(pub ITypeInstrFormat);

#[derive(Debug, PartialEq)]
pub enum DecodedInstr {
    System(ITypeInstrFormat),
    Alu(AluInstr),
    Lsu(LsuInstr),
}

/// Decoded format for instructions executed in ALU.
#[derive(Debug, PartialEq)]
pub struct AluInstr {
    pub alu_opcode: ALU_OPCODE,
    pub dest: u32,
    pub operand1: u32,
    pub operand2: u32,
    pub operand3: u32,
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
        Opcode::OpWfi => Ok(DecodedInstr::System(ITypeInstrFormat(instr))),
        Opcode::OpImm => {
            match decode_op_imm(ITypeInstrFormat(instr), &gpr) {
                Ok(decoded) => Ok(DecodedInstr::Alu(decoded)),
                Err(err) => Err(err),
            }
        },
    }
}

// get opcode
fn get_opcode(instr: u32) -> Result<Opcode, DecodeError> {
    let opcode = instr.get_bits(0..7);
    Opcode::from_u32(opcode).ok_or(DecodeError::UndefinedInstr { opcode })
}

// decode OP-IMM
fn decode_op_imm(instr: ITypeInstrFormat, gpr: &Gpr) -> Result<AluInstr, DecodeError> {
    use crate::isa::funct::Rv32iOpImmFunct3;
    let funct3 = Rv32iOpImmFunct3::from_u32(instr.funct3()).unwrap();
    let (rs1, _rs2) = fetch_operand(instr.rs1(), 0, &gpr);
    match funct3 {
        Rv32iOpImmFunct3::ADDI => Ok(AluInstr {
            alu_opcode: ALU_OPCODE::ADD,
            dest: instr.rd(),
            operand1: rs1,
            operand2: instr.imm12() as u32,
            operand3: 0,
        }),
        Rv32iOpImmFunct3::ORI => Ok(AluInstr {
            alu_opcode: ALU_OPCODE::OR,
            dest: instr.rd(),
            operand1: rs1,
            operand2: instr.imm12() as u32,
            operand3: 0,
        }),
        _ => unimplemented!(),
    }
}

// Operand fetch
fn fetch_operand(rs1: u32, rs2: u32, gpr: &Gpr) -> (u32, u32) {
    (gpr.read_u32(rs1), gpr.read_u32(rs2))
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
