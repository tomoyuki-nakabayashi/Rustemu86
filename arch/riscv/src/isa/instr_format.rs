//! Instruction format for rv32ic.

use bitfield::bitfield;

/// Instruction format is either `Base` (32-bit) or `Compressed` (16-bit).
pub enum InstrFormat {
    Base(BaseInstrFormat),
    Compressed(CompressedInstrFormat),
}

/// `Base` (32-bit) format for rv32i.
#[allow(dead_code, non_camel_case_types)]
pub enum BaseInstrFormat {
    R_FORMAT(RTypeInstr),
    I_FORMAT,
    S_FORMAT,
    B_FORMAT,
    U_FORMAT,
    J_FORMAT,
}

/// `Compressed` (16-bit) format for rv32ic.
#[allow(dead_code, non_camel_case_types)]
pub enum CompressedInstrFormat {
    CR_FORMAT,
    CI_FORMAT,
    CSS_FORMAT,
    CIW_FORMAT,
    CL_FORMAT,
    CS_FORMAT,
    CB_FORMAT,
    CJ_FORMAT,
}

/// R type format:
/// funct7 | rs2 | rs1 | funct3 | rd | opcode
/// OP /
bitfield! {
    #[derive(Clone, Copy, Debug)]
    pub struct RTypeInstr(u32);
    u32;
    pub funct7, _: 31, 25;
    pub rs2, _: 24, 20;
    pub rs1, _: 19, 15;
    pub funct3, _: 14, 12;
    pub rd, _: 11, 7;
    pub opcode, _: 6, 0;
}

/// I type format:
/// imm[11:0] | rs1 | funct3 | rd | opcode
/// OP-IMM /
bitfield! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct ITypeInstr(u32);
    i32;
    pub imm12, _: 31, 20;
    u32;
    pub rs1, _: 19, 15;
    pub funct3, _: 14, 12;
    pub rd, _: 11, 7;
    pub opcode, _: 6, 0;
}

/// U type format:
/// imm[31:12] | rd | opcode
/// JAL
bitfield! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct UTypeInstr(u32);
    i32;
    pub imm20, _: 31, 12;
    u32;
    pub rd, _: 11, 7;
    pub opcode, _: 6, 0;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bitfield() {
        let instr = ITypeInstr(0x0010_8093);
        assert_eq!(1, instr.rd());
    }
}
