//! Decode stage.
use crate::isa::opcode::Opcode;
use crate::isa::instr_format::ITypeInstrFormat;
use bit_field::BitField;
use num::FromPrimitive;

/// Exceptions occur in decode stage.
#[derive(Debug, Fail, PartialEq)]
pub enum DecodeError {
    #[fail(display = "undefined opcode: 0b{:7b}", opcode)]
    UndefinedInstr { opcode: u32 },
}

/// Decoded instruction.
#[derive(Debug, PartialEq)]
pub struct DecodedInstr(pub ITypeInstrFormat);

/// Decode an instruction.
/// There are two sub-stage in the decode.
///   - Decode an instruction according to opcode.
///   - Prepare operand either reading GPR or zero/sign extending the immediate.
pub fn decode(instr: u32) -> Result<DecodedInstr, DecodeError> {
    let opcode = get_opcode(instr)?;
    match opcode {
        Opcode::OpImm => Ok(DecodedInstr(ITypeInstrFormat(instr))),
        Opcode::OpWfi => Ok(DecodedInstr(ITypeInstrFormat(instr))),
    }
}

// get opcode
pub fn get_opcode(instr: u32) -> Result<Opcode, DecodeError> {
    let opcode = instr.get_bits(0..7);
    Opcode::from_u32(opcode).ok_or(DecodeError::UndefinedInstr { opcode })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decode_undefined_opcode() {
        let instr = 0x0000_0007u32; // FLW won't implement for the present.
        let result = decode(instr);

        assert_eq!(
            Err(DecodeError::UndefinedInstr { opcode: 0b0000111 }),
            result
        );
    }
}
