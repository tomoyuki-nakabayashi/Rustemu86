//! Decode stage.
use bit_field::BitField;

/// Exceptions occur in decode stage.
#[derive(Debug, Fail, PartialEq)]
pub enum DecodeError {
    #[fail(display = "undefined opcode: 0b{:7b}", opcode)]
    UndefinedInstr { opcode: u32 },
}

/// Decoded instruction.
#[derive(Debug, PartialEq)]
pub struct DecodedInstr(u32);

/// Decode an instruction.
pub fn decode(instr: u32) -> Result<DecodedInstr, DecodeError> {
    let opcode = instr.get_bits(0..8);
    if opcode != 0x73 {
        Err(DecodeError::UndefinedInstr { opcode: opcode })
    } else {
        Ok(DecodedInstr(instr))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decode_undefined_opcode() {
        let instr = 0x0000_0007u32; // FLW won't implement for the present.
        let result = decode(instr);

        assert_eq!(Err(DecodeError::UndefinedInstr { opcode: 0b0000111 }), result);
    }
}