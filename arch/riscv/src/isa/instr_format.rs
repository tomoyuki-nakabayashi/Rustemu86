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
    R_FORMAT(RTypeInstrFormat),
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
/// dst = rs1 op rs2
bitfield! {
    #[derive(Clone, Copy, Debug)]
    pub struct RTypeInstrFormat(u32);
    u32;
    pub funct7, _: 31, 25;
    pub rs2, _: 24, 20;
    pub rs1, _: 19, 15;
    pub funct3, _: 14, 12;
    pub rd, _: 11, 7;
    pub opcode, _: 6, 0;
}

/// I type format:
/// dst = rs1 op imm[11:0]
/// ADDI / SLTI[U] / ANDI / ORI / XORI
bitfield! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct ITypeInstrFormat(u32);
    i32;
    pub imm12, _: 31, 20;
    u32;
    pub rs1, _: 19, 15;
    pub funct3, _: 14, 12;
    pub rd, _: 11, 7;
    pub opcode, _: 6, 0;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bitfield() {
        let instr = ITypeInstrFormat(0x0010_8093);
        assert_eq!(1, instr.rd());
    }
}